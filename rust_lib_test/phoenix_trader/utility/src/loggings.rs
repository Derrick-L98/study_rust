use std::io;
use tracing_subscriber::{
    fmt::{self, writer::MakeWriterExt},
    layer::SubscriberExt,
    EnvFilter,
};

pub fn log_init() {
    log4rs::init_file("config/log.yaml", Default::default()).unwrap();
}

pub fn tracing_init() {
    let file_appender_debug = tracing_appender::rolling::daily("./log", "debug.log");
    let (non_blocking_debug, _guard) = tracing_appender::non_blocking(file_appender_debug);

    let file_appender_error = tracing_appender::rolling::daily("./log", "error.log");
    let (non_blocking_error, _guard) = tracing_appender::non_blocking(file_appender_error);

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with(
            fmt::Layer::new()
                .with_writer(io::stdout)
                .with_ansi(true)
                .with_line_number(true)
                .with_target(false),
        )
        .with(
            fmt::Layer::new()
                .with_writer(
                    non_blocking_debug
                        .with_min_level(tracing::Level::INFO)
                        .with_max_level(tracing::Level::DEBUG),
                )
                .with_ansi(false)
                .with_line_number(true)
                .with_target(false),
        )
        .with(
            fmt::Layer::new()
                .with_writer(
                    non_blocking_error
                        .with_min_level(tracing::Level::ERROR)
                        .with_max_level(tracing::Level::WARN),
                )
                .with_ansi(false)
                .with_line_number(true)
                .with_target(false),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
}
