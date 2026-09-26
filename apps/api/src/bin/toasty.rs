use anyhow::Result;
use api::infrastructure::repository::build_db_connection;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let db = build_db_connection("postgresql://postgres:postgres@localhost:5432/dozens").await?;
    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;
    Ok(())
}
