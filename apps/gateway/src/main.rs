mod app_env_management;
mod app_error;
mod app_state;
mod pb;
mod redis_utils;
mod utils;
mod ws_gateway;

use std::{collections::HashMap, str::FromStr, sync::Arc};

use app_state::AppState;
use axum::{Router, routing::get};
use tokio::sync::{Mutex, RwLock};
use tonic::transport::Endpoint;

use crate::{
    accounts::AuthService, app_env_management::load_config, messages::MessageService,
    ws_gateway::service::WsService,
};
mod accounts;
mod messages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: app_env_management::Config = load_config();
    let message_service_channel = Endpoint::from_str(&config.message_service_url)
        .expect("Make sure accounts_service_url is provided")
        .connect_lazy();
    let messages_service = MessageService::new(message_service_channel);
    let accounts_service_channel = Endpoint::from_str(&config.accounts_service_url)
        .expect("Make sure accounts_service_url is provided")
        .connect_lazy();
    let accounts_service = AuthService::new(accounts_service_channel);

    let client = redis::Client::open(config.redis_url)?;

    let redis_connection = client.get_multiplexed_async_connection().await?;

    let state = AppState {
        public_decode_key: Arc::new(RwLock::new(Option::None)),
        messages: messages_service,
        accounts: accounts_service,
        redis: redis_connection,
        ws_state: WsService::new(),
        rooms: Arc::new(Mutex::new(HashMap::new())),
        redis_client: client,
    };
    let _ = state.get_or_fetch_public_key().await;

    let app = Router::new()
        .route("/", get(get_home))
        .merge(ws_gateway::router())
        .merge(accounts::router())
        .merge(messages::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_home() -> String {
    "Hello, on Pulse Chat Core".to_string()
}
