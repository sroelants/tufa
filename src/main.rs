mod otp;
mod db;
mod crypto;
mod cli;
mod util;

use clap::Parser;
use cli::{Cli, Commands};
use diesel::SqliteConnection;
use db::{models, models::service::Service};
use indicatif::ProgressStyle;
use otp::Totp;
use std::thread;
use std::time::Duration;
use console::style;

fn main() {
    let cli = Cli::parse();
    let mut connection = db::establish_connection();

    match &cli.command {
        Some(Commands::Add { service, secret }) => { 
            add_service(&mut connection, service, secret);
        }

        Some(Commands::Gen { service }) => { 
            if let Some(service) = get_service(&mut connection, service) {
                let otp = Totp::simple(&service.secret);
                render_progress_bar(&otp);
            }
        }

        Some(Commands::Remove { service }) => {
            remove_service(&mut connection, service);
        }

        _ => {}
    }
}

fn add_service(conn: &mut SqliteConnection, name: &str, secret: &str) -> Service {
   models::service::create(conn, name, secret)
}

fn remove_service(conn: &mut SqliteConnection, name: &str) {
    models::service::remove(conn, name);
}

fn get_service(conn: &mut SqliteConnection, name: &str) -> Option<Service> {
    models::service::get_by_name(conn, name)
}

fn render_progress_bar(otp: &Totp)  {
    use indicatif::ProgressBar;
    let bar = ProgressBar::new(otp.window);
    bar.set_style(ProgressStyle::with_template("{msg} {bar:40.cyan/blue}")
        .unwrap());

    for _ in 0.. {
        let now = util::now();
        let remaining = otp.window - (now - otp.reference_time) % otp.window;
        bar.set_position(remaining);
        bar.set_message(format!("Your code is {}", style(otp.generate()).cyan()));
        thread::sleep(Duration::from_secs(1));
    }

    bar.finish();
}
