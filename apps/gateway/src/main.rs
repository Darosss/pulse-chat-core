mod app_error;
mod app_state;
mod pb;

use axum::{Router, routing::get};
extern crate dotenv;
use app_state::AppState;
use dotenv::from_filename;

use crate::{messages::MessageService, pb::message::message_service_client::MessageServiceClient};
mod messages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    from_filename("../../.env").ok();
    let message_service_client = MessageServiceClient::connect("https://localhost:5161").await?;
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
