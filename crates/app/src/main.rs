use clap::{arg, command};
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

    let settings = Settings::new(config_path).unwrap();

    let service_registry = ServiceRegistry::new();

    Api::serve(settings.server.port, service_registry.clone())
        .await
        .unwrap();

    Ok(())
}
