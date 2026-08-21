use crate::{accounts::AuthService, messages::MessageService};

#[derive(Clone)]
pub struct AppState {
    pub messages: MessageService,
    pub accounts: AuthService,
    // pub presence_client: PresenceServiceClient<Channel>,
}
