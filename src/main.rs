//! `rdl` — desktop CLI client for remote-dl.
//!
//! See `rdl --help` for usage.

mod api;
mod cli;
mod config;
mod error;

use clap::Parser;
use cli::{AuthCmd, Cli, Command, ConfigCmd};
use colored::Colorize;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Get { url, name, folder } => cmd_get(url, name, folder),
        Command::List { limit } => cmd_list(limit),
        Command::Status { id } => cmd_status(id),
        Command::Config { action } => cmd_config(action),
        Command::Auth { action } => cmd_auth(action),
        Command::Watch => cmd_watch(),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{} {err:#}", "error:".red().bold());
            ExitCode::FAILURE
        }
    }
}

fn cmd_get(url: String, name: Option<String>, folder: Option<String>) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let client = api::Client::new(&cfg)?;

    println!("{} {}", "→".cyan(), url.dimmed());
    let job = client.queue_download(&url, name.as_deref(), folder.as_deref())?;
    println!(
        "{} queued — job {}",
        "✓".green(),
        job.id.bright_white().bold()
    );
    Ok(())
}

fn cmd_list(limit: usize) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let client = api::Client::new(&cfg)?;

    let runs = client.list_runs(limit)?;
    if runs.is_empty() {
        println!("{}", "no recent downloads".dimmed());
        return Ok(());
    }

    for run in runs {
        let status = match run.status.as_str() {
            "completed" => "✓".green(),
            "in_progress" | "queued" => "⋯".yellow(),
            "failure" | "cancelled" => "✗".red(),
            _ => "?".dimmed(),
        };
        println!(
            "{}  {:<10}  {:>8}  {}",
            status,
            run.status,
            run.id.dimmed(),
            run.name
        );
    }
    Ok(())
}

fn cmd_status(id: Option<String>) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let client = api::Client::new(&cfg)?;

    let id = match id {
        Some(id) => id,
        None => {
            let runs = client.list_runs(1)?;
            runs.into_iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("no jobs found"))?
                .id
        }
    };

    let status = client.job_status(&id)?;
    println!("{:<10} {}", "id:".dimmed(), status.id);
    println!("{:<10} {}", "name:".dimmed(), status.name);
    println!("{:<10} {}", "status:".dimmed(), status.status);
    if let Some(url) = status.html_url {
        println!("{:<10} {}", "url:".dimmed(), url);
    }
    Ok(())
}

fn cmd_config(action: ConfigCmd) -> anyhow::Result<()> {
    match action {
        ConfigCmd::Get { key } => {
            let cfg = config::Config::load()?;
            let value = cfg.get(&key)?;
            println!("{value}");
        }
        ConfigCmd::Set { key, value } => {
            let mut cfg = config::Config::load().unwrap_or_default();
            cfg.set(&key, &value)?;
            cfg.save()?;
            println!("{} {key} = {}", "✓".green(), redact(&key, &value).dimmed());
        }
        ConfigCmd::Path => {
            println!("{}", config::Config::path()?.display());
        }
    }
    Ok(())
}

fn cmd_auth(action: AuthCmd) -> anyhow::Result<()> {
    match action {
        AuthCmd::Login { token } => {
            let mut cfg = config::Config::load().unwrap_or_default();
            cfg.set("token", &token)?;
            cfg.save()?;

            let client = api::Client::new(&cfg)?;
            client.ping()?;
            println!("{} authenticated", "✓".green());
        }
        AuthCmd::Logout => {
            let mut cfg = config::Config::load().unwrap_or_default();
            cfg.clear("token");
            cfg.save()?;
            println!("{} local credentials cleared", "✓".green());
        }
    }
    Ok(())
}

fn cmd_watch() -> anyhow::Result<()> {
    eprintln!(
        "{} clipboard watch is provided by the rdl-tray companion",
        "note:".yellow()
    );
    eprintln!("      see: https://github.com/GHSFS/remote-dl#installation");
    Ok(())
}

fn redact(key: &str, value: &str) -> String {
    if matches!(key, "token") && value.len() > 8 {
        format!("{}…{}", &value[..4], &value[value.len() - 2..])
    } else {
        value.to_string()
    }
}
