use async_trait::async_trait;
use axum::{
    extract::{Request, State},
    http::Method,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    auth::jwt,
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};

use crate::middleware::{
    api_key::api_key_guard, auth::AdminSession, csrf::csrf_guard, rate_limit::rate_limit_guard,
};
use crate::seeds;
use crate::{models::ability::Ability, models::users::Model as UserModel};

use migration::Migrator;
use serde_json::json;
use std::path::Path;

#[allow(unused_imports)]
use crate::{controllers, models::_entities::users, tasks, workers::downloader::DownloadWorker};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::payment_setup_session::routes())
            .add_route(controllers::payment_webhook::routes())
            .add_route(controllers::dashboard::routes())
            .add_route(controllers::dashboard::admin_routes())
            .add_route(controllers::banner::routes())
            .add_route(controllers::banner::admin_routes())
            .add_route(controllers::user::routes())
            .add_route(controllers::user::admin_routes())
            .add_route(controllers::variant::routes())
            .add_route(controllers::variant::admin_routes())
            .add_route(controllers::shipping::routes())
            .add_route(controllers::shipping::admin_routes())
            .add_route(controllers::payment::routes())
            .add_route(controllers::payment::admin_routes())
            .add_route(controllers::coupon::routes())
            .add_route(controllers::coupon::admin_routes())
            .add_route(controllers::address::routes())
            .add_route(controllers::address::admin_routes())
            .add_route(controllers::wishlist::routes())
            .add_route(controllers::wishlist::account_routes())
            .add_route(controllers::review::routes())
            .add_route(controllers::review::admin_routes())
            .add_route(controllers::order::routes())
            .add_route(controllers::order::admin_routes())
            .add_route(controllers::order::account_routes())
            .add_route(controllers::cart::routes())
            .add_route(controllers::product::routes())
            .add_route(controllers::product::admin_routes())
            .add_route(controllers::category::routes())
            .add_route(controllers::category::admin_routes())
            .add_route(controllers::profile::routes())
            .add_route(controllers::profile::admin_routes())
            .add_route(controllers::post::routes())
            .add_route(controllers::post::admin_routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::shipment::routes())
            .add_route(controllers::shipment::admin_routes())
            .add_route(controllers::admin_inventory::routes())
            .add_route(controllers::admin_settings::routes())
            .add_route(controllers::admin_payment_gateways::routes())
            .add_route(controllers::admin_payment_methods::routes())
            .add_route(controllers::admin_payments::routes())
            .add_route(controllers::admin_payments::gateway_event_routes())
            .add_route(controllers::admin_payments::gateway_log_routes())
    }

    async fn after_routes(router: axum::Router, ctx: &AppContext) -> Result<axum::Router> {
        use utoipa::OpenApi;
        use utoipa_swagger_ui::SwaggerUi;

        let swagger = SwaggerUi::new("/api/docs")
            .url("/api-docs/openapi.json", crate::openapi::ApiDoc::openapi());

        Ok(router
            .merge(swagger)
            .layer(middleware::from_fn_with_state(
                ctx.clone(),
                admin_namespace_guard,
            ))
            .layer(middleware::from_fn_with_state(ctx.clone(), csrf_guard))
            .layer(middleware::from_fn_with_state(
                ctx.clone(),
                rate_limit_guard,
            ))
            .layer(middleware::from_fn(api_key_guard)))
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(crate::workers::payment_webhook_retry::Worker::build(ctx)).await?;
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        // YAML-based seed for users (static data)
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;

        // Dynamic seed by code (with fakeit) - comprehensive seed for all entities
        seeds::seed_all(&ctx.db).await?;

        Ok(())
    }
}

async fn admin_namespace_guard(
    State(ctx): State<AppContext>,
    mut req: Request,
    next: Next,
) -> Response {
    if !req.uri().path().starts_with("/api/admin/") {
        return next.run(req).await;
    }

    // CORS preflight requests do not include Authorization and must not be
    // blocked by admin auth middleware.
    if req.method() == Method::OPTIONS {
        return next.run(req).await;
    }

    let jar = CookieJar::from_headers(req.headers());
    let token = jar.get("auth_token").map(|c| c.value()).or_else(|| {
        req.headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
    });

    let Some(token) = token else {
        return admin_unauthorized().into_response();
    };

    let Ok(jwt_config) = ctx.config.get_jwt_config() else {
        return admin_unauthorized().into_response();
    };
    let Ok(claims) = jwt::JWT::new(&jwt_config.secret).validate(token) else {
        return admin_unauthorized().into_response();
    };

    let current_user_id = claims
        .claims
        .claims
        .get("user_id")
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok());
    let roles = claims.claims.claims.get("roles").and_then(|value| {
        value.as_array().and_then(|items| {
            items
                .iter()
                .map(|item| item.as_str().map(ToString::to_string))
                .collect::<Option<Vec<_>>>()
        })
    });

    if let (Some(current_user_id), Some(roles)) = (current_user_id, roles) {
        let ability = Ability::for_roles_and_user(roles, Some(current_user_id));
        if !ability.can_manage_admin() {
            return admin_forbidden().into_response();
        }

        req.extensions_mut().insert(AdminSession {
            current_user_id,
            ability,
        });

        return next.run(req).await;
    }

    let Ok(user) = UserModel::find_by_pid(&ctx.db, &claims.claims.pid).await else {
        return admin_unauthorized().into_response();
    };
    let Ok(ability) = Ability::for_user(&user, &ctx.db).await else {
        return admin_unauthorized().into_response();
    };
    if !ability.can_manage_admin() {
        return admin_forbidden().into_response();
    }

    req.extensions_mut().insert(AdminSession {
        current_user_id: user.id,
        ability,
    });

    next.run(req).await
}

fn admin_unauthorized() -> impl IntoResponse {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "error": "unauthorized",
            "description": "A valid Bearer token is required to access this resource"
        })),
    )
}

fn admin_forbidden() -> impl IntoResponse {
    (
        StatusCode::FORBIDDEN,
        Json(json!({
            "error": "forbidden",
            "description": "You do not have permission to access this resource"
        })),
    )
}
