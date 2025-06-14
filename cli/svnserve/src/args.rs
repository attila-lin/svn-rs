use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about, max_term_width = 80)]
pub struct AppArgs {
    /// daemon mode
    #[arg(short, long)]
    pub daemon: bool,
    /// inetd mode
    #[arg(short, long)]
    pub inetd: bool,
    /// tunnel mode
    #[arg(short, long)]
    pub tunnel: bool,
    /// listen-once mode (useful for debugging)
    #[arg(short = 'X', long = "listen-once")]
    pub listen_once: bool,

    /// Windows service mode (Service Control Manager)
    #[cfg(target_os = "windows")]
    #[arg(long)]
    service: bool,

    /// root of directory to serve
    #[arg(long, short, value_name = "ROOT", default_value = ".")]
    root: String,

    /// force read only, overriding repository config file
    #[arg(long = "read-only", short = 'R')]
    read_only: bool,

    /// read configuration from file ARG
    #[arg(long, value_name = "ARG")]
    config_file: Option<String>,

    /// write server process ID to file ARG
    ///
    /// [mode: daemon, listen-once, service]
    #[arg(long, value_name = "ARG")]
    pid_file: Option<String>,
}
