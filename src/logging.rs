use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn initialize_logging() {
    tracing_subscriber::registry()
        .with(tui_logger::TuiTracingSubscriberLayer)
        .with(tracing_subscriber::filter::LevelFilter::DEBUG)
        .init();
    tui_logger::init_logger(log::LevelFilter::Warn).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Trace);
}
