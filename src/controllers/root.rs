use axum::{routing::get, Router};

use crate::services::root::handler;

pub async fn register_root_controller(app: Router) -> Router {
    return app.route("/", get(handler));
}
