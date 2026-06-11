use loco_rs::prelude::*;

use crate::middleware::auth::CookieJWT;
use crate::models::users;

pub async fn current_user_id(ctx: &AppContext, auth: &CookieJWT) -> Result<i32> {
    if let Some(user_id) = auth
        .claims
        .claims
        .get("user_id")
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok())
    {
        return Ok(user_id);
    }

    Ok(users::Model::find_by_pid(&ctx.db, &auth.claims.pid)
        .await?
        .id)
}
