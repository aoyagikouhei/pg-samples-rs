use tracing::Level;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, Registry};

pub(crate) fn prepare_log(app_name: &str) {
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
    let filter = Targets::new().with_target(app_name, Level::INFO);
    let subscriber = Registry::default()
        .with(filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
