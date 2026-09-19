use serde::Serialize;

pub type Questions<T> = Vec<Question<T>>;

#[derive(Clone, Debug, Serialize)]
pub enum Question<T> {
  ChoiceQuestion { context: Context },
  ScoreQuestion { context: Context },
  NoulQuestion { context: Context },
}

#[derive(Clone, Debug, Serialize)]
pub struct Context;

// {
//   "state": "Help! My payouts have been failing for 3 days.",
//   "model": "jev-latest",
//   "questions": {
//     "is_urgent": {
//       "type": "noul",
//       "instructions": "Does this convey urgency?"
//     }
//   }
// }
