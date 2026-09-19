use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
pub enum Question<T> {
  ChoiceQuestion {
    instructions: Instructions,
    choices: T,
  },
  ScoreQuestion {
    instructions: Instructions,
  },
  NoulQuestion {
    instructions: Instructions,
  },
}

#[derive(Clone, Debug, Serialize)]
pub struct Instructions(String);

#[derive(Clone, Debug, Serialize)]
pub struct State;

/// Example
/// {
///   "state": "Help! My payouts have been failing for 3 days.",
///   "model": "jev-latest",
///   "questions": {
///     "is_urgent": {
///       "type": "noul",
///       "instructions": "Does this convey urgency?"
///     }
///   }
/// }
pub struct Envelope<T> {
  pub state: State,
  pub model: String,
  pub questions: HashMap<String, Question<T>>,
}
