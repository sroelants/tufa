use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::sqlite::Sqlite;
use dotenv::dotenv;
use std::env::{self, VarError};
use std::error::Error;

pub mod models;
pub mod schema;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .or(get_xdg_config_path())
        .expect("DATABASE_URL must be set.");

    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    run_migrations(&mut connection).unwrap();

    connection
}

fn get_xdg_config_path() -> Result<String, VarError> {
    env::var("XDG_CONFIG_HOME")
        .map(|dir| std::path::Path::new(&dir)
            .join("./tufa.db")
            .to_str()
            .unwrap()
            .to_owned()
        )
}
