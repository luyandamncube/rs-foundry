// src\bin\runner_api.rs
use rs_foundry::api::{routes, state::AppState};
use rs_foundry::config::{paths::DataPaths, settings::Settings};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = Settings::load().map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let data_paths = DataPaths::new(&settings.data_root);
    data_paths.ensure_layout()?;

    let bind_addr = settings.bind_addr();
    let state = AppState {
        settings: settings.clone(),
    };

    let app = routes::router(state);
    let listener = TcpListener::bind(&bind_addr).await?;

    tracing::info!(
        app_name = %settings.app_name,
        app_env = %settings.app_env,
        bind_addr = %bind_addr,
        data_root = %settings.data_root,
        "runner_api listening"
    );

    axum::serve(listener, app).await?;
    Ok(())
}
