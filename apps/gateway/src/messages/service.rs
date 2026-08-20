use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tonic::{Status, transport::Channel};

use crate::{
    app_error::AppError,
    messages::handlers::{CreateMessageBody, MessageItemResponse},
    pb::message::{
        HistoryRequest, HistoryResponse, StreamRequest, message::CreateMessageRequest,
        message_service_client::MessageServiceClient,
    },
};

#[derive(Clone)]
pub struct MessageService {
    pub client: MessageServiceClient<Channel>,
}

impl MessageService {
    pub fn new(client: MessageServiceClient<Channel>) -> Self {
        Self { client }
    }

    pub async fn create_message(
        self,
        channel_id: i32,
        payload: CreateMessageBody,
    ) -> Result<bool, AppError> {
        let mut client = self.client.clone();
        client.create_message(CreateMessageRequest {
            channel_id,
            user_id: payload.user_id,
            content: payload.content,
        });
        Ok(true)
    }
    pub async fn get_history(self, request: HistoryRequest) -> Result<HistoryResponse, Status> {
        let mut client = self.client.clone();
        let history = client.get_channel_history(request).await?;
        Ok(history.into_inner())
    }

    pub async fn handle_socket(self, socket: WebSocket, channel_id: i32) {
        let (mut sender, _receiver) = socket.split();

        let mut client = self.client.clone();
        let response = client
            .stream_live_messages(StreamRequest { channel_id })
            .await
            .unwrap();
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
