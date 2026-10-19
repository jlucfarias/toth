use std::error::Error;
use serde::Deserialize;

use crate::{
  export::epub::Epub,
  ExportFormat,
};

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

  pub fn export(&self, format: ExportFormat) -> Result<Vec<u8>, Box<dyn Error>> {
    match format {
      ExportFormat::Epub => Epub::new().generate(self),
    }
  }
}
