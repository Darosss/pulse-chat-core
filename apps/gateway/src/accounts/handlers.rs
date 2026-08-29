use axum_auth::AuthBearer;

use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use cookie::{
    Cookie, CookieBuilder, SameSite,
    time::{Duration, SignedDuration},
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::{
    app_error::AppError,
    app_state::AppState,
    pb::auth::{AuthResponse, LogoutResponse},
    redis_utils::pubsub::get_gateway_ws_key,
    utils::get_token_data,
};

#[derive(Serialize, Deserialize)]
pub struct AuthResponseSerialized {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}
impl From<AuthResponse> for AuthResponseSerialized {
    fn from(value: AuthResponse) -> Self {
        Self {
            token: value.token,
            user_id: value.user_id,
            username: value.username,
        }
    }
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LogoutBody {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogoutResponseSerialized {
    pub success: bool,
}

impl From<LogoutResponse> for LogoutResponseSerialized {
    fn from(value: LogoutResponse) -> Self {
        Self {
            success: value.success,
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterBody {
    pub email: String,
    pub password: String,
    pub username: String,
}

fn build_refresh_token_cookie(
    refresh_token: String,
    max_age: SignedDuration,
) -> CookieBuilder<'static> {
    Cookie::build(("refresh_token", refresh_token))
        .path("/auth")
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(max_age)
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginBody>,
) -> Result<(CookieJar, Json<AuthResponseSerialized>), AppError> {
    let result = state
        .accounts
        .login(LoginBody {
            email: payload.email,
            password: payload.password,
        })
        .await?;

    let cookie_ttl = Duration::seconds(result.refresh_token_ttl as i64);

    let ref_token = result.refresh_token.clone();
    let cookie = build_refresh_token_cookie(ref_token, cookie_ttl);
    let updated_jar = jar.add(cookie);
    Ok((updated_jar, Json(result.into())))
}

pub async fn logout(
    State(state): State<AppState>,
    AuthBearer(token): AuthBearer,
    jar: CookieJar,
) -> Result<(CookieJar, Json<LogoutResponseSerialized>), AppError> {
    if let Some(cookie) = jar.get("refresh_token") {
        let refresh_token = cookie.value().to_string();
        let accounts = state.accounts.clone();
        let _ = accounts.logout(LogoutBody { refresh_token }).await;
    }

    let user_data = get_token_data(&state, &token).await?;
    let exp_ttl = std::cmp::max(user_data.exp - chrono::Utc::now().timestamp(), 1);
    state
        .blacklist_access_token(&user_data.jti, exp_ttl)
        .await?;

    let mut redis = state.redis.clone();
    let logout_event = serde_json::json!({
        "event": "LOGOUT",
        "user_id": user_data.user_id,
        "jti": user_data.jti
    })
    .to_string();

    let _: () = redis
        .publish(get_gateway_ws_key(&user_data.user_id), logout_event)
        .await
        .unwrap_or(());

    let remove_cookie = axum_extra::extract::cookie::Cookie::build(("refresh_token", ""))
        .path("/auth")
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .max_age(cookie::time::Duration::ZERO);

    let updated_jar = jar.add(remove_cookie);

    Ok((
        updated_jar,
        Json(LogoutResponseSerialized { success: true }),
    ))
}
pub async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<RegisterBody>,
) -> Result<(CookieJar, Json<AuthResponseSerialized>), AppError> {
    let result = state
        .accounts
        .register(RegisterBody {
            email: payload.email,
            password: payload.password,
            username: payload.username,
        })
        .await?;
    let cookie_ttl = Duration::seconds(result.refresh_token_ttl as i64);

    let ref_token = result.refresh_token.clone();
    let cookie = build_refresh_token_cookie(ref_token, cookie_ttl);
    let updated_jar = jar.add(cookie);
    Ok((updated_jar, Json(result.into())))
}
