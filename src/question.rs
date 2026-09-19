use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Question<T> {
  ChoiceQuestion { context: Context },
  ScoreQuestion { context: Context },
  NoulQuestion { context: Context },
}

#[derive(Clone, Debug, Serialize)]
pub struct Context;
