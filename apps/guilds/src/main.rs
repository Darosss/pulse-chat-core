mod app_env_management;
mod pb;
mod services;

use std::net::SocketAddr;

use sqlx::{PgPool, Postgres, migrate::MigrateDatabase};
use tonic::transport::Server;

use crate::{
    app_env_management::load_config, pb::guild::guild_service_server::GuildServiceServer,
    services::guilds::GuildServiceInternal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: app_env_management::Config = load_config();
    if !Postgres::database_exists(&config.database_url)
        .await
        .unwrap_or(false)
    {
        println!("Database guilds_db missing. Creating...");
        Postgres::create_database(&config.database_url).await?;
        println!("Database guilds_db created successfully!");
    }
    let pool = PgPool::connect(&config.database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
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
