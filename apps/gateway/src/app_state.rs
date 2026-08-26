use std::{collections::HashMap, sync::Arc};

use crate::{accounts::AuthService, messages::MessageService, ws_gateway::service::WsService};
use redis::Client;
use redis::aio::MultiplexedConnection;
use tokio::sync::{Mutex, broadcast};

pub type RoomChannels = Arc<Mutex<HashMap<i32, broadcast::Sender<String>>>>;

#[derive(Clone)]
pub struct AppState {
    pub messages: MessageService,
    pub accounts: AuthService,
    pub ws_state: WsService,
    pub redis: MultiplexedConnection,
    pub redis_client: Client,
    pub rooms: RoomChannels,
}
