use axum::{
    routing::{get, patch},
    Router,
};

use crate::handlers::client::{
    api_user::{create_api_user, delete_api_user, get_api_users, update_api_user},
    create_client, delete_client, get_client, get_clients, update_client,
};

use super::auth;

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_clients).post(create_client)).nest(
        "/{client_id}",
        Router::new()
            .route("/", get(get_client).patch(update_client).delete(delete_client))
            .nest(
                "/api-users",
                Router::new()
                    .route("/", get(get_api_users).post(create_api_user))
                    .route("/{api_user_id}", patch(update_api_user).delete(delete_api_user)),
            )
            .nest("/auth", auth::create_routes()),
    )
}
