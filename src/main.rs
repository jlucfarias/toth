use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  book_path: PathBuf,
}

fn main() {
  let args = Args::parse();

  println!("Book path: {}", &args.book_path.display());
}
