use std::collections::HashMap;

use crate::{
  answer::Response,
  client::Client,
  envelope::Envelope,
  providers::{Provider, ProviderError},
  question::Question,
};

pub struct TypesafeJev;

const API_URL: &str = "https://api.typesafe.ai/v1/systemone";

impl Provider for TypesafeJev {
  async fn decide(
    &self,
    client: &Client,
    state: String,
    questions: HashMap<String, Question>,
  ) -> Result<Response, ProviderError> {
    let envelope = Envelope::new(state, "jev-latest".to_owned(), questions);
    let body = serde_json::to_string(&envelope).map_err(ProviderError::SerdeError)?;

    let response = client
      .http_client
      .post(API_URL)
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .body(body)
      .bearer_auth(&client.decision_api_key.0)
      .send()
      .await
      .map_err(ProviderError::HttpError)?
      .error_for_status()
      .map_err(ProviderError::HttpError)?
      .text()
      .await
      .map_err(ProviderError::HttpError)?;

    serde_json::from_str(&response).map_err(ProviderError::SerdeError)
  }
}
