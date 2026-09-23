use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::presentation::errors::PresentationError;

pub struct JsonValidator<T>(pub T);

impl<S, T> FromRequest<S> for JsonValidator<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = PresentationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|error| PresentationError::BadRequest(error.to_string()))?;
        if let Err(error) = value.validate() {
            return Err(PresentationError::BadRequest(error.to_string()));
        }
        Ok(Self(value))
    }
}
