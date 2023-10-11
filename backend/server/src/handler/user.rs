use crate::error::ApiResult;
use crate::extractor::DbConnection;
use crate::handler::PublicApiRequest;
use crate::AppState;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use chrono::Duration;
use rustter_endpoint::{CreateUser, CreateUserOk, Login, LoginOk};
use rustter_query::session::Session;
use tracing::{info, span};

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = rustter_crypto::hash_password(self.password)?;
        let user_id = rustter_query::user::new(&mut conn, password_hash, &self.username)?;

        info!(username = self.username.as_ref(), "new user created");

        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                user_id,
                username: self.username,
            }),
        ))
    }
}

#[async_trait]
impl PublicApiRequest for Login {
    type Response = (StatusCode, Json<LoginOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let _span = span!(
            tracing::Level::INFO, "logging in",
        user = %self.username.as_ref())
        .entered();

        let password_hash = rustter_query::user::get_password_hash(&mut conn, &self.username)?;
        let password_hash = rustter_crypto::password::deserialize_hash(&password_hash)?;
        rustter_crypto::verify_password(self.password, &password_hash)?;

        let user = rustter_query::user::find(&mut conn, &self.username)?;

        // new session
        let (session, signature) = {
            let fingerprint = serde_json::json!({});
            // TODO extract in Session::new()?
            let duration = Duration::weeks(3);
            let session = Session::new(user.id, fingerprint.into(), duration);

            let mut rng = state.rng.clone();
            let signature = state
                .signing_keys
                .sign(&mut rng, session.id.as_uuid().as_bytes());
            let signature = rustter_crypto::encode_base64(signature);
            (session, signature)
        };

        Ok((
            StatusCode::OK,
            Json(LoginOk {
                session_signature: signature,
                session_id: session.id,
                session_expires: session.expires_at,

                display_name: user.display_name,
                email: user.email,
                profile_image: None,
                user_id: user.id,
            }),
        ))
    }
}
