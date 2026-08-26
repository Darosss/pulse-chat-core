use tonic::{Request, transport::Channel};

use crate::{
    app_error::AppError,
    messages::handlers::CreateMessageBody,
    pb::message::{
        HistoryRequest, HistoryResponse, MessageItem, message::CreateMessageRequest,
        message_service_client::MessageServiceClient,
    },
    utils::add_user_id_to_request,
};

#[derive(Clone)]
pub struct MessageService {
    pub client: MessageServiceClient<Channel>,
}

impl MessageService {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: MessageServiceClient::new(channel),
        }
    }

    pub async fn create_message(
        self,
        channel_id: i32,
        user_id: i32,
        payload: CreateMessageBody,
    ) -> Result<MessageItem, AppError> {
        let mut client = self.client.clone();

        let request = add_user_id_to_request(
            Request::new(CreateMessageRequest {
                channel_id,
                content: payload.content,
            }),
            &user_id,
        );

        let result = client.create_message(request).await?;
        Ok(result.into_inner())
    }
    pub async fn get_history(
        self,
        request: HistoryRequest,
        user_id: i32,
    ) -> Result<HistoryResponse, AppError> {
        let mut client = self.client.clone();
        let request = add_user_id_to_request(Request::new(request), &user_id);

        let response = client.get_channel_history(request).await?;
        Ok(response.into_inner())
    }
}
