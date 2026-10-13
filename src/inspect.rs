//! URL pre-flight inspection and HTTP stream header analyzer.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlInspection {
    pub url: String,
    pub final_url: String,
    pub status: u16,
    pub content_length: Option<u64>,
    pub human_size: Option<String>,
    pub content_type: Option<String>,
    pub accepts_ranges: bool,
    pub last_modified: Option<String>,
    pub suggested_filename: String,
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

pub fn extract_suggested_filename(url_str: &str, content_disp: Option<&str>) -> String {
    if let Some(disp) = content_disp {
        for part in disp.split(';') {
            let part = part.trim();
            if let Some(val) = part.strip_prefix("filename=") {
                let clean = val.trim_matches('"').trim_matches('\'').trim();
                if !clean.is_empty() {
                    return clean.to_string();
                }
            }
            if let Some(val) = part.strip_prefix("filename*=") {
                if let Some(actual) = val.split("''").nth(1) {
                    let clean = actual.trim_matches('"').trim();
                    if !clean.is_empty() {
                        return clean.to_string();
                    }
                }
            }
        }
    }

    if let Ok(parsed) = Url::parse(url_str) {
        if let Some(mut segments) = parsed.path_segments() {
            if let Some(last) = segments.next_back() {
                let clean = last.trim();
                if !clean.is_empty() && clean != "/" {
                    return clean.to_string();
                }
            }
        }
    }

    "downloaded_file".to_string()
}

pub fn inspect_remote_url(target_url: &str) -> Result<UrlInspection> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(concat!("rdl-inspect/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| Error::Http(e.to_string()))?;

    // 1. Try HEAD request first
    let head_resp = client.head(target_url).send();

    let resp = match head_resp {
        Ok(r) if r.status().is_success() || r.status().is_redirection() => r,
        _ => {
            // Fall back to GET with byte range 0-0
            client
                .get(target_url)
                .header("Range", "bytes=0-0")
                .send()
                .map_err(|e| Error::Http(format!("Failed to reach target URL: {e}")))?
        }
    };

    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let headers = resp.headers();

    let content_length = headers
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .or_else(|| {
            // Check Content-Range: bytes 0-0/123456
            headers
                .get(reqwest::header::CONTENT_RANGE)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.split('/').nth(1))
                .and_then(|v| v.parse::<u64>().ok())
        });

    let human_size = content_length.map(format_bytes);

    let content_type = headers
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let accepts_ranges = headers
        .get(reqwest::header::ACCEPT_RANGES)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("bytes"))
        .unwrap_or_else(|| status == 206);

    let last_modified = headers
        .get(reqwest::header::LAST_MODIFIED)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let content_disp = headers
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok());

    let suggested_filename = extract_suggested_filename(&final_url, content_disp);

    Ok(UrlInspection {
        url: target_url.to_string(),
        final_url,
        status,
        content_length,
        human_size,
        content_type,
        accepts_ranges,
        last_modified,
        suggested_filename,
    })
}
