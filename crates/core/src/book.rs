use chrono::{
  DateTime,
  Utc,
};
use std::error::Error;
use serde::Deserialize;

use crate::{
  export::epub::Epub,
  ExportFormat,
  utils::serde::{
    date_or_datetime,
    single_or_vec,
  }
};

#[derive(Debug, Deserialize)]
pub struct Book {
  title: String,
  description: Option<String>,
  #[serde(deserialize_with = "single_or_vec")]
  author: Vec<String>,
  #[serde(deserialize_with = "single_or_vec")]
  subject: Vec<String>,
  #[serde(deserialize_with = "single_or_vec")]
  language: Vec<String>,
  #[serde(default, deserialize_with = "date_or_datetime")]
  publish_date: Option<DateTime<Utc>>,
}

impl Book {
  pub fn new(toml_content: &str) -> Result<Self, Box<dyn Error>> {
    let book: Book = toml::from_str(toml_content)?;

    Ok(book)
  }

  pub fn title(&self) -> &str {
    &self.title
  }

  pub fn description(&self) -> &Option<String> {
    &self.description
  }

  pub fn author(&self) -> &Vec<String> {
    &self.author
  }

  pub fn subject(&self) -> &Vec<String> {
    &self.subject
  }

  pub fn language(&self) -> &Vec<String> {
    &self.language
  }

  pub fn publish_date(&self) -> Option<DateTime<Utc>> {
    self.publish_date
  }

  pub fn export(&self, format: ExportFormat) -> Result<Vec<u8>, Box<dyn Error>> {
    match format {
      ExportFormat::Epub => Epub::new().generate(self),
    }
  }
}
