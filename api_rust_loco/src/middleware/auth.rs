use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use loco_rs::{
    app::AppContext,
    auth::jwt,
    errors::Error,
};

#[derive(Debug)]
pub struct CookieJWT {
    pub claims: jwt::UserClaims,
}

impl FromRequestParts<AppContext> for CookieJWT {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppContext) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar
            .get("auth_token")
            .map(|cookie| cookie.value().to_string())
            .ok_or_else(|| Error::Unauthorized("unauthorized".to_string()))?;

        let jwt_secret = state.config.get_jwt_config()?;

        match jwt::JWT::new(&jwt_secret.secret).validate(&token) {
            Ok(token_data) => Ok(Self { claims: token_data.claims }),
            Err(_) => Err(Error::Unauthorized("unauthorized".to_string())),
        }
    }
}
