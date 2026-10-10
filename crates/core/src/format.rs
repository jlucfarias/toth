use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExportFormat {
  Epub,
}

impl ExportFormat {
  pub fn extension(&self) -> &'static str {
    match self {
      ExportFormat::Epub => "epub",
    }
  }
}

impl fmt::Display for ExportFormat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
    f.write_str(self.extension())
  }
}
