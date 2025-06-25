use crate::utils::f64_to_string;
use axum::{Json, Router, extract::State, routing::get};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{ApiContext, Result};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/nodes", get(get_nodes))
}

#[derive(Deserialize, Serialize)]
struct Node {
    public_key: String,
    alias: String,
    #[serde(serialize_with = "f64_to_string")]
    capacity: Option<f64>,
    #[serde(with = "time::serde::rfc3339")]
    first_seen: OffsetDateTime,
}

async fn get_nodes(state: State<ApiContext>) -> Result<Json<Vec<Node>>> {
    let nodes = sqlx::query_as!(
        Node,
        r#"
        SELECT public_key, alias, capacity::float8, first_seen
        FROM nodes
        "#
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(nodes))
}
