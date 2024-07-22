use anyhow::Context;
use clap::{arg, command};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use api::routers::Api;
use domain::services::service_registry::ServiceRegistry;
use settings::settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let matches = command!()
        .arg(arg!(-c --config <FILE> "Config file path").required(true))
        .get_matches();
    let config_path = matches.get_one::<String>("config").unwrap();

    let settings = Settings::from(config_path).unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.connection_url)
        .await
        .context("Error while connecting database")?;

    if settings.database.migrate_on_startup {
        sqlx::migrate!()
            .run(&pool)
            .await
            .context("Error while running database migrations")?
    }

    let service_registry = ServiceRegistry::new(pool);

    Api::serve(settings.server.port, service_registry.clone())
        .await
        .unwrap();

    Ok(())
}
