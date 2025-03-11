use refinery::embed_migrations;
use postgres::{Client, NoTls};
use dotenvy::dotenv;
use std::env;

embed_migrations!("./src/migrations");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_host = env::var("DB_HOST").expect("DB_HOST is empty");
    let db_user = env::var("DB_USER").expect("DB_USER is empty");
    let db_pass = env::var("DB_PASS").expect("DB_PASS is empty");
    let db_name = env::var("DB_NAME").expect("DB_NAME is empty");

    let database_url = format!(
        "postgres://{}:{}@{}/{}", 
        db_user, 
        db_pass, 
        db_host, 
        db_name);
    
    let mut client = Client::connect(&database_url, NoTls)?;
    migrations::runner().run(&mut client)?;

    println!("Migrations applied successfully!");
    Ok(())
}