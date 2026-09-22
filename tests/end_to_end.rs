use std::collections::HashMap;

use rust_decision::{
  answer::Answer,
  client::Client,
  providers::{Provider, typesafe::TypesafeJev},
  question::{ChoiceQuestion, NoulQuestion, Question, ScoreQuestion},
};

#[tokio::test]
#[ignore = "requires DECISION_API_KEY and calls the live API"]
async fn choice() {
  let client = Client::from_env().expect("set DECISION_API_KEY");
  let questions = HashMap::from([(
    "department".to_owned(),
    Question::Choice(ChoiceQuestion::new(
      "Which team should handle this?".to_owned(),
      [("billing", "Payments"), ("technical", "Outages")],
    )),
  )]);

  let response = TypesafeJev
    .decide(&client, "I was charged twice.".to_owned(), questions)
    .await
    .expect("choice request failed");

  println!("response: {response:?}");

  let Answer::ChoiceAnswer {
    choice,
    probabilities,
    ..
  } = &response.answers["department"]
  else {
    panic!("expected a choice answer");
  };
  assert!(["billing", "technical"].contains(&choice.0.as_str()));
  assert!(probabilities.contains_key(&choice.0));
}

#[tokio::test]
#[ignore = "requires DECISION_API_KEY and calls the live API"]
async fn score() {
  let client = Client::from_env().expect("set DECISION_API_KEY");
  let questions = HashMap::from([(
    "frustration".to_owned(),
    Question::Score(ScoreQuestion::new(
      "How frustrated is the customer?".to_owned(),
      vec![
        "Calm".to_owned(),
        "Frustrated".to_owned(),
        "Very angry".to_owned(),
      ],
    )),
  )]);

  let response = TypesafeJev
    .decide(
      &client,
      "My payouts have been failing for three days!".to_owned(),
      questions,
    )
    .await
    .expect("score request failed");

  println!("response: {response:?}");

  let Answer::ScoreAnswer {
    score,
    legend,
    probabilities,
    ..
  } = &response.answers["frustration"]
  else {
    panic!("expected a score answer");
  };
  assert!((0.0..=2.0).contains(&score.0));
  assert_eq!(legend.len(), 3);
  assert!(!probabilities.is_empty());
}

#[tokio::test]
#[ignore = "requires DECISION_API_KEY and calls the live API"]
async fn noul() {
  let client = Client::from_env().expect("set DECISION_API_KEY");
  let questions = HashMap::from([(
    "urgent".to_owned(),
    Question::Noul(NoulQuestion::new("Is this urgent?".to_owned())),
  )]);

  let response = TypesafeJev
    .decide(
      &client,
      "Our payment system is down. Help immediately!".to_owned(),
      questions,
    )
    .await
    .expect("noul request failed");

  println!("response: {response:?}");

  let Answer::NoulAnswer { noul } = &response.answers["urgent"] else {
    panic!("expected a noul answer");
  };
  assert!((0.0..=1.0).contains(&noul.get()));
}
