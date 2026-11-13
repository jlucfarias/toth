use epub_builder::{
  EpubBuilder,
  EpubContent,
  EpubVersion,
  ReferenceType,
  ZipLibrary,
};
use upon::Engine;
use std::{
  error::Error,
  fs,
};

use crate::{
  Book,
  EpubConfig,
};

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

  fn add_cover(builder: &mut EpubBuilder<ZipLibrary>, book: &Book) -> Result<(), Box<dyn Error>> {
    let cover_path = book.root_path().join(book.cover_path());
    let cover_content = fs::read(cover_path)?;
    let kind = infer::get(&cover_content).ok_or("Test")?;
    let mime = match kind.mime_type() {
      mime @ ("image/png" | "image/jpeg" | "image/gif") => mime,
      other => return Err(format!("Unsupported cover format: {other}").into()),
    };

    builder.add_cover_image(format!("images/cover.{}", kind.extension()), cover_content.as_slice(), mime)?;

    Ok(())
  }

  fn add_content(builder: &mut EpubBuilder<ZipLibrary>, book: &Book) -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new();

    let epub_configs = book.epub_configs().unwrap_or_else(|| EpubConfig::new());

    if !epub_configs.ignore_titlepage() {
      engine.add_template("titlepage", include_str!("../../templates/pages/titlepage.xhtml"))?;

      let titlepage_context = upon::value! {
        lang: book.language().first(),
        titlepage_name: "Titlepage",
        title: book.title(),
      };
      let result = engine.template("titlepage").render(&titlepage_context).to_string()?;

      builder.add_content(
        EpubContent::new("text/titlepage.xhtml", result.as_bytes())
          .title("Titlepage")
          .reftype(ReferenceType::TitlePage)
      )?;
    }

    Ok(())
  }

  pub(crate) fn generate(&self, book: &Book) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;
    builder.epub_version(EpubVersion::V30);

    Self::add_metadata(&mut builder, book)?;
    Self::add_cover(&mut builder, book)?;
    Self::add_content(&mut builder, book)?;

    let mut buffer = Vec::new();
    builder.generate(&mut buffer)?;

    Ok(buffer)
  }
}
