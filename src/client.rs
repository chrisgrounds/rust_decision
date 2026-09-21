pub struct Client {
  pub(crate) decision_api_key: DecisionApiKey,
  pub http_client: reqwest::Client,
}

#[derive(Clone, Debug)]
pub struct DecisionApiKey(pub(crate) String);

impl Client {
  pub fn from_env() -> Result<Self, std::env::VarError> {
    let decision_api_key = std::env::var("DECISION_API_KEY")?;

    Ok(Self::new(decision_api_key))
  }

  pub fn new(decision_api_key: String) -> Self {
    Self {
      decision_api_key: DecisionApiKey(decision_api_key),
      http_client: reqwest::Client::new(),
    }
  }
}
