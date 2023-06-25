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

        // Get the request data
        let request_data = request.into_inner();

        let new_repo_path = format!(
            "{}/{}",
            self.config.git_repository_base_path.clone(),
            request_data.repository_path
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

        // add post-receive hook : le lien symbolique peut être remplacé
        if Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ln -s ./hooks/post-receive {}/.git/hooks/post-receive",
                new_repo_path.clone()
            ))
            .output()
            .is_err()
        {
            return Err(Status::unknown("Failed adding post-receive hook"));
        }

        // Make the hook file executable
        if Command::new("sh")
            .arg("-c")
            .arg(format!(
                "chmod +x {}/.git/hooks/post-receive",
                new_repo_path.clone()
            ))
            .output()
            .is_err()
        {
            return Err(Status::unknown("Failed making post-receive hook executable"));
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
