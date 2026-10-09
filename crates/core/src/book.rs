use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Book {
  title: String,
}

impl Book {
  pub fn new(toml_content: &str) -> Result<Self, Box<dyn Error>> {
    let book: Book = toml::from_str(toml_content)?;

    Ok(book)
  }

  pub fn title(&self) -> &str {
    &self.title
  }
}
