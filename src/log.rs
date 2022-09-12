pub use log;
use miette::{IntoDiagnostic, Result};
pub use tracing::Level;
use tracing_subscriber::fmt;

pub enum Format {
    Default,
    Trace
}

pub fn logger_init(level: tracing::Level, format: Format) -> Result<()> {
    match format {
        Format::Default => {
            let subscriber = tracing_subscriber::fmt()
                .with_max_level(level)
                .event_format(
                    fmt::format()
                        .without_time()
                        .with_target(false)
                        .with_level(false)
                        .compact()
                )
                .finish();
            tracing::subscriber::set_global_default(subscriber)
                .into_diagnostic()?
        }
        Format::Trace => tracing::subscriber::set_global_default(
            tracing_subscriber::fmt()
                .with_max_level(level)
                .event_format(fmt::format().pretty())
                .finish()
        )
        .into_diagnostic()?
    };
    tracing_log::LogTracer::init().into_diagnostic()?;

    Ok(())
}
