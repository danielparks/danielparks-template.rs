use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelFilter,
    TermLogger, TerminalMode,
};
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Params {
}

async fn main() {
    if let Err(error) = cli(Params::from_args()).await {
        eprintln!("Error: {:#}", error);
        exit(1);
    }
}

async fn cli(params: Params) -> anyhow::Result<()> {
    // Configure different logging for a module (sqlx::query here).
    CombinedLogger::init(vec![
        // Default logger
        new_term_logger(
            LevelFilter::Info,
            new_logger_config().add_filter_ignore_str("sqlx::query").build(),
        ),
        // Logger for sqlx::query
        new_term_logger(
            LevelFilter::Warn,
            new_logger_config().add_filter_allow_str("sqlx::query").build(),
        ),
    ])
    .unwrap();

    Ok(())
}

fn new_term_logger(level: LevelFilter, config: Config) -> Box<TermLogger> {
    TermLogger::new(level, config, TerminalMode::Mixed, ColorChoice::Auto)
}

fn new_logger_config() -> ConfigBuilder {
    let mut builder = ConfigBuilder::new();
    builder.set_time_to_local(true);
    builder.set_target_level(LevelFilter::Error);
    builder
}
