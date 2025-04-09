use clap::{Parser, Subcommand};

/// Command line arguments for the tunnel application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Run as a client
    Client {
        /// Server IP address to connect to
        #[arg(short, long)]
        server_ip: String,

        /// Port to connect to
        #[arg(short, long)]
        port: u16,
    },
    /// Run as a server
    Server {
        /// IP address to bind the server
        #[arg(short, long)]
        bind_ip: String,

        /// Port to run the server on
        #[arg(short, long)]
        port: u16,
    },
}