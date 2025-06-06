use clap::{Args, ColorChoice, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = r#"
 Voice of the LAN 
 - A powerful Tool that scans open TCP ports locally and runs a proxy server forwarding requests to the detected service.
    "#,
    long_about = r#"
 Voice of the LAN - A powerful LAN proxy that speaks your network's language.
 It is a Rust-based command-line and proxy server tool that scans local TCP ports to find open services,
 then starts a proxy server forwarding requests to the first detected open port.
 It provides real-time feedback via terminal animations and supports forwarding HTTP requests using Actix Web and Reqwest.
"#
)]
#[clap(color = ColorChoice::Always)]
#[clap(styles = get_styles())]
pub struct VoxlanArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Run a proxy server
    Run(RunArgs),

    ///List all the open ports
    List,

    ///Connect to the running server and send messages interactively via HTTP POST requests
    Client(ClientArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Specify the target port to forward traffic to, port to bind the proxy server
    #[arg(short, long)]
    pub bind_port: Option<u16>,
}

#[derive(Args, Debug)]
pub struct ClientArgs {
    /// HTTP endpoint path (client mode only)
    #[arg(long, short, default_value = "/")]
    pub path: String,

    /// Specify the target port to forward traffic to, port to bind the proxy server
    #[arg(short, long)]
    pub bind_port: Option<u16>,
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
        )
        .usage(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
}
