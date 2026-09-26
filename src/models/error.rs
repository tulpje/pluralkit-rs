use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

#[derive(Debug)]
pub enum PluralKitError {
    PluralKit {
        code: u16,
        status: u16,
        error: ErrorResponse,
    },
    Http {
        status: u16,
        text: Option<String>,
    },
    Reqwest(reqwest::Error),
    Other(crate::Error),
}

impl std::error::Error for PluralKitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PluralKit { .. } | Self::Http { .. } => None,
            Self::Reqwest(err) => Some(err),
            Self::Other(err) => Some(err.as_ref()),
        }
    }
}

impl Display for PluralKitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PluralKit { error, .. } => write!(f, "PluralKit Error: {error}"),
            Self::Http { status, .. } => write!(f, "HTTP Error: {status}"),
            Self::Reqwest(err) => err.fmt(f),
            Self::Other(err) => err.fmt(f),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub errors: Option<HashMap<String, ErrorItem>>,
    pub retry_after: Option<u32>,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.code)?;
        if let Some(ref errors) = self.errors
            && !errors.is_empty()
        {
            f.write_str(": ")?;
            f.write_str(
                &errors
                    .iter()
                    .map(|(field, error)| format!("{field}: {error}"))
                    .collect::<Vec<_>>()
                    .join(", "),
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorItem {
    pub message: String,
    pub max_length: Option<u16>,
    pub actual_length: Option<u16>,
}

impl Display for ErrorItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
