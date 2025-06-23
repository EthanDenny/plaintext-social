use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    cli::run_cli(migration::Migrator).await;
}
