use tonic::{Status, transport::Channel};

use crate::pb::message::{
    HistoryRequest, HistoryResponse, message_service_client::MessageServiceClient,
};

#[derive(Clone)]
pub struct MessageService {
    pub client: MessageServiceClient<Channel>,
}

impl MessageService {
    pub fn new(client: MessageServiceClient<Channel>) -> Self {
        Self { client }
    }
    pub async fn get_history(self, request: HistoryRequest) -> Result<HistoryResponse, Status> {
        let mut client = self.client.clone();
        let history = client.get_channel_history(request).await?;
        Ok(history.into_inner())
    }
}
