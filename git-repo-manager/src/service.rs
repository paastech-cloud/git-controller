use std::fs;
use std::process::Command;

use paastech_proto::gitrepomanager::git_repo_manager_server::GitRepoManager;
use paastech_proto::gitrepomanager::{RepositoryRequest, RepositoryResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GitRepoManagerServiceConfig {
    pub git_repository_base_path: String,
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
        let request_data = request.into_inner();

        let new_repo_path = format!(
            "{}/{}",
            self.config.git_repository_base_path.clone(),
            request_data.repository_path
        );

        // Check if the repository already exists, if it does return error otherwise continue
        if let Ok(_) = fs::metadata(new_repo_path.clone()) {
            return Err(Status::already_exists(format!(
                "repository {} already exists",
                new_repo_path.clone(),
            )));
        }

        // Initialize empty git repository if it fails rollback and return error otherwise continue
        if Command::new("sh")
            .arg("-c")
            .arg(format!("git init --bare {}", new_repo_path.clone()))
            .output()
            .is_err()
        {
            return Err(Status::unknown("Failed initializing repository"));
        }

        // add post-receive hook
        if Command::new("sh")
            .arg("-c")
            .arg(format!(
                "cp ./hooks/post-receive {}",
                format!("{}/hooks/", new_repo_path.clone())
            ))
            .output()
            .is_err()
        {
            return Err(Status::unknown("Failed adding post-receive hook"));
        }

        let reply = RepositoryResponse {
            message: format!("Created repository {}", request_data.repository_path).to_owned(),
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

        if let Err(_) = fs::metadata(full_repo_path.clone()) {
            return Err(Status::not_found(""));
        }

        if let Err(_) = fs::remove_dir_all(full_repo_path) {
            return Err(Status::unknown("Failed removing repository"));
        }

        let reply = RepositoryResponse {
            message: "".to_owned(),
        };

        Ok(Response::new(reply))
    }
}
