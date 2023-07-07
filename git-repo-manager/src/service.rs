use std::fs;
use std::path::Path;
use std::process::Command;

use log::error;
use log::info;

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
            // Log
            error!("create_repository: hook is missing at {}", &hook_full_path);
            return Err(Status::unknown(format!(
                "Hook is missing at  {}",
                hook_full_path,
            )));
        }

        let request_data = request.into_inner();

        let new_repo_full_path = format!(
            "{}/{}",
            self.config.git_repository_base_path, request_data.repository_path
        );

        // Log
        info!("Creating new repository : {}", new_repo_full_path);

        // Check if the repository already exists, if it does return error otherwise continue
        if fs::metadata(&new_repo_full_path).is_ok() {
            // Log
            error!(
                "create_repository: repository already exists at {}",
                &new_repo_full_path
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
            // Log
            error!(
                "create_repository: something went wrong when initiazing repository at {}",
                &new_repo_full_path
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
            message: format!("Created repository {}", request_data.repository_path),
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

        // Log
        info!("Deleting Repository : {}", full_repo_path);

        if fs::metadata(&full_repo_path).is_err() {
            // Log
            error!(
                "delete_repository: not found repository at {}",
                &full_repo_path
            );
            return Err(Status::not_found(""));
        }

        if fs::remove_dir_all(&full_repo_path).is_err() {
            // Log
            error!(
                "delete_repository: something went wrong when removing repository at {}",
                &full_repo_path
            );
            return Err(Status::unknown("Failed removing repository"));
        }

        let reply = RepositoryResponse {
            message: "".to_owned(),
        };

        Ok(Response::new(reply))
    }
}
