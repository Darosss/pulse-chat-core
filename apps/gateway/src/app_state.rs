use std::{collections::HashMap, sync::Arc};

use crate::{
    accounts::AuthService, app_error::AppError, messages::MessageService,
    ws_gateway::service::WsService,
};
use jsonwebtoken::DecodingKey;
use redis::Client;
use redis::aio::MultiplexedConnection;
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
}
