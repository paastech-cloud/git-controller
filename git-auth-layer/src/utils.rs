use std::env;

use crate::constants::{
    DB_HOST_KEY, DB_NAME_KEY, DB_PASSWORD_KEY, DB_PORT_KEY, DB_USER_KEY, SSH_ORIGINAL_COMMAND_KEY,
};

use regex::Regex;
use sqlx::postgres::PgConnectOptions;
use sqlx::ConnectOptions;

/// Returns the repository name if present.
/// When the binary is executed when connecting to the server over ssh, ssh adds the SSH_ORIGINAL_COMMAND
/// it is supposed to be "git-receive-pack <repo_name>" if it is, extract the repo name and return it
/// otherwise return None
pub fn get_repo_name() -> Option<String> {
    // regex to check if command is
    let regex = Regex::new(r#"/^git-receive-pack\s\/srv\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/"#).unwrap();

    // Extract ssh_command
    if let Some(ssh_command) = std::env::var(SSH_ORIGINAL_COMMAND_KEY).ok() {
        if !regex.is_match(&ssh_command) {
            return None;
        }

        return Some(ssh_command.split(" ").last().unwrap().to_owned());
    }

    None
}

/// Returns wether the user has access to the repository, meaning there exists a row in the projects
/// table with given repository_name and user_id.
///
/// Returns `sqlx::Error` if there is any of these issues :
///
/// * couldn't connect to database
///
/// * couldn't find the row which returns `sqlx::Error::RowNotFound`
///
/// # Arguments
///
/// * `name` - An integer representing the user_id in the database
///
/// * `repository_name` - The repository name the user is trying to access
///
pub async fn check_user_repository_access(
    user_id: i32,
    repository_name: String,
) -> Result<bool, sqlx::Error> {
    let mut connection = PgConnectOptions::new()
        .username(&env::var(DB_USER_KEY).unwrap())
        .password(&env::var(DB_PASSWORD_KEY).unwrap())
        .host(&env::var(DB_HOST_KEY).unwrap())
        .port(env::var(DB_PORT_KEY).unwrap().parse().unwrap())
        .database(&env::var(DB_NAME_KEY).unwrap())
        .connect()
        .await?;

    match sqlx::query_as::<_, (i32,)>(
        "SELECT p.id FROM Project p WHERE p.userId = $1 AND p.uuid = $2",
    )
    .bind(user_id)
    .bind(repository_name)
    .fetch_one(&mut connection)
    .await
    {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}
