use dotenvy::dotenv;
use sqlx::{Pool, Postgres};


pub async fn migrate(db: Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let migrate = sqlx::migrate!("./migrations").run(&db).await;
    if migrate.is_err(){
        println!("failed migrating db. please check your db connection.");
        panic!();
    }
    println!("Migrations applied successfully!");
    Ok(())
}