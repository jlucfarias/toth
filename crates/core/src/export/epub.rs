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

  fn add_metadata(builder: &mut EpubBuilder<ZipLibrary>, book: &Book) -> Result<(), Box<dyn Error>> {
    builder
      .metadata("title", book.title())?
      .metadata("generator", "Toth Ebook Generator")?;

    if let Some(description) = book.description() {
      builder.add_description(description);
    }

    if let Some(publish_date) = book.publish_date() {
      builder.set_publication_date(publish_date);
    }

    for author in book.author() {
      builder.metadata("author", author)?;
    }

    for subject in book.subject() {
      builder.metadata("subject", subject)?;
    }

    for language in book.language() {
      builder.metadata("lang", language)?;
    }

    Ok(())
  }

  pub(crate) fn generate(&self, book: &Book) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;
    builder.epub_version(EpubVersion::V30);

    Self::add_metadata(&mut builder, book)?;

    let mut buffer = Vec::new();
    builder.generate(&mut buffer)?;

    Ok(buffer)
  }
}
