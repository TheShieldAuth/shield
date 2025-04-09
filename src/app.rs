use axum::{http::header, Router};
use sea_orm::DatabaseConnection;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer, sensitive_headers::SetSensitiveHeadersLayer,
};
use tracing::{error, info};

use crate::{
    middleware::logger::logger,
    packages::{
        admin,
        db::get_db_connection_pool,
        logger,
        settings::{Settings, SETTINGS},
    },
    routes,
};

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub db: DatabaseConnection,
}

pub async fn create_app() -> Router {
    logger::setup();
    let db = get_db_connection_pool()
        .await
        .map_err(|e| error!("⛑️ Failed to get database connection pool: {}", e))
        .unwrap();

    let is_settings_reloaded = admin::setup(&db).await.expect("Failed to setup admin account");
    if is_settings_reloaded {
        info!("New admin credentials initialized and settings reloaded!");
    }

    let app_state = AppState {
        settings: SETTINGS.read().clone(),
        db,
    };
    Router::new()
        .with_state(app_state)
        .merge(routes::create_routes())
        .layer(logger())
        // Mark the `Authorization` request header as sensitive so it doesn't show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(header::AUTHORIZATION)))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static("x-request-id")))
        .layer(CorsLayer::permissive()) // TODO: Update it later
}
