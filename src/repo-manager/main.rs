use log::info;

mod constants;
mod service;

use std::env::VarError;

use constants::{GIT_HOOKS_BASE_PATH_KEY, GIT_REPOSITORY_BASE_PATH_KEY, SERVER_PORT_KEY};
use dotenv::dotenv;
use paastech_proto::gitrepomanager::git_repo_manager_server::GitRepoManagerServer;
use service::{GitRepoManagerService, GitRepoManagerServiceConfig};
use tonic::transport::Server;

fn load_env_into_config() -> Result<GitRepoManagerServiceConfig, VarError> {
    Ok(GitRepoManagerServiceConfig {
        git_repository_base_path: std::env::var(GIT_REPOSITORY_BASE_PATH_KEY).unwrap_or_else(
            |error| panic!("Err {}: env var {}", error, GIT_REPOSITORY_BASE_PATH_KEY),
        ),
        githook_base_path: std::env::var(GIT_HOOKS_BASE_PATH_KEY)
            .unwrap_or_else(|error| panic!("Err {}: env var {}", error, GIT_HOOKS_BASE_PATH_KEY)),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();

    let addr = format!(
        "[::1]:{}",
        std::env::var(SERVER_PORT_KEY)
            .unwrap_or_else(|error| panic!("Err {}: env var {}", error, SERVER_PORT_KEY))
    )
    .parse()?;

    let gitsprout_config = load_env_into_config()?;

    let gitsprout_service = GitRepoManagerService {
        config: gitsprout_config,
    };

    info!(
        "server started succesfully on port : {}",
        std::env::var(SERVER_PORT_KEY)
            .unwrap_or_else(|error| panic!("Err {}: env var {}", error, SERVER_PORT_KEY))
    );

    Server::builder()
        .add_service(GitRepoManagerServer::new(gitsprout_service))
        .serve(addr)
        .await?;

    Ok(())
}
