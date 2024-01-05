pub mod http;
pub mod in_memory;
pub mod kernel;
pub mod provider;
pub mod repository;
pub mod response;
pub mod service;
pub mod telemetry;
pub mod usecases;

use std::error::Error;

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = telemetry::get_subscriber(
        "service-http".into(),
        "INFO".into(),
        std::io::stdout,
    );

    telemetry::init_subscriber(subscriber);

    info!("starting service-http");

    let provider = provider::setup();

    http::start(&"0.0.0.0:80".parse()?, provider).await?;

    Ok(())
}
