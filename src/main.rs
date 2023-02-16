mod otp;
mod db;
mod crypto;
mod cli;
mod util;

use clap::Parser;
use cli::{Cli, Commands};
use db::models::service;
use diesel::SqliteConnection;

fn main() {
    let cli = Cli::parse();
    let mut connection = db::establish_connection();

    match &cli.command {
        Some(Commands::Add { service, secret }) => { add_service(&mut connection, service, secret)}
        Some(Commands::Gen { service }) => { get_service(&mut connection, service)}
        _ => {}
        // Some(Commands::Remove { service }) => {}
        // None => {}
    }
}

fn add_service(conn: &mut SqliteConnection, name: &str, secret: &str) {
    let service = service::create(conn, name, secret);
    println!("{:?}", service);
}

fn get_service(conn: &mut SqliteConnection, name: &str) {
    let service = service::get_by_name(conn, name);
    println!("{:?}", service);
}
