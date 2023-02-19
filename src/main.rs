mod otp;
mod db;
mod crypto;
mod cli;
mod util;

use clap::Parser;
use cli::{Cli, Commands};
use crypto::AesData;
use diesel::SqliteConnection;
use db::{models, models::service::Service};
use indicatif::ProgressStyle;
use otp::Totp;
use std::thread;
use std::time::Duration;
use console::style;
use anyhow::Error;
use dialoguer::{Confirm, Password};

fn main() {
    let cli = Cli::parse();
    let mut connection = db::establish_connection();

    let result = match &cli.command {
        Some(Commands::Add { service, secret, encrypted }) => { 
            add_service(&mut connection, service, secret, *encrypted)
        }

        Some(Commands::Gen { service, oneshot }) => { 
            let otp = generate_otp(&mut connection, service);

            if *oneshot {
                otp.and_then(|otp| -> Result<(), Error> {
                    println!("{}", otp.generate()?);
                    Ok(())
                })
            } else {
                    otp.map(|otp| render_progress_bar(&otp))
            }
        }

        Some(Commands::Rm { service }) => {
            remove_service(&mut connection, service)
        }

        Some(Commands::Ls) => {
            list_services(&mut connection)
        }

        _ => {Ok(())}
    };

    match result {
        Err(error) => println!("💥 Error: {}", error),
        Ok(_) => {}
    }
}

fn add_service(
    conn: &mut SqliteConnection, 
    service: &str, 
    secret: &str, 
    encrypted: bool
) -> Result<(), Error> {
    if encrypted {
        let password = Password::new()
        .with_prompt("Please provide a password to encrypt the secret.")
        .interact()?;

        let secret: String = AesData::encrypt(secret, &password)?.into();
        models::service::create(conn, service, &secret, true)?;
    } else {
        models::service::create(conn, service, &secret, false)?;
    }

    Ok(())
}

fn generate_otp(conn: &mut SqliteConnection, service: &str) -> Result<Totp, Error> {
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

fn remove_service(conn: &mut SqliteConnection, name: &str) -> Result<(), Error> {
    models::service::get_by_name(conn, name)
        .ok_or(Error::msg(format!("Service {} not found.", style(name).blue())))?;

    let confirmed = Confirm::new()
    .with_prompt(
        format!("{} {}?", 
            style("⚠️ Are you sure you want to remove 2FA for").red(), 
            style(name).blue().bold()
        )
    )
    .interact()
    .is_ok();

    if confirmed {
        models::service::remove(conn, name)?;
    }

    Ok(())
}

fn get_service(conn: &mut SqliteConnection, name: &str) -> Option<Service> {
    models::service::get_by_name(conn, name)
}

fn list_services(conn: &mut SqliteConnection) -> Result<(), Error>{
    models::service::get_all(conn)?.into_iter()
        .map(|service| service.name)
        .for_each(|name| println!("{}", name));

    Ok(())
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
