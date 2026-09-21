use crate::{answer::Answer, client::Client};

pub mod typesafe;

trait Provider {
  async fn post(&self, client: Client) -> Result<Answer, ProviderError>;
}

#[derive(Debug, Clone)]
pub enum Providers {
  TypesafeJev,
}

#[derive(Debug)]
pub enum ProviderError {
  HttpError(reqwest::Error),
  SerdeError(serde_json::Error),
}
