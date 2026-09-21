use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
pub struct State(String);

#[derive(Clone, Debug, Serialize)]
pub struct Model(String);

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
/// Each envelope holds one question type; score and noul questions need no choice type.
pub struct Envelope<Q> {
  pub state: State,
  pub model: Model,
  pub questions: HashMap<String, Q>,
}

impl<Q> Envelope<Q> {
  pub fn new(state: String, model: String, questions: HashMap<String, Q>) -> Self {
    Self {
      state: State(state),
      model: Model(model),
      questions,
    }
  }
}
