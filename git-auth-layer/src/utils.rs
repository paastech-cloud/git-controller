use std::process::exit;

use crate::constants::{SSH_ORIGINAL_COMMAND_KEY, UNEXPECTED_ERROR};
use sqlx::postgres::PgConnectOptions;
use sqlx::ConnectOptions;

pub fn get_repo_name() -> String {
    // Extract ssh_command
    let raw_ssh_command = std::env::var(SSH_ORIGINAL_COMMAND_KEY).unwrap_or_else(|_| {
        eprintln!("{}", &UNEXPECTED_ERROR);
        exit(1);
    });

    // get repo name
    match raw_ssh_command.split_whitespace().last() {
        Some(value) => value.to_owned(),
        None => {
            eprintln!("{}", &UNEXPECTED_ERROR);
            exit(1);
        }
    }
}

pub async fn check_user_repository_access(
    user_id: i32,
    repository_name: String,
) -> Result<bool, sqlx::Error> {
    let mut connection = PgConnectOptions::new()
        .username("paastech")
        .password("paastech")
        .host("localhost")
        .port(5432)
        .database("paatech")
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
