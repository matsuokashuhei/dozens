use thiserror::Error;
use toasty::{Db, ModelSet};

pub mod user_identity_repository;
pub mod user_relationship_repository;
pub mod user_repository;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database connection error: {0}")]
    ConnectionError(String),
}

pub async fn build_db_connection(url: &str) -> Result<Db, DatabaseError> {
    Db::builder()
        .models(models())
        .connect(url)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))
}

fn models() -> ModelSet {
    toasty::models!(
        user_repository::UserRecord,
        user_identity_repository::UserIdentityRecord,
        user_relationship_repository::UserRelationshipRecord
    )
}

pub(crate) fn map_db_error(
    error: toasty::Error,
) -> crate::domain::repository::errors::RepositoryError {
    use crate::domain::repository::errors::RepositoryError;

    let mut source = std::error::Error::source(&error);
    while let Some(err) = source {
        if let Some(e) = err.downcast_ref::<tokio_postgres::Error>() {
            if e.code() == Some(&tokio_postgres::error::SqlState::UNIQUE_VIOLATION) {
                return RepositoryError::UniqueViolation;
            }
            break;
        }
        source = err.source();
    }
    RepositoryError::DatabaseError(error.to_string())
}
