use chrono::{
  DateTime,
  Utc,
};
use std::{
  error::Error,
  path::{
    Path,
    PathBuf,
  }
};
use serde::Deserialize;

use crate::{
  export::epub::Epub,
  ExportFormat,
  utils::serde::{
    date_or_datetime,
    single_or_vec,
  }
};

#[derive(Copy, Clone, Debug, Deserialize, Default)]
pub struct EpubConfig {
  ignore_titlepage: bool,
}

impl EpubConfig {
  pub fn new() -> Self {
    Self {
      ignore_titlepage: false,
    }
  }

  pub fn ignore_titlepage(&self) -> &bool {
    &self.ignore_titlepage
  }
}

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

  #[serde(default)]
  epub_configs: Option<EpubConfig>,

  #[serde(skip)]
  root_path: PathBuf,
  cover_path: PathBuf,
}

impl Book {
  pub fn new(toml_content: &str, path: &Path) -> Result<Self, Box<dyn Error>> {
    let mut book: Book = toml::from_str(toml_content)?;
    book.root_path = path.to_path_buf();

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

  pub fn root_path(&self) -> &Path {
    &self.root_path
  }

  pub fn cover_path(&self) -> &Path {
    &self.cover_path
  }

  pub fn epub_configs(&self) -> Option<EpubConfig> {
    self.epub_configs
  }

  pub fn export(&self, format: ExportFormat) -> Result<Vec<u8>, Box<dyn Error>> {
    match format {
      ExportFormat::Epub => Epub::new().generate(self),
    }
  }
}
