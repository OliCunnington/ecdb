use crate::{ 
    config::Config,
    db::connection::Database,
};
use axum_session::SessionStore;
use axum_session_surreal::SessionSurrealPool;
use surrealdb::engine::remote::ws::Client;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: &'static Database,
    pub session: &'static SessionStore<SessionSurrealPool<Client>>
}

impl AppState {
    pub fn new(config: Config, db: &'static Database, session: &'static SessionStore<SessionSurrealPool<Client>>) -> Self {
        Self { 
            config, 
            db,
            session
        }
    }
}
