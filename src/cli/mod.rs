use clap::{Parser, Subcommand};

use tufa_client::db::{self, models};
use anyhow::Result;

mod add;
mod gen;
mod rm;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Tufa {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a service
    Add(add::Cmd),

    /// Generate a new 2FA code
    Gen(gen::Cmd),

    // Remove a service
    Rm(rm::Cmd),

    // List all the registered services
    Ls,
}

impl Tufa {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Some(Commands::Add(cmd)) => cmd.run(),
            Some(Commands::Gen(cmd)) => cmd.run(),
            Some(Commands::Rm(cmd)) => cmd.run(),
            Some(Commands::Ls) => { list_services() },
            _ => Ok(())
        }
    }
}

fn list_services() -> Result<()>{
    let mut conn = db::establish_connection();

    models::service::get_all(&mut conn)?.into_iter()
        .map(|service| service.name)
        .for_each(|name| println!("{}", name));

    Ok(())
}

