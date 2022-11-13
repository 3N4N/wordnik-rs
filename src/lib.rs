use serde_json::Value;

pub fn validate_key(url: &str) -> Result<Value, Box<dyn std::error::Error>> {
  let client = reqwest::blocking::Client::new();
  let res = client
    .get(url)
    .header("Content-Type", "application/json")
    .send()?
    .text()?;

  let res: Value = serde_json::from_str(&res)?;
  Ok(res)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn key_valid() {
    let response = r#"{"message":"Invalid authentication credentials"}"#;
    let vk = validate_key(
      "https://api.wordnik.com/v4/word.json/sere/definitions?api_key=wrongkey",
    );
    assert!(vk.is_ok());
    assert_eq!(response, serde_json::to_string(&vk.unwrap()).unwrap());
  }
}
