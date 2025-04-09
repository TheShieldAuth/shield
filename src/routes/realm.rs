use axum::{routing::get, Router};

use crate::handlers::realm::{create_realm, delete_realm, get_realm, get_realms, update_realm};

use super::{client, user};

pub fn create_routes() -> Router {
    Router::new().route("/", get(get_realms).post(create_realm)).nest(
        "/{realm_id}",
        Router::new()
            .route("/", get(get_realm).patch(update_realm).delete(delete_realm))
            .nest("/clients", client::create_routes())
            .nest("/users", user::create_routes()),
    )
}
