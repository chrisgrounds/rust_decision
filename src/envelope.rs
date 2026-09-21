use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
struct State(String);

#[derive(Clone, Debug, Serialize)]
struct Model(String);

#[derive(Serialize)]
pub(crate) struct Envelope<Q> {
  state: State,
  model: Model,
  questions: HashMap<String, Q>,
}

impl<Q> Envelope<Q> {
  pub(crate) fn new(state: String, model: String, questions: HashMap<String, Q>) -> Self {
    Self {
      state: State(state),
      model: Model(model),
      questions,
    }
  }
}
