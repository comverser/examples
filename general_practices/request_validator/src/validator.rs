use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPayload<T>(pub T);

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationErrors(#[from] ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationErrors(error) => {
                let message = format!("Input Validator Error: [{error}]");
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            ServerError::AxumJsonRejection(error) => {
                let message = format!("Axum JSON Rejection: [{error}]");
                (StatusCode::BAD_REQUEST, message).into_response()
            }
        }
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedPayload<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|error| ServerError::AxumJsonRejection(error))?;

        if let Err(validation_errors) = value.validate() {
            return Err(ServerError::ValidationErrors(validation_errors));
        }

        Ok(ValidatedPayload(value))
    }
}
