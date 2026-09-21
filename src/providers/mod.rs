use std::{collections::HashMap, error::Error, fmt};

use crate::{answer::Response, client::Client, question::Question};

pub mod typesafe;

pub trait Provider {
  fn post(
    &self,
    client: &Client,
    state: String,
    questions: HashMap<String, Question>,
  ) -> impl std::future::Future<Output = Result<Response, ProviderError>> + Send;
}

#[derive(Debug)]
pub enum ProviderError {
  HttpError(reqwest::Error),
  SerdeError(serde_json::Error),
}

impl fmt::Display for ProviderError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::HttpError(error) => write!(f, "HTTP request failed: {error}"),
      Self::SerdeError(error) => write!(f, "JSON de/serialization failed: {error}"),
    }
  }
}

impl Error for ProviderError {}
