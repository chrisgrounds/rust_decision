pub use crate::probability::{OutOfBounds, Probabilities, Probability};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
  pub answers: HashMap<String, Answer>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Answer {
  #[serde(rename = "choice")]
  ChoiceAnswer {
    choice: Choice,
    probabilities: Probabilities,
    confidence: Probability,
  },
  #[serde(rename = "score")]
  ScoreAnswer {
    score: Score,
    probabilities: Probabilities,
    confidence: Probability,
    legend: Legend,
  },
  #[serde(rename = "noul")]
  NoulAnswer { noul: Probability },
}

#[derive(Clone, Debug, Deserialize)]
pub struct Choice(pub String);

#[derive(Clone, Debug, Deserialize)]
pub struct Score(pub f64);

pub type Legend = HashMap<String, String>;
