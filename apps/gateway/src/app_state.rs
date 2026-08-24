use crate::{accounts::AuthService, messages::MessageService};
use redis::aio::MultiplexedConnection;

#[derive(Clone)]
pub struct AppState {
    pub messages: MessageService,
    pub accounts: AuthService,
    pub redis: MultiplexedConnection, // pub presence_client: PresenceServiceClient<Channel>,
}
