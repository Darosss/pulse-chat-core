use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use redis::{AsyncCommands, aio::MultiplexedConnection};
use serde_json::{Value, json};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
use tonic::Request;

use crate::{
    app_state::RoomChannels,
    messages::{MessageItemResponse, MessageService},
    pb::message::StreamRequest,
    redis_utils::{
        presence::{PresenceStatus, get_presence_key, set_user_status},
        pubsub::get_or_join_room_channel,
    },
    utils::add_user_id_to_request,
};

type ActiveRooms = HashMap<i32, oneshot::Sender<()>>;

#[derive(Clone)]
pub struct WsService {
    presence_expiration: i64,
}

impl WsService {
    pub fn new() -> Self {
        Self {
            presence_expiration: 45,
        }
    }

    async fn on_ping(
        &self,
        redis: &mut MultiplexedConnection,
        client_tx: &mpsc::Sender<String>,
        presence_key: &str,
    ) -> Result<(), mpsc::error::SendError<String>> {
        let _: Result<(), _> = redis.expire(&presence_key, self.presence_expiration).await;
        let pong = json!({ "event": "PONG" }).to_string();
        client_tx.send(pong).await
    }

    async fn on_join_room(
        &self,
        redis_client: redis::Client,
        redis: &mut MultiplexedConnection,
        rooms: RoomChannels,
        val: Value,
        active_rooms: &mut ActiveRooms,
        user_id: &i32,
        message_service: &mut MessageService,
        client_tx: mpsc::Sender<String>,
    ) {
        let channel_id = val["payload"]["room_id"].as_i64().unwrap_or(0) as i32;
        if !active_rooms.contains_key(&channel_id) {
            let mut room_rx = get_or_join_room_channel(redis_client, rooms, &channel_id).await;

            let request =
                add_user_id_to_request(Request::new(StreamRequest { channel_id }), &user_id);

            if let Ok(response) = message_service.client.stream_live_messages(request).await {
                let mut grpc_stream = response.into_inner();
                let _ = set_user_status(
                    redis,
                    &user_id,
                    PresenceStatus::Online,
                    Option::Some(&channel_id),
                )
                .await;

                let (stop_tx, mut stop_rx) = oneshot::channel::<()>();

                tokio::spawn(async move {
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

                            if client_tx.send(payload).await.is_err() {
                                break;
                            }
                        }
                                    Ok(None) | Err(_) => break,
                                }
                            }

                            Ok(redis_event_json) = room_rx.recv() => {
                                if client_tx.send(redis_event_json).await.is_err() {
                                    break;
                                }
                            }

                            _ = &mut stop_rx => {
                                break;
                            }
                        }
                    }
                });

                active_rooms.insert(channel_id, stop_tx);
            }
        }
    }
    async fn on_leave_room(&self, val: Value, active_rooms: &mut ActiveRooms) {
        let room_id = val["payload"]["room_id"].as_i64().unwrap_or(0);

        if let Some(stop_tx) = active_rooms.remove(&(room_id as i32)) {
            let _ = stop_tx.send(());
        }
    }
    pub async fn handle_socket(
        self,
        socket: WebSocket,
        mut redis: MultiplexedConnection,
        redis_client: redis::Client,
        rooms: RoomChannels,
        message_service: MessageService,
        user_id: i32,
    ) {
        let presence_key = get_presence_key(&user_id);
        let (mut ws_sender, mut ws_receiver) = socket.split();

        let mut active_rooms: ActiveRooms = HashMap::new();
        let (client_tx, mut client_rx) = mpsc::channel::<String>(100);
        let _ = set_user_status(&mut redis, &user_id, PresenceStatus::Online, Option::None).await;
        loop {
            tokio::select! {
            maybe_outgoing = client_rx.recv() => {
                match maybe_outgoing {
                    Some(msg) => {
                        if ws_sender.send(Message::Text(msg.into())).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
                },
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
                                        if self.on_ping(&mut redis, &client_tx, &presence_key).await.is_err() {
                                           break;
                                        }
                                    }
                                    "JOIN_ROOM" => {
                                            self.on_join_room(redis_client.clone(), &mut redis, rooms.clone(), val, &mut active_rooms, &user_id, &mut message_service.clone(), client_tx.clone()).await
                                        },

                                    "LEAVE_ROOM" => {
                                        self.on_leave_room(val, &mut active_rooms).await
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
            };
        }
    }
}
