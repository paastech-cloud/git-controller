use anyhow::{Context, Result};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::constants::LOG_FILE_PATH;

pub fn configure_log4rs() -> Result<()> {
    // TODO add timestamp
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(LOG_FILE_PATH)
        .with_context(|| format!("Failed to initialize logfile : {}", LOG_FILE_PATH))?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .with_context(|| format!("Failed to initialize logfile config"))?;

    let _ = log4rs::init_config(config);

    Ok(())
}
