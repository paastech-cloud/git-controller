use std::env;

use crate::constants::{DB_HOST_KEY, DB_NAME_KEY, DB_PASSWORD_KEY, DB_PORT_KEY, DB_USER_KEY};

use anyhow::{bail, Context};
use regex::Regex;
use sqlx::postgres::PgConnectOptions;
use sqlx::ConnectOptions;

/// Checks wether the given command is `git-receive-pack` followed by an uuid and .git
///
/// Returns true if it is valid, otherwise false
///
/// # Arguments
///
/// * `command` A string of the command to validate
///
pub fn is_command_valid(command: &str) -> bool {
    let regex = Regex::new(r#"^git-receive-pack\s'[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\.git'$"#).unwrap();

    log::info!("{}", &command);

    regex.is_match(command)
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
    repository_name: &str,
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
        "SELECT p.id FROM projects p WHERE p.user_id = $1 AND p.uuid::text = $2",
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

/// Returns the user_id from the arguments it should be added in the authorized_keys files as described in the documentation
///
/// It fails if the command is either :
///
/// - Badly formed / the user_id is missing
/// - The user_id is not an i32
pub fn get_user_id_from_args() -> anyhow::Result<i32> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        bail!("Badly formed authorized_keys entry");
    }

    // Get the user id from args
    let user_id: i32 = args[1]
        .parse()
        .with_context(|| format!("Failed extracting user_id as i32 from args {:?}", args))?;

    Ok(user_id)
}

#[cfg(test)]
mod tests {
    use crate::utils::is_command_valid;

    #[test]
    fn commands_should_be_valid() {
        let command_a = "git-receive-pack '210e364a-3a07-43ba-85b8-2e1c646bd39a.git'";
        let command_b = "git-receive-pack '210e364b-3a09-43ba-85b8-2e1c646bd39a.git'";

        assert_eq!(is_command_valid(command_a), true);
        assert_eq!(is_command_valid(command_b), true);
    }

    #[test]
    fn commands_should_not_be_valid() {
        let command_a = "git-send-pack '210e364a-3a07-43ba-85b8-2e1c646bd39a.git'";
        let command_b = "git-receive-pack '/srv/210e364a-3a07-43ba-85b8-2e1c646bd39a'";
        let command_c = "git-receive-pack '210e364a-3a07-43ba-858-2e1c6bd39a.git'";

        assert_eq!(is_command_valid(command_a), false);
        assert_eq!(is_command_valid(command_b), false);
        assert_eq!(is_command_valid(command_c), false);
    }
}
