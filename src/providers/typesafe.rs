use std::collections::HashMap;

use serde::Serialize;

use crate::{
  answer::Response,
  client::Client,
  providers::{Provider, ProviderError},
  question::Question,
};

pub struct TypesafeJev;

impl Provider for TypesafeJev {
  async fn post<T: Serialize + Send>(
    &self,
    client: Client,
    state: String,
    questions: HashMap<String, Question<T>>,
  ) -> Result<Response, ProviderError> {
    let url = "https://api.typesafe.ai/v1/systemone";
    let response = client
      .http_client
      .post(url)
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .body(
        serde_json::json!({
          "model": "jev-latest",
          "state": state,
          "questions": questions
        })
        .to_string(),
      )
      .bearer_auth(&client.decision_api_key.0)
      .send()
      .await
      .map_err(ProviderError::HttpError)?
      .error_for_status()
      .map_err(ProviderError::HttpError)?;

    let body = response
      .text()
      .await
      .map_err(|e| ProviderError::HttpError(e))?;

    let response: Response = serde_json::from_str(&body).map_err(ProviderError::SerdeError)?;

    Ok(response)
  }
}
