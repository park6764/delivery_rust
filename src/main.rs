use deadpool_diesel::postgres::Pool;
use dotenvy::dotenv;

pub mod models;
pub mod schema;
pub mod routes;

fn main() {
    dotenv().expect("reading .env file failed");

    let manager = deadpool_diesel::postgres::Manager::new(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"), deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().expect("DB Pool build failed");
}