# Rust Decision

This is a Rust client for decision APIs, such as Typesafe's Jev model (which is currently the only model we support).

Pass your API key as an environment variable called `DECISION_API_KEY`.

Create a client from the environment:

```rust
use std::collections::HashMap;

use rust_decision::{
  client::Client,
  providers::{Provider, typesafe::TypesafeJev},
  question::{ChoiceQuestion, NoulQuestion, Question, ScoreQuestion},
};

let client = rust_decision::client::Client::from_env()?;
```

## Noul

To call the decision API for yes or no answers:

```rust
let state = "Our payment system is down. Please help immediately.".to_owned();

let questions = HashMap::from([(
  "is_urgent".to_owned(),
  Question::Noul(NoulQuestion::new("Is this urgent?".to_owned())),
)]);

let response = TypesafeJev
  .decide(&client, state, questions)
  .await
  .map_err(|error| std::io::Error::other(format!("TypeSafe request failed: {error:?}")))?;
```

## Choice

To call the decision API for choices:

```rust
let state = "I was charged twice.".to_owned();

let questions = HashMap::from([(
  "department".to_owned(),
  Question::Choice(ChoiceQuestion::new(
    "Which team should handle this?".to_owned(),
    [("billing", "Payments"), ("technical", "Outages")],
  )),
)]);

let response = TypesafeJev
  .decide(&client, state, questions)
  .await
  .map_err(|error| std::io::Error::other(format!("TypeSafe request failed: {error:?}")))?;
```

## Score

To call the decision API for scores:

```rust
let state = "My payouts have been failing for three days!".to_owned();

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
  .decide(&client, state, questions)
  .await
  .map_err(|error| std::io::Error::other(format!("TypeSafe request failed: {error:?}")))?;
```
