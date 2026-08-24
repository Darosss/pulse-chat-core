use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use redis::{AsyncCommands, aio::MultiplexedConnection};
use serde_json::json;
use tonic::{Request, Streaming, transport::Channel};

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
    presence_expiration: u64,
}

impl MessageService {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: MessageServiceClient::new(channel),
            presence_expiration: 45,
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

    async fn handle_chat_socket_loop(
        &self,
        socket: WebSocket,
        mut redis: MultiplexedConnection,
        mut grpc_stream: Streaming<MessageItem>,
        presence_key: &str,
    ) {
        let (mut ws_sender, mut ws_receiver) = socket.split();

        loop {
            tokio::select! {
                maybe_grpc_msg = grpc_stream.message() => {
                    match maybe_grpc_msg {
                        Ok(Some(grpc_msg)) => {
                            let msg = MessageItemResponse::from(grpc_msg);
                            let payload = json!({
                                "event": "NEW_MESSAGE",
                                "payload": msg
                            }).to_string();

                            if ws_sender.send(Message::Text(payload.into())).await.is_err() {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                maybe_ws_msg = ws_receiver.next() => {
                    match maybe_ws_msg {
                        Some(Ok(Message::Text(text))) => {
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
                                let event = match &val["event"] {
                                    serde_json::Value::String(e) => e as &str,
                                    _=>  return,
                                };
                                match event {
                                    "PING" => {
                                        let _: Result<(), _> = redis.expire(&presence_key, self.presence_expiration as i64).await;
                                        let pong = json!({ "event": "PONG" }).to_string();
                                        if ws_sender.send(Message::Text(pong.into())).await.is_err() {
                                            break;
                                        }
                                    }
                                    _ => {}
                                }

                            }
                        }
                        Some(Ok(Message::Ping(_))) => {
                            let _: Result<(), _> = redis.expire(&presence_key, self.presence_expiration as i64).await;
                        }
                        Some(Ok(Message::Close(_))) | None => break,
                        _ => {}
                    }
                }
            }
        }
    }
    pub async fn handle_socket(
        self,
        socket: WebSocket,
        mut redis: MultiplexedConnection,
        user_id: i32,
        channel_id: i32,
    ) {
        let request = add_user_id_to_request(Request::new(StreamRequest { channel_id }), user_id);
        let mut client = self.client.clone();
        let Ok(grpc_stream) = client.stream_live_messages(request).await else {
            return;
        };

        let presence_key = format!("presence:user:{user_id}");
        let _: Result<(), _> = redis
            .set_ex(&presence_key, "online", self.presence_expiration)
            .await;

        Self::handle_chat_socket_loop(
            &self,
            socket,
            redis,
            grpc_stream.into_inner(),
            &presence_key,
        )
        .await;
    }
}
