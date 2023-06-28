use std::fs;
use std::path::Path;
use std::process::Command;

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
        // Get the request data
        let request_data = request.into_inner();

        // Create the full path to the repository
        let new_repo_path = format!(
            "{}/{}",
            self.config.git_repository_base_path, request_data.repository_path
        );

        // Check if the repository already exists, if it does return error otherwise continue
        if fs::metadata(new_repo_path.clone()).is_ok() {
            return Err(Status::already_exists(format!(
                "repository {} already exists",
                new_repo_path,
            )));
        }

        // Initialize empty git repository if it fails rollback and return error otherwise continue
        if Command::new("sh")
            .arg("-c")
            .arg(format!("git init --bare {}", new_repo_path))
            .output()
            .is_err()
        {
            return Err(Status::unknown("Failed initializing repository"));
        }

        let hook_path = format!("{}/hooks/post-receive", self.config.githook_base_path);

        let hook_path_exists = Path::new(&hook_path).exists();

        if hook_path_exists {
            // If the hooks path exists create a symbolic link to the post-receive file
            if Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "ln -s {}/hooks/post-receive {}/hooks/post-receive",
                    self.config.githook_base_path, new_repo_path,
                ))
                .output()
                .is_err()
            {
                return Err(Status::unknown("Failed to create symbolic links"));
            }
        } else {
            // clean up the repository if it fails rollback and return error otherwise continue
            if Command::new("sh")
                .arg("-c")
                .arg(format!("rm -rf {}", new_repo_path))
                .output()
                .is_err()
            {
                return Err(Status::unknown("Failed cleaning up repository"));
            }

            return Err(Status::unknown(
                "Failed hooks path to post-receive file doesn't exist",
            ));
        }

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
            self.config.git_repository_base_path.clone(),
            request_data.repository_path
        );

        if fs::metadata(full_repo_path.clone()).is_err() {
            return Err(Status::not_found(""));
        }

        if fs::remove_dir_all(full_repo_path).is_err() {
            return Err(Status::unknown("Failed removing repository"));
        }

        let reply = RepositoryResponse {
            message: "".to_owned(),
        };

        Ok(Response::new(reply))
    }
}
