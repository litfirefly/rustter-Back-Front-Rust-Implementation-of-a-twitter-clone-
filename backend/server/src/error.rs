use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;

pub struct ApiError {
    pub code: Option<StatusCode>,
    pub err: color_eyre::Report,
}

pub type ApiResult<T> = Result<T, ApiError>;
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Login failed")]
    Login((StatusCode, String)),
    #[error("Registration failed")]
    Registration((StatusCode, String)),
}

impl ServerError {
    pub fn missing_login() -> Self {
        Self::Login((StatusCode::NOT_FOUND, "User not found".to_string()))
    }

    pub fn wrong_password() -> Self {
        Self::Login((StatusCode::UNAUTHORIZED, "Wrong credentials".to_string()))
    }

    pub fn account_exists() -> Self {
        Self::Registration((StatusCode::CONFLICT, "User already exists".to_string()))
    }
}

pub fn err_response<T: Into<String>>(code: StatusCode, msg: T) -> Response {
    (
        code,
        Json(rustter_endpoint::RequestFailed { msg: msg.into() }),
    )
        .into_response()
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if let Some(code) = self.code {
            return err_response(code, format!("{}", self.err));
        }

        if let Some(server_err) = self.err.downcast_ref::<ServerError>() {
            return match server_err {
                ServerError::Login((code, msg)) => err_response(*code, msg),
                ServerError::Registration((code, msg)) => err_response(*code, msg),
            };
        }

        err_response(StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self.err))
    }
}

impl<E> From<E> for ApiError
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self {
            code: None,
            err: err.into(),
        }
    }
}
