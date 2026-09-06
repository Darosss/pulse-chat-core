use tonic::{Request, transport::Channel};

use crate::{
    app_error::AppError,
    guilds::handlers::CreateGuildBody,
    pb::guild::{CreateGuildRequest, GuildResponse, guild_service_client::GuildServiceClient},
    utils::add_user_id_to_request,
};

#[derive(Clone)]
pub struct GuildsService {
    pub client: GuildServiceClient<Channel>,
}

impl GuildsService {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: GuildServiceClient::new(channel),
        }
    }

    pub async fn create_guild(
        self,
        user_id: i32,
        payload: CreateGuildBody,
    ) -> Result<GuildResponse, AppError> {
        let mut client = self.client.clone();

        let request = add_user_id_to_request(
            Request::new(CreateGuildRequest { name: payload.name }),
            &user_id,
        );

        let result = client.create_guild(request).await?;
        Ok(result.into_inner())
    }
}
