mod app_env_management;
mod pb;
mod services;

use std::net::SocketAddr;

use sqlx::PgPool;
use tonic::transport::Server;

use crate::{
    app_env_management::load_config, pb::guild::guild_service_server::GuildServiceServer,
    services::guilds::GuildServiceInternal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: app_env_management::Config = load_config();

    let pool = PgPool::connect(&config.database_url).await?;
    let guild_service = GuildServiceInternal { pool };

    let server_addres: SocketAddr = config
        .guilds_service_url
        .parse()
        .expect("Unable to parse socket adress");
    Server::builder()
        .add_service(GuildServiceServer::new(guild_service))
        .serve(server_addres)
        .await?;

    Ok(())
}
