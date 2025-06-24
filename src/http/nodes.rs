use axum::{Router, routing::get};

use super::ApiContext;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/nodes", get(get_nodes))
}

async fn get_nodes() -> &'static str {
    "Hello, nodes!"
}
