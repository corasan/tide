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
  #[arg(short, long)]
  path: String,
}

fn main() {
  init_logger();

  let args = Args::parse();
  let path = Path::new(&args.path);

  watcher::init_watcher(path).unwrap();
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
