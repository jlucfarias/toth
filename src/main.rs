use clap::Parser;
use serde::Deserialize;
use std::{
  error::Error,
  fs,
  path::PathBuf,
  process::ExitCode
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  book_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Book {
  title: String,
}

fn read_book_settings(path: &PathBuf) -> Result<Book, Box<dyn Error>> {
  let final_path = path.join("book.toml");
  let content = fs::read_to_string(final_path)?;
  let config = toml::from_str(&content)?;

  Ok(config)
}

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
  let settings = read_book_settings(&args.book_path)?;

  println!("Book title: {}", settings.title);

  Ok(())
}
