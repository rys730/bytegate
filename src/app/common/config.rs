use std::env;

pub struct AppConfig {
    pub db_config: DBConfig
}

pub struct DBConfig {
    pub db_host: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
}

pub fn load_config() -> AppConfig {
    let db_config = DBConfig{
        db_host: env::var("DB_HOST").expect("DB_HOST is not set"),
        db_user: env::var("DB_USER").expect("DB_USER is not set"),
        db_pass: env::var("DB_PASS").expect("DB_PASS is not set"),
        db_name: env::var("DB_NAME").expect("DB_NAME is not set"),
    };
    AppConfig { db_config: db_config }
}