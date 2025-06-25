#![allow(dead_code)]

use std::time::Duration;

use log::{error, info};
use serde::Deserialize;
use sqlx::{PgPool, QueryBuilder};

use crate::utils::{sats_to_btc, unix_to_timestamptz};

const POOLING_INTERVAL: u64 = 60;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

/// This function runs at each interval defined by POOLING_INTERVAL in order to
/// synchronize the mempool API data with the application database.
pub async fn synchronizer(db: PgPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(POOLING_INTERVAL));

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
