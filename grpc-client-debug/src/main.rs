use paastech_proto::gitrepomanager::git_repo_manager_client::GitRepoManagerClient;
use paastech_proto::gitrepomanager::RepositoryRequest;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection to the gRPC server
    let mut client = GitRepoManagerClient::connect("http://[::1]:50051").await?;

    // Prepare the request to create a repository
    let request = tonic::Request::new(RepositoryRequest {
        repository_path: "charley".to_owned(),
    });

    // Call the create_repository RPC method
    let response = client.create_repository(request).await?;

    // Process the response
    let repository_response = response.get_ref();
    println!("Response: {}", repository_response.message);

    Ok(())
}
