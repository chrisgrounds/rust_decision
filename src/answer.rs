pub use crate::probability::{OutOfBounds, Probabilities, Probability};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Answer {
  ChoiceAnswer {
    choice: SelectedOption,
    probabilities: Probabilities,
    confidence: Probability,
  },
  ScoreAnswer {
    score: SelectedOption,
    probabilities: Probabilities,
    confidence: Probability,
    legend: Legend,
  },
  NoulAnswer {
    noul: Probability,
  },
}

#[derive(Clone, Debug, Deserialize)]
pub struct SelectedOption;

#[derive(Clone, Debug, Deserialize)]
pub struct Legend;
