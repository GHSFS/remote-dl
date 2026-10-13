//! `rdl` — desktop CLI client for remote-dl.
//!
//! See `rdl --help` for usage.

mod api;
mod batch;
mod cli;
mod config;
mod error;
mod inspect;

use clap::Parser;
use cli::{AuthCmd, Cli, Command, ConfigCmd};
use colored::Colorize;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Get {
            url,
            name,
            folder,
            json,
        } => cmd_get(url, name, folder, json),
        Command::List { limit, json } => cmd_list(limit, json),
        Command::Status { id, json } => cmd_status(id, json),
        Command::Inspect { url, json } => cmd_inspect(&url, json),
        Command::Batch { file, folder, json } => cmd_batch(file, folder, json),
        Command::Ping { json } => cmd_ping(json),
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

fn cmd_get(
    url: String,
    name: Option<String>,
    folder: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let client = api::Client::new(&cfg)?;

    if !json {
        println!("{} {}", "→".cyan(), url.dimmed());
    }
    let job = client.queue_download(&url, name.as_deref(), folder.as_deref())?;

    if json {
        let out = serde_json::json!({
            "status": "queued",
            "url": url,
            "job_id": job.id,
            "name": name,
            "folder": folder
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!(
            "{} queued — job {}",
            "✓".green(),
            job.id.bright_white().bold()
        );
    }
    Ok(())
}

fn cmd_list(limit: usize, json: bool) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let client = api::Client::new(&cfg)?;

    let runs = client.list_runs(limit)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&runs)?);
        return Ok(());
    }

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

fn cmd_status(id: Option<String>, json: bool) -> anyhow::Result<()> {
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
    if json {
        println!("{}", serde_json::to_string_pretty(&status)?);
        return Ok(());
    }

    println!("{:<10} {}", "id:".dimmed(), status.id);
    println!("{:<10} {}", "name:".dimmed(), status.name);
    println!("{:<10} {}", "status:".dimmed(), status.status);
    if let Some(url) = status.html_url {
        println!("{:<10} {}", "url:".dimmed(), url);
    }
    Ok(())
}

fn cmd_inspect(url: &str, json: bool) -> anyhow::Result<()> {
    let inspection = inspect::inspect_remote_url(url)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&inspection)?);
        return Ok(());
    }

    println!(
        "{} {}",
        "URL Inspection:".bright_white().bold(),
        inspection.url.dimmed()
    );
    println!("{:<18} {}", "Final URL:".dimmed(), inspection.final_url);
    println!(
        "{:<18} {}",
        "HTTP Status:".dimmed(),
        if inspection.status < 400 {
            inspection.status.to_string().green()
        } else {
            inspection.status.to_string().red()
        }
    );
    if let Some(ref human) = inspection.human_size {
        println!("{:<18} {}", "Content-Length:".dimmed(), human.bright_cyan());
    } else {
        println!(
            "{:<18} {}",
            "Content-Length:".dimmed(),
            "Unknown (chunked)".yellow()
        );
    }
    if let Some(ref ct) = inspection.content_type {
        println!("{:<18} {}", "Content-Type:".dimmed(), ct);
    }
    println!(
        "{:<18} {}",
        "Resumable/Ranges:".dimmed(),
        if inspection.accepts_ranges {
            "Yes (Accept-Ranges: bytes)".green()
        } else {
            "No (Streaming single-pass)".yellow()
        }
    );
    println!(
        "{:<18} {}",
        "Suggested Name:".dimmed(),
        inspection.suggested_filename.bright_white().bold()
    );

    Ok(())
}

fn cmd_batch(file: PathBuf, folder: Option<String>, json: bool) -> anyhow::Result<()> {
    let urls = batch::parse_batch_urls(&file)?;
    if urls.is_empty() {
        if json {
            let out = serde_json::json!({ "total": 0, "queued": 0, "failed": 0, "items": [] });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else {
            println!("{}", "No valid URLs found in file".yellow());
        }
        return Ok(());
    }

    let cfg = config::Config::load()?;
    let client = api::Client::new(&cfg)?;

    if !json {
        println!(
            "{} Enqueueing {} URL(s) from {}",
            "→".cyan(),
            urls.len().to_string().bold(),
            file.display()
        );
    }

    let summary = batch::execute_batch(&client, &urls, folder.as_deref());

    if json {
        println!("{}", serde_json::to_string_pretty(&summary)?);
        return Ok(());
    }

    println!(
        "\nBatch Summary: {} total | {} queued | {} failed",
        summary.total,
        summary.queued.to_string().green(),
        summary.failed.to_string().red()
    );
    for item in summary.items {
        if let Some(ref jid) = item.job_id {
            println!(
                "  {} {} -> job {}",
                "✓".green(),
                item.url.dimmed(),
                jid.bold()
            );
        } else {
            println!(
                "  {} {} ({})",
                "✗".red(),
                item.url.dimmed(),
                item.error.as_deref().unwrap_or("error")
            );
        }
    }

    Ok(())
}

fn cmd_ping(json: bool) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let client = api::Client::new(&cfg)?;
    client.ping()?;

    if json {
        let out = serde_json::json!({
            "status": "connected",
            "worker": cfg.worker,
            "authenticated": true
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!(
            "{} Connected and authenticated with edge worker",
            "✓".green()
        );
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
        ConfigCmd::Show { json } => {
            let cfg = config::Config::load().unwrap_or_default();
            if json {
                let out = serde_json::json!({
                    "config_path": config::Config::path()?.display().to_string(),
                    "worker": cfg.worker,
                    "folder": cfg.folder,
                    "has_token": cfg.token_enc.is_some()
                });
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                println!("{}", "rdl Configuration:".bright_white().bold());
                println!(
                    "{:<14} {}",
                    "Config file:".dimmed(),
                    config::Config::path()?.display()
                );
                println!(
                    "{:<14} {}",
                    "Worker URL:".dimmed(),
                    cfg.worker.as_deref().unwrap_or("(not set)")
                );
                println!(
                    "{:<14} {}",
                    "Default folder:".dimmed(),
                    cfg.folder.as_deref().unwrap_or("(not set)")
                );
                println!(
                    "{:<14} {}",
                    "Auth token:".dimmed(),
                    if cfg.token_enc.is_some() {
                        "Set (DPAPI-protected)".green()
                    } else {
                        "Not set".yellow()
                    }
                );
            }
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
