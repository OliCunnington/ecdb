#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub database_namespace: String,
    pub database_name: String,
    pub bind_addr: String,
    // pub migrations_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env.dev.pub variables if the file exists
        if std::path::Path::new(".env.dev.pub").exists() {
            dotenvy::from_filename(".env.dev.pub").ok();
            
            // Get the DATABASE_NAME value after loading the file
            let db_name = std::env::var("DATABASE_NAME")
                .unwrap_or_else(|_| "unknown".to_string());
            tracing::info!(".env.dev.pub variable loaded DATABASE_NAME: {}", db_name);
        }

        // Load .env variables
        dotenvy::dotenv().ok();

        Self {
            bind_addr: std::env::var("BIND_ADDR")
                .unwrap_or_else(|_| "localhost:3000".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "localhost:8000".to_string()),
            database_username: std::env::var("DATABASE_USERNAME")
                .unwrap_or_else(|_| "root".to_string()),
            database_password: std::env::var("DATABASE_PASSWORD")
                .unwrap_or_else(|_| "root".to_string()),
            database_namespace: std::env::var("DATABASE_NAMESPACE")
                .unwrap_or_else(|_| "main".to_string()),
            database_name: std::env::var("DATABASE_NAME")
                .unwrap_or_else(|_| "main".to_string()),
            // migrations_dir: std::env::var("MIGRATIONS_DIR")
            //     .unwrap_or_else(|_| "migrations".to_string()),
        }
    }
}
