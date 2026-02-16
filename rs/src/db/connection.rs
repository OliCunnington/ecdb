use surrealdb::{
    engine::remote::ws::{Client, Ws}, 
    opt::auth::Root, 
    Surreal,
    Error
};
use std::sync::LazyLock;

use crate::{
    config::Config,
    // error::DatabaseError,
};

pub type Database = Surreal<Client>;

// Global database instance following the documentation pattern
static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn initialize_database(config: &Config) -> Result<&'static Surreal<Client>, Error> {
    // Connect to SurrealDB server via Web Sockets
    DB.connect::<Ws>(&config.database_url).await?;

    tracing::info!("Database WebSocket connection initialized successfully");

    // Sign in as root user using config credentials
    DB.signin(Root {
        username: &config.database_username,
        password: &config.database_password,
    }).await?;

    tracing::info!("Database service_user signed in successfully");
    
    // Use namespace and database from .env config file (create new if they do not exist already)
    DB.use_ns(&config.database_namespace).use_db(&config.database_name).await?;

    Ok(&DB)
}