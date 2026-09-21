use crate::{
  answer::Answer,
  client::Client,
  providers::{Provider, ProviderError},
};

pub struct TypesafeJev;

impl Provider for TypesafeJev {
  async fn post(&self, client: Client) -> Result<Answer, ProviderError> {
    let url = "";
    let response = client
      .http_client
      .post(url)
      .send()
      .await
      .map_err(|e| ProviderError::HttpError(e))?;

    let body = response
      .text()
      .await
      .map_err(|e| ProviderError::HttpError(e))?;

    let answer: Answer = serde_json::from_str(&body).map_err(|e| ProviderError::SerdeError(e))?;

    Ok(answer)
  }
}
