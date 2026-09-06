use sqlx::{Pool, Postgres, types::chrono};
use tonic::{Response, Status};
use uuid::Uuid;

use crate::pb::guild::{CreateGuildRequest, GuildResponse, guild_service_server};

pub struct GuildServiceInternal {
    pub pool: Pool<Postgres>,
}
#[tonic::async_trait]
impl guild_service_server::GuildService for GuildServiceInternal {
    #[allow(
        mismatched_lifetime_syntaxes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    async fn create_guild(
        &self,
        request: tonic::Request<CreateGuildRequest>,
    ) -> std::result::Result<tonic::Response<GuildResponse>, tonic::Status> {
        let guild_id = Uuid::new_v4();

        let user_id_header = request
            .metadata()
            .get("x-user-id")
            .ok_or_else(|| Status::unauthenticated("Missing x-user-id metadata header"))?;
        let owner_id: i32 = user_id_header
            .to_str()
            .map_err(|_| Status::invalid_argument("Invalid header character encoding"))?
            .parse()
            .map_err(|_| Status::invalid_argument("x-user-id must be a valid 64-bit integer"))?;

        let req = request.into_inner();
        let guild_name = &req.name;

        let created_at = chrono::Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO guilds (name, owner_id, created_at)
            VALUES ($1, $2, $3)
            "#,
            guild_name,
            owner_id,
            created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            Status::internal("Failed to save guild record")
        })?;

        Ok(Response::new(GuildResponse {
            id: guild_id.to_string(),
            name: req.name,
            owner_id,
            created_at: created_at.to_string(),
        }))
    }
}
