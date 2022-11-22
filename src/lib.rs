use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
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

/// The struct representing API response for word definition.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
  pub word: String,
  #[serde(default, rename = "text")]
  pub definition: String,
  #[serde(default)]
  pub part_of_speech: String,
  pub attribution_text: String,
  pub source_dictionary: String,
  pub attribution_url: String,
  pub wordnik_url: String,
}

impl Definition {
  fn to_pretty(&self) -> String {
    if self.part_of_speech.clone() == "" {
      self.definition.clone() + "\n"
    } else {
      self.part_of_speech.clone() + " " + &self.definition + "\n"
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

  /// Get definitions of a word.
  ///
  /// For a pretty string of definitions,
  /// see [get_definitions_pretty](Self::get_definitions_pretty).
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
    let mut definitions: Vec<Definition> = serde_json::from_value(res)?;
    definitions.retain(|def| def.definition != "" );
    Ok(definitions)
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
  /// let v = api.get_definitions_pretty("word").unwrap();
  /// println!("{}", v);
  /// ```
  pub fn get_definitions_pretty(
    &self,
    word: &str,
  ) -> Result<String, Box<dyn std::error::Error>> {
    let definitions = self.get_definitions(word)?;
    let mut hm_definitions: HashMap<String, Vec<Definition>> = HashMap::new();
    for i in &definitions {
      if hm_definitions.contains_key(&i.attribution_text) {
        let v = hm_definitions.get_mut(&i.attribution_text).unwrap();
        v.push(i.clone());
      } else {
        let mut v: Vec<Definition> = Vec::new();
        v.push(i.clone());
        hm_definitions.insert(i.attribution_text.clone(), v);
      }
    }

    let mut s = "".to_string();
    for (attribution_text, definitions) in hm_definitions.iter() {
      s = s
        + attribution_text
        + "\n"
        + definitions
          .iter()
          .fold("".into(), |acc, d| {
            [acc, "  * ".into(), d.to_pretty()].join("")
          })
          .as_str()
        + "\n";
    }

    Ok(s)
  }
}
