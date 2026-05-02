//! Command-line interface definitions.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "rdl",
    version,
    about = "Stream-to-cloud download manager — desktop client",
    long_about = "rdl queues URLs to your remote-dl backend and reports their status. \
                  See https://github.com/GHSFS/remote-dl for setup."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Queue a URL for download.
    #[command(visible_alias = "g")]
    Get {
        /// Source URL.
        url: String,

        /// Save as filename. Defaults to the URL basename.
        #[arg(short, long)]
        name: Option<String>,

        /// Destination subfolder under the configured root.
        #[arg(short, long)]
        folder: Option<String>,
    },

    /// Show recent downloads.
    #[command(visible_alias = "ls")]
    List {
        /// Maximum number of entries to show.
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },

    /// Show status of a single job (latest if id omitted).
    #[command(visible_alias = "st")]
    Status {
        /// Job id. If omitted, shows the most recent job.
        id: Option<String>,
    },

    /// Get or set local client configuration.
    Config {
        #[command(subcommand)]
        action: ConfigCmd,
    },

    /// Manage authentication.
    Auth {
        #[command(subcommand)]
        action: AuthCmd,
    },

    /// Run in clipboard-watch mode (delegates to rdl-tray).
    Watch,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCmd {
    /// Read a config value.
    Get {
        /// Config key (worker | token | folder).
        key: String,
    },
    /// Set a config value.
    Set {
        /// Config key (worker | token | folder).
        key: String,
        /// New value.
        value: String,
    },
    /// Print the path to the active config file.
    Path,
}

#[derive(Subcommand, Debug)]
pub enum AuthCmd {
    /// Authenticate with a permanent token (obtain one from the bot first).
    Login {
        /// Permanent token.
        #[arg(short, long)]
        token: String,
    },
    /// Clear local credentials.
    Logout,
}
