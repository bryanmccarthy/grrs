#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String, // Word to search for
  path: std::path::PathBuf, // File to search in
}

#[derive(Debug)]
struct CustomError(String);

fn main() {
  let args = Cli::parse();
  let path = &args.path; 
  let content = std::fs::read_to_string(path).expect("could not read file");

  for line in content.lines() {
    if line.contains(&args.pattern) {
      println!("{}", line);
    }
  }
}
