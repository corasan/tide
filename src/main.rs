use clap::Parser;
use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use log::{error, info};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;
use std::io::{self, Write};

// let path = "../routinify/supabase"

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> notify::Result<()> {
    init_logger();

    let args = Args::parse();
    let path = args.path;
    
    println!("Watching for changes in: {}", path);

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            tx.send(res).unwrap();
        },
        Config::default(),
    )?;

    watcher.watch(Path::new(&path), RecursiveMode::Recursive).unwrap();

    println!("Watcher started. Press Ctrl-C to quit.");

    for res in rx {
        match res {
            Ok(event) => {
                let event_type = match event.kind {
                    EventKind::Create(_) => "Created",
                    EventKind::Modify(_) => "Modified",
                    EventKind::Remove(_) => "Removed",
                    _ => "Other",
                };

                for path in event.paths {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            println!("{} {} {}", 
                                event_type.blue().bold(), 
                                "→".yellow(), 
                                file_name_str.white().underline()
                            );
                        }
                    }
                }
            }
            Err(e) => println!("{}", format!("Watch error: {:?}", e).red()),
        }
    }

    Ok(())
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