use clap::Parser;
use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::path::Path;

mod watcher;

// let path = "../routinify/supabase"

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Path to the directory containing SQL migration files
  #[arg(short, long)]
  path: String,

  /// Output file path for the generated TS types
  #[arg(short, long)]
  output: String,
}

fn main() {
  init_logger();

  let args = Args::parse();
  let path = Path::new(&args.path);
  let output_file = Path::new(&args.output);

  watcher::init_watcher(path, output_file).unwrap();
}

fn init_logger() {
  Builder::new()
    .format(|buf, record| {
      let level = record.level();
      writeln!(
        buf,
        "[{}] - {}",
        match level {
          log::Level::Error => level.to_string().red(),
          log::Level::Warn => level.to_string().yellow(),
          log::Level::Info => level.to_string().green(),
          log::Level::Debug => level.to_string().purple(),
          log::Level::Trace => level.to_string().cyan(),
        },
        record.args()
      )
    })
    .filter(None, LevelFilter::max())
    .init();
}
