use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tonic::{Request, metadata::MetadataValue, transport::Channel};

use crate::{
    app_error::AppError,
    messages::handlers::{CreateMessageBody, MessageItemResponse},
    pb::message::{
        HistoryRequest, HistoryResponse, MessageItem, StreamRequest, message::CreateMessageRequest,
        message_service_client::MessageServiceClient,
    },
    utils::add_user_id_to_request,
};

#[derive(Clone)]
pub struct MessageService {
    pub client: MessageServiceClient<Channel>,
}

impl MessageService {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: MessageServiceClient::new(channel),
        }
    }

    pub async fn create_message(
        self,
        channel_id: i32,
        user_id: i32,
        payload: CreateMessageBody,
    ) -> Result<MessageItem, AppError> {
        let mut client = self.client.clone();

        let request = add_user_id_to_request(
            Request::new(CreateMessageRequest {
                channel_id,
                content: payload.content,
            }),
            user_id,
        );

        let result = client.create_message(request).await?;
        Ok(result.into_inner())
    }
    pub async fn get_history(
        self,
        request: HistoryRequest,
        user_id: i32,
    ) -> Result<HistoryResponse, AppError> {
        let mut client = self.client.clone();
        let request = add_user_id_to_request(Request::new(request), user_id);

        let response = client.get_channel_history(request).await?;
        Ok(response.into_inner())
    }

    pub async fn handle_socket(self, socket: WebSocket, user_id: i32, channel_id: i32) {
        let (mut sender, _receiver) = socket.split();
        let request = add_user_id_to_request(Request::new(StreamRequest { channel_id }), user_id);
        let mut client = self.client.clone();
        let response = client.stream_live_messages(request).await.unwrap();
        let mut grpc_stream = response.into_inner();

        while let Ok(Some(grpc_msg)) = grpc_stream.message().await {
            let msg = MessageItemResponse::from(grpc_msg);
            let payload = json! (
                {
                "event": "NEW_MESSAGE",
                "payload": msg
            }
            )
            .to_string();

            if sender.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    }
}
