#![allow(dead_code)]

use std::time::Duration;

use log::{error, info};
use serde::Deserialize;
use sqlx::{PgPool, QueryBuilder};

use crate::{
    config::Config,
    utils::{sats_to_btc, unix_to_timestamptz},
};

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeLocation {
    de: Option<String>,
    en: String,
    es: Option<String>,
    fr: Option<String>,
    ja: Option<String>,
    #[serde(rename = "pt-BR")]
    pt_br: Option<String>,
    ru: Option<String>,
    #[serde(rename = "zh-CN")]
    zh_cn: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    public_key: String,
    alias: String,
    channels: i64,
    capacity: u64,
    first_seen: u64,
    updated_at: i64,
    city: Option<NodeLocation>,
    country: Option<NodeLocation>,
    #[serde(rename = "iso_code")]
    iso_code: Option<String>,
    subdivision: Option<String>,
}

type Nodes = Vec<Node>;

/// Get all nodes from mempool's API
async fn get_nodes_data() -> anyhow::Result<Nodes> {
    let nodes = reqwest::get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
        .await?
        .json()
        .await?;

    Ok(nodes)
}

/// This functions get all nodes, and save them in batch
async fn save_nodes_data(db: &PgPool, nodes: Nodes) -> anyhow::Result<()> {
    let mut builder =
        QueryBuilder::new("INSERT INTO nodes (public_key, alias, capacity, first_seen)");

    builder.push_values(nodes, |mut b, node| {
        b.push_bind(node.public_key)
            .push_bind(node.alias)
            .push_bind(sats_to_btc(node.capacity))
            .push_bind(unix_to_timestamptz(node.first_seen));
    });

    builder.push(
        " ON CONFLICT (public_key) DO UPDATE SET alias = EXCLUDED.alias, capacity = EXCLUDED.capacity, first_seen = EXCLUDED.first_seen",
    );

    builder.build().execute(db).await?;

    Ok(())
}

/// This function runs at each interval defined by POOLING_INVERVAL_IN_SECONDS (env var)
/// in order to synchronize the mempool API data with the application database.
pub async fn synchronizer(config: Config, db: PgPool) {
    let mut interval =
        tokio::time::interval(Duration::from_secs(config.pooling_inverval_in_seconds));

    loop {
        interval.tick().await;

        info!("Synchronizing node data");

        match get_nodes_data().await {
            Ok(nodes) => {
                if let Err(e) = save_nodes_data(&db, nodes).await {
                    error!("Error when trying to save nodes data: {e}")
                }
            }
            Err(e) => error!("Error when trying to get nodes data: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{PgPool, Row};

    #[tokio::test]
    async fn test_get_nodes_data() {
        let result = get_nodes_data().await;

        assert!(result.is_ok(), "Expected Ok(...)");
    }

    #[sqlx::test]
    async fn test_save_nodes_data(db: PgPool) {
        let node = Node {
            public_key: "020d1617e27ac022395352f2b3774969593d3d6ddff6fb117d820a9dda8da45217"
                .to_string(),
            alias: "exampleNode".to_string(),
            channels: 10,
            capacity: 100,
            first_seen: 1625097600,
            updated_at: 1729968000,
            city: Some(NodeLocation {
                de: Some("München".to_string()),
                en: "Munich".to_string(),
                es: Some("Múnich".to_string()),
                fr: Some("Munich".to_string()),
                ja: None,
                pt_br: Some("Munique".to_string()),
                ru: Some("Мюнхен".to_string()),
                zh_cn: Some("慕尼黑".to_string()),
            }),
            country: Some(NodeLocation {
                de: Some("Deutschland".to_string()),
                en: "Germany".to_string(),
                es: Some("Alemania".to_string()),
                fr: Some("Allemagne".to_string()),
                ja: Some("ドイツ".to_string()),
                pt_br: Some("Alemanha".to_string()),
                ru: Some("Германия".to_string()),
                zh_cn: Some("德国".to_string()),
            }),
            iso_code: Some("DE".to_string()),
            subdivision: Some("Bavaria".to_string()),
        };

        let _ = save_nodes_data(&db, vec![node.clone()]).await;

        let row = sqlx::query("SELECT public_key, alias FROM nodes")
            .fetch_one(&db)
            .await
            .unwrap();
        let public_key: String = row.get("public_key");

        assert_eq!(public_key, node.public_key);
    }
}
