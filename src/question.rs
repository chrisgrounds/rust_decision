use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ChoiceQuestion<T> {
  pub instructions: Instructions,
  pub choices: T,
}

impl<T> ChoiceQuestion<T> {
  pub fn new(instructions: String, choices: T) -> Self {
    Self {
      instructions: Instructions(instructions),
      choices,
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct ScoreQuestion {
  pub instructions: Instructions,
}

impl ScoreQuestion {
  pub fn new(instructions: String) -> Self {
    Self {
      instructions: Instructions(instructions),
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
