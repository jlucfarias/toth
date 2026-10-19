use epub_builder::{
  EpubBuilder,
  EpubVersion,
  ZipLibrary
};
use std::error::Error;

use crate::Book;

pub(crate) struct Epub {
}

impl Epub {
  pub(crate) fn new() -> Self {
    Self {}
  }

  pub(crate) fn generate(&self, _book: &Book) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;
    builder.epub_version(EpubVersion::V30);

    let mut buffer = Vec::new();
    builder.generate(&mut buffer)?;

    Ok(buffer)
  }
}
