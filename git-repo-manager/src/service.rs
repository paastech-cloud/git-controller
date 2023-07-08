use std::fs;
use std::path::Path;
use std::process::Command;

use log::debug;
use log::info;
use log::trace;

use paastech_proto::gitrepomanager::git_repo_manager_server::GitRepoManager;
use paastech_proto::gitrepomanager::{RepositoryRequest, RepositoryResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GitRepoManagerServiceConfig {
    pub git_repository_base_path: String,
    pub githook_base_path: String,
}

#[derive(Debug, Default)]
pub struct GitRepoManagerService {
    pub config: GitRepoManagerServiceConfig,
}

type GitSproutResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl GitRepoManager for GitRepoManagerService {
    async fn create_repository(
        &self,
        request: Request<RepositoryRequest>,
    ) -> GitSproutResult<RepositoryResponse> {
        let hook_full_path = format!("{}/post-receive", self.config.githook_base_path);

        // Check if hook is present
        if !Path::new(&hook_full_path).exists() {
            return Err(Status::unknown(format!(
                "hook is missing at  {}",
                hook_full_path,
            )));
        }

        let request_data = request.into_inner();

        let new_repo_full_path = format!(
            "{}/{}",
            self.config.git_repository_base_path, request_data.repository_path
        );

        trace!("creating new repository : {}", new_repo_full_path);

        // Check if the repository already exists, if it does return error otherwise continue
        if fs::metadata(&new_repo_full_path).is_ok() {
            debug!(
                "error while creating repository : already exist at {}",
                new_repo_full_path
            );
            return Err(Status::already_exists(format!(
                "repository {} already exists",
                new_repo_full_path,
            )));
        }

        // Initialize empty git repository
        if Command::new("sh")
            .arg("-c")
            .arg(format!("git init --bare {}", new_repo_full_path))
            .output()
            .is_err()
        {
            debug!(
                "error while creating repository : failed initilizing at {}",
                new_repo_full_path
            );
            return Err(Status::unknown("Failed initializing repository"));
        }

        // No need to check for errors since
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ln -s {}/post-receive {}/hooks/post-receive",
                self.config.githook_base_path, new_repo_full_path,
            ))
            .output()
            .ok();

        let reply = RepositoryResponse {
            message: format!("created repository {}", request_data.repository_path),
        };

        Ok(Response::new(reply))
    }

    async fn delete_repository(
        &self,
        request: Request<RepositoryRequest>,
    ) -> Result<Response<RepositoryResponse>, Status> {
        let request_data = request.into_inner();

        let full_repo_path = format!(
            "{}/{}",
            &self.config.git_repository_base_path, request_data.repository_path
        );

        trace!("deleting repository : {}", full_repo_path);

        if fs::metadata(&full_repo_path).is_err() {
            debug!(
                "error while deleting repository :  not found at {}",
                full_repo_path
            );
            return Err(Status::not_found(""));
        }

        if fs::remove_dir_all(&full_repo_path).is_err() {
            debug!(
                "error while deleting repository :  failed when removing repository {}",
                full_repo_path
            );
            return Err(Status::unknown("Failed removing repository"));
        }

        let reply = RepositoryResponse {
            message: "".to_owned(),
        };

        Ok(Response::new(reply))
    }
}
