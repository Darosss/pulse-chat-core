use crate::{app_error::AppError, app_state::RoomChannels};
use futures_util::StreamExt;
use redis::{Client, aio::PubSub};
use tokio::sync::broadcast;
use tonic::Status;

const BROADCAST_CHANNEL_CAPACITY: usize = 100;
pub async fn get_or_join_room_channel(
    redis_client: Client,
    rooms: RoomChannels,
    channel_id: &i32,
) -> broadcast::Receiver<String> {
    let mut rooms = rooms.lock().await;

    if let Some(sender) = rooms.get(&channel_id) {
        return sender.subscribe();
    }

    let (tx, rx) = broadcast::channel::<String>(BROADCAST_CHANNEL_CAPACITY);
    rooms.insert(*channel_id, tx.clone());

    let redis_client = redis_client.clone();
    let room_id = channel_id.to_string();

    tokio::spawn(async move {
        if let Ok(mut pubsub) = redis_client.get_async_pubsub().await {
            let redis_channel = format!("channel:{room_id}:events");
            if pubsub.subscribe(&redis_channel).await.is_ok() {
                let mut stream = pubsub.on_message();

                while let Some(msg) = stream.next().await {
                    if let Ok(payload) = msg.get_payload::<String>() {
                        if tx.send(payload).is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });

    rx
}

pub fn get_gateway_ws_key(user_id: &i32) -> String {
    format!("gateway:ws:{user_id}")
}

pub async fn get_ws_redis_stream(
    redis_client: redis::Client,
    user_id: &i32,
) -> Result<PubSub, AppError> {
    if let Ok(mut pubsub) = redis_client.get_async_pubsub().await {
        if pubsub.subscribe(get_gateway_ws_key(user_id)).await.is_ok() {
            return Ok(pubsub);
        }
    }
    Err(AppError::Gateway(Status::internal(
        "Couldn't connect to redis - ws channel",
    )))
}
