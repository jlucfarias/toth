use clap::{
  Parser,
  ValueEnum
};
use std::{
  error::Error,
  fs,
  path::PathBuf,
  process::ExitCode
};

use toth_core::{
  Book,
  ExportFormat
};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum FileFormat {
  Epub,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  book_path: PathBuf,

  #[arg(short, long, value_enum, default_value_t = FileFormat::Epub)]
  format: FileFormat,

  #[arg(short, long)]
  output_path: Option<PathBuf>,
}

const CONFIG_FILE: &str = "book.toml";

fn main() -> ExitCode {
  let args = Args::parse();

  match run(&args) {
    Ok(()) => ExitCode::SUCCESS,
    Err(err) => {
      eprintln!("Error: {:?}", err);

      ExitCode::FAILURE
    }
  }
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
  let toml_path = args.book_path.join(CONFIG_FILE);
  let toml_content = fs::read_to_string(&toml_path)?;
  let book = Book::new(&toml_content)?;
  let format = match args.format {
    FileFormat::Epub => ExportFormat::Epub,
  };
  let output_dir = args.output_path.as_deref().unwrap_or(args.book_path.as_path());
  let file_name = format!("{}.{}", book.title(), format.extension());
  let final_path = output_dir.join(&file_name);
  let file_content = book.export(format)?;

  fs::write(&final_path, file_content)?;
  println!("{file_name} generated successfully at {}", final_path.display());

  Ok(())
}
