pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::env;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv::from_filename("/etc/contactbook-api.conf").ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("Connecting to database.");
    PgConnection::establish(&database_url)
}

pub fn get_listen_address() -> String {

    dotenv::from_filename("/etc/contactbook-api.conf").ok().unwrap_or_default();
    let listen_address = match env::var("LISTEN_ADDRESS") {
        Ok(v) => v,
        Err(_) => {
            eprint!("No listen address found, using 127.0.0.1\n");
            String::from("127.0.0.1")
        }
    };

    listen_address.to_owned()

}

pub fn get_listen_port<'a>() -> u16 {

    dotenv::from_filename("/etc/contactbook-api.conf").ok().unwrap_or_default();

    let listen_port = match env::var("LISTEN_PORT") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("No listen port found, using 3030\n");
            String::from("3030")
        }
    };

    let port = match listen_port.parse::<u16>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Could not parse port, using 3030");
            3030
        }
    };
    port
}