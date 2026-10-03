use anyhow::Result;
use api::{
    domain::{
        model::{user::User, user_identity::UserIdentity, username::Username},
        repository::{
            user_identity_repository::UserIdentityRepository, user_repository::UserRepository,
        },
    },
    infrastructure::{
        repository::{
            build_db_connection, user_identity_repository::UserIdentityRepositoryImpl,
            user_repository::UserRepositoryImpl,
        },
        service::local_auth,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let db = build_db_connection("postgresql://postgres:postgres@localhost:5432/dozens").await?;
    let user_repository = UserRepositoryImpl::new(db.clone());
    let user_identity_repository = UserIdentityRepositoryImpl::new(db);

    let iss = local_auth::issuer();
    let sub = local_auth::subject();
    let user = match user_repository.get_user_by_sub(&sub).await {
        Ok(user) => user,
        Err(_) => {
            let user = user_repository
                .create_user(User::new(Username::generate().as_str().to_string()))
                .await?;
            user_identity_repository
                .create_user_identity(UserIdentity::new(user.id, iss, sub.clone()))
                .await?;
            user
        }
    };

    let token = local_auth::issue_access_token(&sub, 3600)?;
    println!("user_id: {}", user.id);
    println!("sub: {sub}");
    println!("Authorization: Bearer {token}");
    Ok(())
}
