mod constants;
mod utils;

use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use std::process::{exit, Command};
use utils::{check_user_repository_access, get_repo_name};

use crate::constants::{NOT_FOUND_ERROR, UNEXPECTED_ERROR};

#[tokio::main]
async fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("/var/paastech/git-auth-layer/log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    let _ = log4rs::init_config(config);

    log::info!("Client connected");

    // Extract args there should be the command itself and a string to identify the user
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        log::error!("Args : {:?} malformed", args);
        eprintln!("{}", &UNEXPECTED_ERROR);
        exit(1);
    }

    let user_id: i32 = args[1].parse().unwrap();
    let repository_path = match get_repo_name() {
        Some(value) => value,
        None => {
            log::error!("Failed extracting repository name from command");
            eprintln!("{}", UNEXPECTED_ERROR);
            exit(1);
        }
    };

    let repository_name = repository_path.replace("/srv/", "");
    match check_user_repository_access(user_id, repository_name).await {
        Ok(_) => {
            // Execute git-receive-pack
            Command::new("git-receive-pack")
                .arg(&repository_path)
                .status()
                .ok();
        }
        Err(sqlx::Error::RowNotFound) => {
            log::info!(
                "Client {} did not have access to repository {}",
                user_id,
                repository_path
            );
            eprintln!("{}", &NOT_FOUND_ERROR);
            exit(1);
        }
        Err(_) => {
            log::error!("Unexpected error when querying database");
            eprintln!("{}", &UNEXPECTED_ERROR);
            exit(1);
        }
    };
}
