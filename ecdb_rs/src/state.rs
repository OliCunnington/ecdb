use crate::{ 
    config::Config,
    db::connection::Database,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: &'static Database,
    pub session: &'static SessionStore
}

impl AppState {
    pub fn new(config: Config, db: &'static Database, session: &'static SessionStore) -> Self {
        Self { 
            config, 
            db,
            session
        }
    }
}
