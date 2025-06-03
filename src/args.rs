use clap::{Args, Parser, Subcommand};

/// Voice of the LAN - A powerful LAN proxy that speaks your network's language.
/// It is a Rust-based command-line and proxy server tool that scans local TCP ports to find open services,
/// then starts a proxy server forwarding requests to the first detected open port.
/// It provides real-time feedback via terminal animations and supports forwarding HTTP requests using Actix Web and Reqwest.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct VoxlanArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Run a proxy server
    Run(RunArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Specify the target port to forward traffic to.
    #[arg(short, long)]
    pub port: Option<u16>,
}
