use std::collections::HashMap;

use serde::Serialize;

use crate::{answer::Response, client::Client, question::Question};

pub mod typesafe;

pub trait Provider {
  fn post<T: Serialize + Send>(
    &self,
    client: Client,
    state: String,
    questions: HashMap<String, Question<T>>,
  ) -> impl std::future::Future<Output = Result<Response, ProviderError>> + Send;
}

#[derive(Debug)]
pub enum ProviderError {
  HttpError(reqwest::Error),
  SerdeError(serde_json::Error),
}
