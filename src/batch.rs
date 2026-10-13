//! Batch file parsing and sequential URL enqueuer.

use crate::api::Client;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemResult {
    pub url: String,
    pub job_id: Option<String>,
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSummary {
    pub total: usize,
    pub queued: usize,
    pub failed: usize,
    pub items: Vec<BatchItemResult>,
}

pub fn parse_batch_urls<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let content = fs::read_to_string(path).map_err(Error::Io)?;
    let mut urls = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            urls.push(trimmed.to_string());
        }
    }

    Ok(urls)
}

pub fn execute_batch(client: &Client, urls: &[String], folder: Option<&str>) -> BatchSummary {
    let mut items = Vec::new();
    let mut queued = 0;
    let mut failed = 0;

    for url in urls {
        match client.queue_download(url, None, folder) {
            Ok(job) => {
                queued += 1;
                items.push(BatchItemResult {
                    url: url.clone(),
                    job_id: Some(job.id),
                    status: "queued".to_string(),
                    error: None,
                });
            }
            Err(e) => {
                failed += 1;
                items.push(BatchItemResult {
                    url: url.clone(),
                    job_id: None,
                    status: "failed".to_string(),
                    error: Some(e.to_string()),
                });
            }
        }
    }

    BatchSummary {
        total: urls.len(),
        queued,
        failed,
        items,
    }
}
