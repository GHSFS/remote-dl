//! Persistent client configuration.
//!
//! Stored at `%APPDATA%\rdl\config.json`. Sensitive fields (currently `token`)
//! are wrapped with the Windows DPAPI so they can only be decrypted by the same
//! Windows user account that wrote them.

use crate::error::{Error, Result};
use base64::engine::general_purpose::STANDARD as B64;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Base URL of the deployed edge worker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<String>,

    /// DPAPI-wrapped permanent token (base64-encoded ciphertext).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_enc: Option<String>,

    /// Default destination folder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("dev", "GHSFS", "rdl")
            .ok_or_else(|| Error::Config("could not resolve config directory".into()))?;
        Ok(dirs.config_dir().join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&raw)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let raw = serde_json::to_string_pretty(self)?;
        fs::write(&path, raw)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<String> {
        match key {
            "worker" => self.worker.clone(),
            "folder" => self.folder.clone(),
            "token" => self
                .token_enc
                .as_ref()
                .map(|enc| dpapi::unprotect(enc).unwrap_or_else(|_| "<corrupted>".into())),
            other => return Err(Error::Config(format!("unknown key: {other}"))),
        }
        .ok_or_else(|| Error::Config(format!("{key} is not set")))
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "worker" => self.worker = Some(value.trim_end_matches('/').to_string()),
            "folder" => self.folder = Some(value.to_string()),
            "token" => self.token_enc = Some(dpapi::protect(value)?),
            other => return Err(Error::Config(format!("unknown key: {other}"))),
        }
        Ok(())
    }

    pub fn clear(&mut self, key: &str) {
        match key {
            "worker" => self.worker = None,
            "folder" => self.folder = None,
            "token" => self.token_enc = None,
            _ => {}
        }
    }

    pub fn token(&self) -> Result<String> {
        self.token_enc
            .as_ref()
            .ok_or_else(|| Error::Config("not authenticated — run `rdl auth login`".into()))
            .and_then(|enc| dpapi::unprotect(enc))
    }

    pub fn worker(&self) -> Result<&str> {
        self.worker
            .as_deref()
            .ok_or_else(|| Error::Config("worker URL not set — run `rdl config set worker …`".into()))
    }
}

#[cfg(windows)]
mod dpapi {
    use super::{Error, Result, B64};
    use base64::Engine as _;
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Security::Cryptography::{
        CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB,
    };

    pub fn protect(plain: &str) -> Result<String> {
        let mut input = CRYPT_INTEGER_BLOB {
            cbData: plain.len() as u32,
            pbData: plain.as_ptr() as *mut u8,
        };
        let mut output = CRYPT_INTEGER_BLOB::default();

        unsafe {
            CryptProtectData(
                &mut input,
                None,
                None,
                None,
                None,
                0,
                &mut output,
            )
            .map_err(|e| Error::Config(format!("DPAPI protect failed: {e}")))?;

            let bytes = std::slice::from_raw_parts(output.pbData, output.cbData as usize);
            let encoded = B64.encode(bytes);
            let _ = LocalFree(windows::Win32::Foundation::HLOCAL(output.pbData as _));
            Ok(encoded)
        }
    }

    pub fn unprotect(encoded: &str) -> Result<String> {
        let mut bytes = B64
            .decode(encoded)
            .map_err(|e| Error::Config(format!("invalid token encoding: {e}")))?;
        let mut input = CRYPT_INTEGER_BLOB {
            cbData: bytes.len() as u32,
            pbData: bytes.as_mut_ptr(),
        };
        let mut output = CRYPT_INTEGER_BLOB::default();

        unsafe {
            CryptUnprotectData(
                &mut input,
                None,
                None,
                None,
                None,
                0,
                &mut output,
            )
            .map_err(|e| Error::Config(format!("DPAPI unprotect failed: {e}")))?;

            let plain = std::slice::from_raw_parts(output.pbData, output.cbData as usize);
            let s = String::from_utf8_lossy(plain).into_owned();
            let _ = LocalFree(windows::Win32::Foundation::HLOCAL(output.pbData as _));
            Ok(s)
        }
    }
}

#[cfg(not(windows))]
mod dpapi {
    //! Non-Windows fallback used only for builds on the test runners.
    //! The shipped binary is Windows-only, so this path is never used in
    //! production.
    use super::{Error, Result, B64};
    use base64::Engine as _;
    pub fn protect(plain: &str) -> Result<String> {
        Ok(B64.encode(plain.as_bytes()))
    }
    pub fn unprotect(encoded: &str) -> Result<String> {
        let bytes = B64
            .decode(encoded)
            .map_err(|e| Error::Config(format!("invalid token encoding: {e}")))?;
        String::from_utf8(bytes).map_err(|e| Error::Config(format!("non-utf8 token: {e}")))
    }
}
