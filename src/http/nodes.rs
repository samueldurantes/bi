use crate::utils::{f64_to_string, string_to_f64};
use axum::{Json, Router, extract::State, routing::get};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{ApiContext, Result};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/nodes", get(get_nodes))
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Node {
    public_key: String,
    alias: String,
    #[serde(serialize_with = "f64_to_string", deserialize_with = "string_to_f64")]
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

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sqlx::PgPool;
    use time::format_description::well_known::Rfc3339;
    use tower::util::ServiceExt;

    #[sqlx::test]
    async fn test_get_nodes(db: PgPool) {
        let config = Config {
            database_url: "".to_owned(),
            port: 8080,
            pooling_inverval_in_seconds: 60,
        };
        let state = ApiContext {
            db: db.clone(),
            config,
        };
        let app = Router::new()
            .route("/nodes", get(get_nodes))
            .with_state(state);

        let node = Node {
            public_key: "0284aa8233ee3c0482ecf87c60094cacd15a17feb0bfa59f1898fe91478020eb76"
                .to_owned(),
            alias: "Imamura".to_owned(),
            capacity: Some(9.95841213),
            first_seen: OffsetDateTime::parse("2024-01-05T06:18:52+00:00", &Rfc3339).unwrap(),
        };

        sqlx::query(
            r#"
            INSERT INTO nodes (public_key, alias, capacity, first_seen)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(&node.public_key)
        .bind(&node.alias)
        .bind(&node.capacity)
        .bind(&node.first_seen)
        .execute(&db)
        .await
        .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/nodes")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let response_nodes: Vec<Node> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(response_nodes, vec![node]);
    }
}
