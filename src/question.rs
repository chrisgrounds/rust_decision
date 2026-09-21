use serde::Serialize;
use std::collections::HashMap;

pub type Choices = HashMap<String, String>;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Question {
  Choice(ChoiceQuestion),
  Score(ScoreQuestion),
  Noul(NoulQuestion),
}

#[derive(Clone, Debug, Serialize)]
pub struct ChoiceQuestion {
  pub instructions: Instructions,
  #[serde(rename = "criteria")]
  pub choices: Choices,
}

impl ChoiceQuestion {
  pub fn new<K, V>(instructions: String, choices: impl IntoIterator<Item = (K, V)>) -> Self
  where
    K: Into<String>,
    V: Into<String>,
  {
    Self {
      instructions: Instructions(instructions),
      choices: choices
        .into_iter()
        .map(|(key, value)| (key.into(), value.into()))
        .collect(),
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct ScoreQuestion {
  pub instructions: Instructions,
  pub criteria: Vec<String>,
}

impl ScoreQuestion {
  pub fn new(instructions: String, criteria: Vec<String>) -> Self {
    Self {
      instructions: Instructions(instructions),
      criteria,
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct NoulQuestion {
  pub instructions: Instructions,
}

impl NoulQuestion {
  pub fn new(instructions: String) -> Self {
    Self {
      instructions: Instructions(instructions),
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct Instructions(String);

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn serializes_mixed_questions_in_api_format() {
    let questions = HashMap::from([
      (
        "department",
        Question::Choice(ChoiceQuestion::new(
          "Which team should handle this?".to_owned(),
          HashMap::from([("billing", "Payments"), ("technical", "Outages")]),
        )),
      ),
      (
        "severity",
        Question::Score(ScoreQuestion::new(
          "How severe is this?".to_owned(),
          vec!["Minor".to_owned(), "Critical".to_owned()],
        )),
      ),
      (
        "urgent",
        Question::Noul(NoulQuestion::new("Is this urgent?".to_owned())),
      ),
    ]);

    assert_eq!(
      serde_json::to_value(questions).unwrap(),
      serde_json::json!({
        "department": {
          "type": "choice",
          "instructions": "Which team should handle this?",
          "criteria": {"billing": "Payments", "technical": "Outages"}
        },
        "severity": {
          "type": "score",
          "instructions": "How severe is this?",
          "criteria": ["Minor", "Critical"]
        },
        "urgent": {
          "type": "noul",
          "instructions": "Is this urgent?"
        }
      })
    );
  }
}
