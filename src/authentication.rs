use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid Credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Validate Credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let (user_id, expected_hash) = get_stored_credentials(&credentials.username, pool)
        .await?
        .ok_or_else(|| AuthError::InvalidCredentials(anyhow::anyhow!("Unknown Username")))?;

    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| verify_password_hash(expected_hash, credentials.password))
    })
    .await
    .context("Failed to spawn blocking thread")?
    .context("Invalid Password")
    .map_err(AuthError::InvalidCredentials)?;

    Ok(user_id)
}

#[tracing::instrument(name = "Verify Password Hash", skip(expected_hash, password_candidate))]
fn verify_password_hash(
    expected_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_hash =
        PasswordHash::new(expected_hash.expose_secret()).context("Failed to parse hash string")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_hash,
        )
        .context("Invalid Password")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
    SELECT user_id, password_hash
    FROM users
    WHERE username = $1
    "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to retrieve stored credentials.")?
    .map(|row| (row.user_id, Secret::new(row.password_hash)));

    Ok(row)
}
