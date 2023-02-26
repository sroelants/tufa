use std::{thread, time::Duration};

use clap::Parser;
use anyhow::{Result, Error};
use console::style;
use dialoguer::Password;
use diesel::SqliteConnection;
use indicatif::ProgressStyle;

use tufa_client::{crypto::AesData, db::{self, models::{self, service::Service}}, otp::Totp};
use tufa_common::util;

#[derive(Parser)]
#[command(infer_subcommands = true)]
pub struct Cmd {
    /// The service you'd like to generate a code for
    service: String,

    /// Simply print the code and nothing else. Useful for piping into 
    /// a clipboard utility like `xclip`.
    #[arg(short, long)]
    oneshot: bool,

}

impl Cmd {
    pub fn run(&self) -> Result<()>{
        let mut conn = db::establish_connection();

        let otp = generate_otp(&mut conn, &self.service);

        if self.oneshot {
            otp.and_then(|otp| -> Result<()> {
                println!("{}", otp.generate()?);
                Ok(())
            })
        } else {
                otp.map(|otp| render_progress_bar(&otp))
        }?;

        Ok(())
    }
}

fn get_service(conn: &mut SqliteConnection, name: &str) -> Option<Service> {
    models::service::get_by_name(conn, name)
}

fn generate_otp(conn: &mut SqliteConnection, service: &str) -> Result<Totp> {
    let service = get_service(conn, service)
        .ok_or(Error::msg("Could not find service."))?;

    let secret = if service.encrypted == 1 {
        let password = Password::new()
        .with_prompt("Please provide your password.")
        .interact()
        .map_err(|_| Error::msg("Failed to read password"))?;

        let aes_data: AesData = service.secret.parse()
            .map_err(|_| Error::msg("Malformed encrypted secret."))?;

        aes_data.decrypt(&password)
            .map_err(|_| Error::msg("Failed to decrypt secret. Did you mistype your password?"))?
    } else {
        service.secret
    };

    Ok(Totp::simple(&secret))
}

fn render_progress_bar(otp: &Totp) {
    use indicatif::ProgressBar;
    let bar = ProgressBar::new(otp.window);
    bar.set_style(ProgressStyle::with_template("{msg} {bar:40.blue/cyan}")
        .unwrap());


    for _ in 0.. {
        let now = util::now();
        let remaining = otp.window - (now - otp.reference_time) % otp.window;
        bar.set_position(remaining);
        if let Ok(code) = otp.generate() {
            bar.set_message(format!("🔑 Your code is {}  ", style(code).blue()));
        }
        thread::sleep(Duration::from_secs(1));
    }

    bar.finish();
}
