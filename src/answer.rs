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
    choice: SelectedOption,
    probabilities: Probabilities,
    confidence: Probability,
  },
  #[serde(rename = "score")]
  ScoreAnswer {
    score: SelectedOption,
    probabilities: Probabilities,
    confidence: Probability,
    legend: Legend,
  },
  #[serde(rename = "noul")]
  NoulAnswer { noul: Probability },
}

#[derive(Clone, Debug, Deserialize)]
pub struct SelectedOption;

#[derive(Clone, Debug, Deserialize)]
pub struct Legend;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserializes_noul_answers_from_response_wrapper() {
    let response: Response = serde_json::from_str(
      r#"{
        "model": "jev-latest",
        "answers": {
          "is_urgent": {"type": "noul", "noul": 0.95},
          "is_resolved": {"type": "noul", "noul": 0}
        },
        "usage": {"input_tokens": 100, "output_tokens": 20}
      }"#,
    )
    .unwrap();

    assert_eq!(response.answers.len(), 2);
    for (key, expected) in [("is_urgent", 0.95), ("is_resolved", 0.0)] {
      let Answer::NoulAnswer { noul } = &response.answers[key] else {
        panic!("expected a Noul answer");
      };
      assert_eq!(noul.0, expected);
    }
  }

  #[test]
  fn rejects_invalid_probabilities_in_response_wrapper() {
    for value in ["-1", "2", "1.000000001", "-0.01", "true", "\"0.95\""] {
      let body = format!(r#"{{"answers": {{"is_urgent": {{"type": "noul", "noul": {value}}}}}}}"#);
      assert!(serde_json::from_str::<Response>(&body).is_err());
    }
  }
}
