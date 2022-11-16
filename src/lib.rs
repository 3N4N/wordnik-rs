use serde_json::Value;
use std::fmt::Display;

/// The struct representing the API endpoint.
///
/// # Example
/// ```
/// use wordnik::Wordnik;
///
/// let api = Wordnik::new(
///   "YOUR_API_KEY".to_string(),
///   "https://api.wordnik.com/v4/word.json/".to_string(),
/// );
/// ```
pub struct Wordnik {
  api_key: String,
  entry: String,
}

/// The supported operations.
///
/// See [Wordnik docs](https://developer.wordnik.com/docs#/word).
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

use serde::Deserialize;
use serde::Serialize;

/// The struct representing API response for word definition.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
  pub word: String,
  #[serde(rename = "text")]
  pub definition: String,
  #[serde(default)]
  pub part_of_speech: String,
  pub attribution_text: String,
  pub source_dictionary: String,
  pub attribution_url: String,
  pub wordnik_url: String,
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

  /// Get definitions of a word.
  ///
  /// # Example
  ///
  /// ```
  /// use wordnik::Wordnik;
  ///
  /// let api = Wordnik::new(
  ///   "YOUR_API_KEY".to_string(),
  ///   "https://api.wordnik.com/v4/word.json/".to_string(),
  /// );
  ///
  /// let v = api.get_definitions("word").unwrap();
  /// println!("{:#?}", v);
  /// ```
  pub fn get_definitions(
    &self,
    word: &str,
  ) -> Result<Vec<Definition>, Box<dyn std::error::Error>> {
    let url = self.entry.clone()
      + word
      + "/"
      + Operation::Definitions.to_string().as_str()
      + "?api_key="
      + &self.api_key;
    let res = self.make_request(url)?;
    let definitions: Vec<Definition> = serde_json::from_value(res)?;
    Ok(definitions)
  }
}
