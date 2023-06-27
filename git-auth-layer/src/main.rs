mod config;
mod constants;
mod utils;

use config::configure_log4rs;

use std::{
    env,
    process::{exit, Command},
};
use utils::{check_user_repository_access, get_user_id_from_args};

use crate::{
    constants::{CONFIG_FILE_PATH, GIT_REPOSITORIES_BASE_PATH_KEY, SSH_ORIGINAL_COMMAND_KEY},
    utils::is_command_valid,
};

// TODO custom errors et tout ouais
#[tokio::main]
async fn main() {
    // get configuration
    let _ = dotenvy::from_filename(&CONFIG_FILE_PATH).unwrap_or_else(|_| {
        exit(1);
    });

    let _ = configure_log4rs().unwrap_or_else(|_| {
        exit(1);
    });

    // Extract args there should be the command itself and a string to identify the user
    let user_id = get_user_id_from_args().unwrap_or_else(|e| {
        log::error!("{:?}", e);
        exit(1);
    });

    // Extract ssh command from environment
    let ssh_command = env::var(SSH_ORIGINAL_COMMAND_KEY).unwrap_or_else(|_| {
        log::error!("Client did not use git cli correctly");
        exit(1);
    });

    if !is_command_valid(&ssh_command) {
        log::error!("Client executed an invalid command");
        exit(1);
    }

    let git_repositories_base_path =
        env::var(GIT_REPOSITORIES_BASE_PATH_KEY).unwrap_or_else(|_| {
            log::error!("Missing {} configuration", GIT_REPOSITORIES_BASE_PATH_KEY);
            exit(1);
        });

    let repository_name = ssh_command
        .split(' ')
        .last()
        .unwrap_or_else(|| {
            log::error!("Unexpected error when extracting repository name from command");
            exit(1);
        })
        .replace('\'', "")
        .replace(".git", "")
        .as_str()
        .to_owned();

    let repository_path = format!("{}/{}.git", git_repositories_base_path, repository_name);

    match check_user_repository_access(user_id, &repository_name).await {
        Ok(_) => {
            // Execute git-receive-pack
            Command::new("git-receive-pack")
                .arg(&repository_path)
                .status()
                .unwrap_or_else(|_| {
                    log::error!(
                        "Error when git-receive-pack on repository {}",
                        repository_path
                    );
                    exit(1);
                });

            log::info!(
                "Client {} successfully push to repository {}",
                user_id,
                repository_name
            );
        }
        Err(sqlx::Error::RowNotFound) => {
            log::info!(
                "Client {} did not have access to repository {}",
                user_id,
                repository_name
            );
            exit(1);
        }
        Err(e) => {
            log::error!("Unexpected error when querying database {:?}", e);
            exit(1);
        }
    };
}
