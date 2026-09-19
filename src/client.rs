pub struct Client {
  api_key: ApiKey,
}

#[derive(Clone, Debug)]
pub struct ApiKey(String);

impl Client {
  pub fn new(api_key: String) -> Self {
    Self {
      api_key: ApiKey(api_key),
    }
  }
}
