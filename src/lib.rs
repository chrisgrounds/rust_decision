pub mod probability;

pub use probability::{OutOfBounds, Probabilities, Probability};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Question {
  ChoiceQuestion {
    choice: SelectedOption,
    probabilities: Probabilities,
    confidence: Probability,
  },
  ScoreQuestion {
    score: SelectedOption,
    probabilities: Probabilities,
    confidence: Probability,
    legend: Legend,
  },
  NoulQuestion {
    noul: Probability,
  },
}

#[derive(Clone, Debug, Deserialize)]
pub struct SelectedOption;

#[derive(Clone, Debug, Deserialize)]
pub struct Legend;
