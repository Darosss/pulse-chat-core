use std::{collections::HashMap, sync::Arc};

use crate::redis_utils::auth::get_blacklist_key;
use crate::{
    accounts::AuthService, app_error::AppError, messages::MessageService,
    ws_gateway::service::WsService,
};
use jsonwebtoken::DecodingKey;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use tokio::sync::{Mutex, RwLock, broadcast};
use tonic::Status;

pub type RoomChannels = Arc<Mutex<HashMap<i32, broadcast::Sender<String>>>>;

#[derive(Clone)]
pub struct AppState {
    pub public_decode_key: Arc<RwLock<Option<DecodingKey>>>,
    pub messages: MessageService,
    pub accounts: AuthService,
    pub ws_state: WsService,
    pub redis: MultiplexedConnection,
    pub redis_client: Client,
    pub rooms: RoomChannels,
}

impl AppState {
    pub async fn get_or_fetch_public_key(&self) -> Result<DecodingKey, AppError> {
        {
            let reader = self.public_decode_key.read().await;
            if let Some(ref key) = *reader {
                return Ok(key.clone());
            }
        }

        let mut writer = self.public_decode_key.write().await;

        if let Some(ref key) = *writer {
            return Ok(key.clone());
        }

        let client = self.accounts.clone();

        match client.get_public_jwt_key().await {
            Ok(res) => {
                let pem_bytes = res.key;
                let key = DecodingKey::from_rsa_pem(pem_bytes.as_bytes()).map_err(|_| {
                    AppError::AccountsService(Status::internal("Invalid PEM received: {}"))
                })?;

                *writer = Some(key.clone());
                Ok(key)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn blacklist_access_token(&self, jti: &str, exp_ttl: i64) -> Result<bool, AppError> {
        let ttl = if exp_ttl > 0 { exp_ttl } else { 1 } as u64;
        let mut redis_client = self.redis.clone();
        if redis_client
            .set_ex::<_, _, ()>(get_blacklist_key(&jti), 1, ttl)
            .await
            .is_err()
        {
            return Err(AppError::Gateway(Status::internal(
                "Redis storage error. Couldn't blacklist existing refresh_token",
            )));
        }
        Ok(true)
    }
}
