use crate::{ 
    config::Config,
    db::connection::Database,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: &'static Database,
}

impl AppState {
    pub fn new(config: Config, db: &'static Database) -> Self {
        Self { 
            config, 
            db,
        }
    }
}
