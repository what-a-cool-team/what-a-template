use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use api::routers::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    App::serve(8080, "http://localhost:3000")
        .await
        .unwrap();

    Ok(())
}
