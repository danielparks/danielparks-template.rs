//! Various logging functions.

use anyhow::bail;
use simplelog::{
    CombinedLogger, Config, ConfigBuilder, LevelFilter, TermLogger,
    TerminalMode,
};

/// Initialize logging for the executable.
pub fn init(verbose: u8) -> anyhow::Result<()> {
    let filter = match verbose {
        4.. => bail!("-v is only allowed up to 3 times."),
        3 => LevelFilter::Trace,
        2 => LevelFilter::Debug,
        1 => LevelFilter::Info,
        0 => LevelFilter::Warn,
    };

    // Configure different logging for a module (sqlx::query here).
    CombinedLogger::init(vec![
        // Default logger
        new_term_logger(
            filter,
            new_logger_config()
                .add_filter_ignore_str("sqlx::query")
                .build(),
        ),
        // Logger for sqlx::query
        new_term_logger(
            LevelFilter::Warn,
            new_logger_config()
                .add_filter_allow_str("sqlx::query")
                .build(),
        ),
    ])
    .unwrap();

    Ok(())
}

/// Convenience function to make creating [`TermLogger`]s clearer.
#[allow(clippy::unnecessary_box_returns)] // Using `Box` isn’t our decision.
fn new_term_logger(level: LevelFilter, config: Config) -> Box<TermLogger> {
    TermLogger::new(
        level,
        config,
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
}

/// Convenience function to make creating [`ConfigBuilder`]s clearer.
fn new_logger_config() -> ConfigBuilder {
    let mut builder = ConfigBuilder::new();
    builder.set_target_level(LevelFilter::Error);

    if builder.set_time_offset_to_local().is_err() {
        // We haven’t finished setting up logging, so just print to stderr.
        eprintln!(
            "Warning: could not get local timezone so logging timestamps will \
            be in UTC."
        );
    }

    builder
}
