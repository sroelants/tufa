use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a 2FA service
    Add {
        /// The name of the service you'd like to add 2FA for
        service: String,

        /// The 2FA secret provided by the service
        secret: String,
    },

    // Remove a service
    Rm {
        /// The service you'd like to remove
        service: String,
    },

    /// Generate a new 2FA code
    Gen {
        /// The service you'd like to generate a code for
        service: String,
    },

    // List all the registered services
    Ls,
}
