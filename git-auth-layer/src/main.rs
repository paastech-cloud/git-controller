mod constants;
mod utils;

use std::process::{exit, Command};

use utils::{check_user_repository_access, get_repo_name};

use crate::constants::{NOT_FOUND_ERROR, UNEXPECTED_ERROR};

#[tokio::main]
async fn main() {
    // Extract args there should be the command itself and a string to identify the user
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", &UNEXPECTED_ERROR);
        exit(1);
    }

    let repository_path = get_repo_name();
    let user_id: i32 = args[1].parse().unwrap();

    // query database
    let repository_name = repository_path.replace("/srv/", "");
    match check_user_repository_access(user_id, repository_name).await {
        Ok(_) => {
            // Execute git-receive-pack
            Command::new("git-receive-pack")
                .arg(&repository_path)
                .status()
                .ok();
        }
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("{}", &NOT_FOUND_ERROR);
            exit(1);
        }
        Err(_) => {
            eprintln!("{}", &UNEXPECTED_ERROR);
            exit(1);
        }
    };
}
