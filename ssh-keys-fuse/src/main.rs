mod postgres_fs;

use dotenv::dotenv;
use postgres_fs::PostgresFS;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // This line loads the environment variables

    // git_auth_binary_path: std::env::var(AUTH_LAYER_PATH).expect("AUTH_LAYER_PATH must be set."),

    // Mount the filesystem
    let url_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let filesystem = PostgresFS::new(url_connection_string).await.unwrap();
    let mountpoint = std::env::var("MOUNT_POINT").expect("MOUNTPOINT must be set.");
    PostgresFS::mount(filesystem, mountpoint);

    Ok(())
}
