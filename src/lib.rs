use serde_json::Value;
use std::fmt::Display;

pub struct Wordnik {
  api_key: String,
  entry: String,
}

pub enum Operation {
  Audio,
  Definitions,
  Etymologies,
  Examples,
  Frequency,
  Hyphenation,
  Phrases,
  Pronunciations,
  RelatedWords,
  ScrabbleScore,
  TopExample,
}

impl Display for Operation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Operation::Audio => write!(f, "audio"),
      Operation::Definitions => write!(f, "definitions"),
      Operation::Etymologies => write!(f, "etymologies"),
      Operation::Examples => write!(f, "examples"),
      Operation::Frequency => write!(f, "frequency"),
      Operation::Hyphenation => write!(f, "hyphenation"),
      Operation::Phrases => write!(f, "phrases"),
      Operation::Pronunciations => write!(f, "pronunciations"),
      Operation::RelatedWords => write!(f, "relatedWords"),
      Operation::ScrabbleScore => write!(f, "scrabbleScore"),
      Operation::TopExample => write!(f, "topExample"),
    }
  }
}

impl Wordnik {
  pub fn new(api_key: String, entry: String) -> Wordnik {
    Wordnik { api_key, entry }
  }

  fn make_request(
    &self,
    url: String,
  ) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client
      .get(url)
      .header("Content-Type", "application/json")
      .send()?
      .text()?;

    let res: Value = serde_json::from_str(&res)?;
    Ok(res)
  }

  pub fn get_definitions(
    &self,
    word: &str,
  ) -> Result<Value, Box<dyn std::error::Error>> {
    let url = self.entry.clone()
      + word
      + "/"
      + Operation::Definitions.to_string().as_str()
      + "?api_key="
      + &self.api_key;
    self.make_request(url)
  }
}
