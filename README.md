This is a Rust client for decision APIs, such as Typesafe's Jev model - which is currently the only model we support.

Pass your API key as an environment variable called `DECISION_API_KEY`.

Create a client from the environment:

```rust
use rust_decision::{
  client::Client,
  providers::{Provider, typesafe::TypesafeJev},
  question::{NoulQuestion, Question},
};

let client = rust_decision::client::Client::from_env()?;
```

Then to call the decision API, for example:

```rust
let state = "Our payment system is down. Please help immediately.".to_owned();

 let questions: HashMap<String, Question<HashMap<String, String>>> = HashMap::from([(
    "is_urgent".to_owned(),
    Question::Noul(NoulQuestion::new(
      "Is this urgent?".to_owned(),
    )),
  )]);

  let response = TypesafeJev
    .post(client, state, questions)
    .await
    .map_err(|error| std::io::Error::other(format!("TypeSafe request failed: {error:?}")))?;
```
