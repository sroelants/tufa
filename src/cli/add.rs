use clap::Parser;
use anyhow::Result;
use dialoguer::Password;

use crate::{crypto::AesData, db::{self, models}};

#[derive(Parser)]
#[command(infer_subcommands = true)]
pub struct Cmd {
    /// The name of the service you'd like to add 2FA for
    service: String,

    /// The 2FA secret provided by the service
    secret: String,

    /// Encrypt the secret with a password
    #[arg(short, long)]
    encrypted: bool,
}

impl Cmd {
    pub fn run(&self) -> Result<()>{
        let mut conn = db::establish_connection();

        if self.encrypted {
            let password = Password::new()
            .with_prompt("Please provide a password to encrypt the secret.")
            .interact()?;

            let secret: String = AesData::encrypt(&self.secret, &password)?.into();
            models::service::create(&mut conn, &self.service, &secret, true)?;
        } else {
            models::service::create(&mut conn, &self.service, &self.secret, false)?;
        }

        Ok(())
    }
}
