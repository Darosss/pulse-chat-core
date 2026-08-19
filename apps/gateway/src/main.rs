mod app_env_management;
mod app_error;
mod app_state;
mod pb;

use app_state::AppState;
use axum::{Router, routing::get};

use crate::{
    app_env_management::load_config, messages::MessageService,
    pb::message::message_service_client::MessageServiceClient,
};
mod messages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();

    // let message_service_url = match dotenv::var(message_service_env_key) {
    //     Ok(value) => value,
    //     //NOTE: TODO: keep it panic right now, change into error when separate fn
    //     Err(_) => panic!(
    //         "Make sure you set the {} environment variable",
    //         message_service_env_key
    //     ),
    // };

    let message_service_client = MessageServiceClient::connect(config.message_service_url).await?;
    let messages_service = MessageService::new(message_service_client);
    let state = AppState {
        messages: messages_service,
        // auth_client: todo!(),
        // presence_client: todo!(),
    };

    let app = Router::new()
        .route("/", get(get_home))
        .merge(messages::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_home() -> String {
    "Hello, on Pulse Chat Core".to_string()
}
