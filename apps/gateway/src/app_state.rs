use crate::messages::service::MessageService;

#[derive(Clone)]
pub struct AppState {
    pub messages: MessageService,
    // pub auth_client: AuthServiceClient<Channel>,
    // pub presence_client: PresenceServiceClient<Channel>,
}
