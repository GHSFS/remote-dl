//! HTTP client for the remote-dl edge worker.

use crate::config::Config;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const USER_AGENT: &str = concat!("rdl/", env!("CARGO_PKG_VERSION"));

pub struct Client {
    http: reqwest::blocking::Client,
    base: String,
    token: String,
}

impl Client {
    pub fn new(cfg: &Config) -> Result<Self> {
        let http = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(30))
            .https_only(true)
            .build()
            .map_err(|e| Error::Http(e.to_string()))?;

        Ok(Self {
            http,
            base: cfg.worker()?.to_string(),
            token: cfg.token()?,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base, path)
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub fn ping(&self) -> Result<()> {
        let resp = self
            .http
            .get(self.url("/api/ping"))
            .header("Authorization", self.auth_header())
            .send()
            .map_err(|e| Error::Http(e.to_string()))?;
        check_status(&resp)?;
        Ok(())
    }

    pub fn queue_download(
        &self,
        url: &str,
        name: Option<&str>,
        folder: Option<&str>,
    ) -> Result<JobRef> {
        let body = QueueRequest { url, name, folder };
        let resp = self
            .http
            .post(self.url("/api/dl"))
            .header("Authorization", self.auth_header())
            .json(&body)
            .send()
            .map_err(|e| Error::Http(e.to_string()))?;
        check_status(&resp)?;
        let job: JobRef = resp.json().map_err(|e| Error::Http(e.to_string()))?;
        Ok(job)
    }

    pub fn list_runs(&self, limit: usize) -> Result<Vec<RunInfo>> {
        let resp = self
            .http
            .get(self.url(&format!("/api/runs?limit={limit}")))
            .header("Authorization", self.auth_header())
            .send()
            .map_err(|e| Error::Http(e.to_string()))?;
        check_status(&resp)?;
        let envelope: RunListResponse =
            resp.json().map_err(|e| Error::Http(e.to_string()))?;
        Ok(envelope.runs)
    }

    pub fn job_status(&self, id: &str) -> Result<RunInfo> {
        let resp = self
            .http
            .get(self.url(&format!("/api/runs/{id}")))
            .header("Authorization", self.auth_header())
            .send()
            .map_err(|e| Error::Http(e.to_string()))?;
        check_status(&resp)?;
        let info: RunInfo = resp.json().map_err(|e| Error::Http(e.to_string()))?;
        Ok(info)
    }
}

fn check_status(resp: &reqwest::blocking::Response) -> Result<()> {
    let status = resp.status();
    if status.is_success() {
        return Ok(());
    }
    Err(match status.as_u16() {
        401 | 403 => Error::Auth(format!("server rejected credentials ({status})")),
        404 => Error::NotFound,
        500..=599 => Error::Http(format!("server error: {status}")),
        _ => Error::Http(format!("unexpected status: {status}")),
    })
}

#[derive(Debug, Serialize)]
struct QueueRequest<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct JobRef {
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct RunListResponse {
    runs: Vec<RunInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RunInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(default)]
    pub html_url: Option<String>,
}
