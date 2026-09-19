use std::fmt;

use serde::Deserialize;

pub type Probabilities = Vec<Probability>;

#[derive(Clone, Debug)]
pub struct Probability(pub f32);

#[derive(Clone, Debug)]
pub struct OutOfBounds;

impl fmt::Display for OutOfBounds {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("f32 is out of bounds")
  }
}

impl Probability {
  fn new(value: f32) -> Result<Self, OutOfBounds> {
    if !(0.0..=1.0).contains(&value) {
      return Err(OutOfBounds);
    }

    Ok(Probability(value))
  }
}

impl<'de> Deserialize<'de> for Probability {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_f32(ProbabilityVisitor)
  }
}

struct ProbabilityVisitor;

impl<'de> serde::de::Visitor<'de> for ProbabilityVisitor {
  type Value = Probability;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("Must be an f32")
  }

  fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Probability::new(v).map_err(serde::de::Error::custom)
  }
}

#[cfg(test)]
mod tests {
  use quickcheck::quickcheck;
  use serde::de::value::{BoolDeserializer, Error, F32Deserializer, StrDeserializer};

  use super::*;

  quickcheck! {
    fn deserialization_respects_bounds(value: f32) -> bool {
      let result =
        Probability::deserialize(F32Deserializer::<Error>::new(value));

      match result {
        Ok(probability) => {
          (0.0..=1.0).contains(&value) && probability.0 == value
        }
        Err(_) => !(0.0..=1.0).contains(&value),
      }
    }
  }

  #[test]
  fn deserializes_probabilities_within_bounds() {
    for value in [0.1, 0.5, 0.9] {
      let probability = Probability::deserialize(F32Deserializer::<Error>::new(value)).unwrap();
      assert_eq!(probability.0, value);
    }
  }

  #[test]
  fn deserializes_inclusive_boundaries() {
    for value in [0.0, 1.0] {
      let probability = Probability::deserialize(F32Deserializer::<Error>::new(value)).unwrap();
      assert_eq!(probability.0, value);
    }
  }

  #[test]
  fn rejects_out_of_bounds_probabilities() {
    for value in [-0.1, 1.1, f32::MIN, f32::MAX] {
      let error = Probability::deserialize(F32Deserializer::<Error>::new(value)).unwrap_err();
      assert_eq!(error.to_string(), "f32 is out of bounds");
    }
  }

  #[test]
  fn rejects_infinite_probabilities() {
    for value in [f32::NEG_INFINITY, f32::INFINITY] {
      let error = Probability::deserialize(F32Deserializer::<Error>::new(value)).unwrap_err();
      assert_eq!(error.to_string(), "f32 is out of bounds");
    }
  }

  #[test]
  fn rejects_non_numeric_inputs() {
    let error = Probability::deserialize(StrDeserializer::<Error>::new("0.5")).unwrap_err();
    assert!(error.to_string().contains("Must be an f32"));

    let error = Probability::deserialize(BoolDeserializer::<Error>::new(true)).unwrap_err();
    assert!(error.to_string().contains("Must be an f32"));
  }
}
