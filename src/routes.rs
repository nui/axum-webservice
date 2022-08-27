use axum::routing::{get, post};
use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .route("/hello", post(crate::handlers::hello))
        .route("/now", get(crate::handlers::now))
}
