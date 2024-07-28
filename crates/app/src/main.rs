use anyhow::Context;
use clap::{arg, command};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::log::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::routers::Api;
use domain::services::service_registry::ServiceRegistry;
use settings::settings::Settings;
use filesystem::RealFileSystem;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let matches = command!()
        .arg(arg!(-c --config <FILE> "Config file path").required(true))
        .get_matches();
    let config_path = matches.get_one::<String>("config").unwrap();

    let fs = RealFileSystem;
    let settings = Settings::from(&fs, config_path).unwrap();

    info!("Initializing database connection...");
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.connection_url)
        .await
        .context("Error while connecting database")?;

    if settings.database.migrate_on_startup {
        info!("Migrating database...");
        sqlx::migrate!()
            .run(&pool)
            .await
            .context("Error while running database migrations")?
    }

    let service_registry = ServiceRegistry::new(pool);

    info!("Starting server...");
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", settings.server.port))
        .await
        .unwrap();
    axum::serve(listener, Api::new(service_registry.clone()))
        .await
        .unwrap();

    Ok(())
}
