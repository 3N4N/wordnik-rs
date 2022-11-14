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
