use toasty::{
    Db,
    schema::Model,
    stmt::{List, Query},
};

use crate::infrastructure::{self, repository::DatabaseError};

pub(crate) async fn build_db_connection() -> Result<Db, DatabaseError> {
    infrastructure::repository::build_db_connection(
        "postgresql://postgres:postgres@localhost:5433/dozens_test",
    )
    .await
}

pub(crate) async fn delete_records<T: Model>(db: &mut Db) {
    Query::<List<T>>::all().delete().exec(db).await.unwrap();
}
