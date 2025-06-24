use std::net::{Ipv4Addr, SocketAddr};

use crate::config::Config;
use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;

mod nodes;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Config,
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: config.clone(),
    };

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port));
    let listener = TcpListener::bind(addr).await?;
    let app = app_router(api_context);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Error when trying to run HTTP server")
}

fn app_router(api_context: ApiContext) -> Router {
    Router::new().merge(nodes::router()).with_state(api_context)
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
