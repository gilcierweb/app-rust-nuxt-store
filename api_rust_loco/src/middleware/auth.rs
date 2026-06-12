use crate::cache::blacklist_cache;
use crate::models::ability::Ability;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use loco_rs::{app::AppContext, auth::jwt, errors::Error};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sha2::{Digest, Sha256};

use crate::models::_entities::jwt_blacklist;

#[derive(Debug)]
pub struct CookieJWT {
    pub claims: jwt::UserClaims,
}

#[derive(Clone, Debug)]
pub struct AdminSession {
    pub current_user_id: i32,
    pub ability: Ability,
    pub roles: Vec<String>,
}

/// Token hash pre-validated by `admin_namespace_guard`.
/// When present, `CookieJWT` skips the blacklist DB query.
#[derive(Clone, Debug)]
pub struct ValidatedTokenHash(pub String);

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

impl FromRequestParts<AppContext> for CookieJWT {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar
            .get("auth_token")
            .map(|cookie| cookie.value().to_string())
            .ok_or_else(|| Error::Unauthorized("unauthorized".to_string()))?;

        let jwt_secret = state.config.get_jwt_config()?;

        match jwt::JWT::new(&jwt_secret.secret).validate(&token) {
            Ok(token_data) => {
                let token_hash = hash_token(&token);

                // If admin_namespace_guard already validated this token, it's
                // not blacklisted — skip the DB query entirely.
                let is_blacklisted =
                    if parts.extensions.get::<ValidatedTokenHash>().is_some() {
                        false
                    } else if let Some(cached) = blacklist_cache().get(&token_hash) {
                        cached
                    } else {
                        let now = Utc::now();
                        let result = jwt_blacklist::Entity::find()
                            .filter(jwt_blacklist::Column::TokenHash.eq(&token_hash))
                            .filter(jwt_blacklist::Column::ExpiresAt.gt(now))
                            .one(&state.db)
                            .await
                            .map_err(Error::DB)?
                            .is_some();
                        blacklist_cache().insert(token_hash.clone(), result);
                        result
                    };

                if is_blacklisted {
                    return Err(Error::Unauthorized("token revoked".to_string()));
                }

                Ok(Self {
                    claims: token_data.claims,
                })
            }
            Err(_) => Err(Error::Unauthorized("unauthorized".to_string())),
        }
    }
}

