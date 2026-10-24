use toasty::{
    Db,
    schema::Model,
    stmt::{List, Query},
};

use crate::infrastructure::{self, repository::DatabaseError};

pub(crate) async fn build_db_connection() -> Result<Db, DatabaseError> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/dozens_test".to_owned());
    infrastructure::repository::build_db_connection(&url).await
}

pub(crate) async fn delete_records<T: Model>(db: &mut Db) {
    Query::<List<T>>::all().delete().exec(db).await.unwrap();
}
