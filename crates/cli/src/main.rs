use clap::Parser;
use std::{
  error::Error,
  fs,
  path::PathBuf,
  process::ExitCode
};

use toth_core::Book;

const CONFIG_FILE: &str = "book.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  book_path: PathBuf,
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
  let toml_path = args.book_path.join(CONFIG_FILE);
  let toml_content = fs::read_to_string(&toml_path)?;
  let book = Book::new(&toml_content)?;

  println!("Book title: {}", book.title());

  Ok(())
}
