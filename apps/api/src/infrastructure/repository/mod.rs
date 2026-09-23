use thiserror::Error;
use toasty::{Db, ModelSet};

pub mod user_identity_repository;
pub mod user_repository;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database connection error: {0}")]
    ConnectionError(String),
}

pub async fn build_db_connection() -> Result<Db, DatabaseError> {
    Db::builder()
        .models(models())
        .connect("postgresql://postgres:postgres@localhost:5432/dozens")
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))
}

fn models() -> ModelSet {
    toasty::models!(
        user_repository::UserRecord,
        user_identity_repository::UserIdentityRecord
    )
}
