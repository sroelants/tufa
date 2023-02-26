use clap::Parser;
use anyhow::{Result, Error};
use console::style;
use dialoguer::Confirm;

use tufa_client::db::{self, models};

#[derive(Parser)]
#[command(infer_subcommands = true)]
pub struct Cmd {
    /// The service you'd like to remove
    service: String,
}

impl Cmd {
    pub fn run(&self) -> Result<()>{
        let mut conn = db::establish_connection();
        models::service::get_by_name(&mut conn, &self.service)
            .ok_or(Error::msg(
                format!("Service {} not found.", 
                style(&self.service).blue())
            ))?;

        let confirmed = Confirm::new()
        .with_prompt(
            format!("{} {}?", 
                style("⚠️ Are you sure you want to remove 2FA for").red(), 
                style(&self.service).blue().bold()
            )
        )
        .interact()
        .is_ok();

        if confirmed {
            models::service::remove(&mut conn, &self.service)?;
        }

        Ok(())
    }
}
