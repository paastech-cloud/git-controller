use std::env;

use postgres::{Client, Error, NoTls};

pub fn connect() {
    println!("Connecting to database...");

    let host = env::var("POSTGRES_HOST").unwrap();
    let user = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PWD").unwrap();
    let db = env::var("POSTGRES_DB").unwrap();
    let port_result = env::var("POSTGRES_PORT");
    let mut port = String::from("5432");
    if port_result.is_ok() {
        port = port_result.unwrap();
    }

    let url = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, db);

    let mut client = Client::connect(
      &url,
      NoTls
    );

    if !client.is_ok() {
        let error = client.err().unwrap().to_string();
        println!("Failed to connect to database {} on host {} as user {}", db, host, user);
        println!("{}", error);
        return;
    }

    println!("Connected successfully to database {} on host {} as user {}", db, host, user);
    //TODO

    return;
}