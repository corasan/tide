use colored::*;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

mod schema_processor;

pub fn generate_types(migrations_dir: &Path, output_file: &Path) -> std::io::Result<()> {
  match schema_processor::process_migrations(migrations_dir, output_file) {
    Ok(_) => println!("Initial TypeScript types generated successfully!\n"),
    Err(e) => eprintln!("Error generating initial TypeScript types: {}", e),
  }

  Ok(())
}

pub fn init_watcher(path: &Path, output_file: &Path) -> notify::Result<()> {
  let (tx, rx) = channel();
  let mut watcher = RecommendedWatcher::new(
    move |res: Result<Event, notify::Error>| {
      tx.send(res).unwrap();
    },
    Config::default(),
  )?;

  watcher
    .watch(Path::new(&path), RecursiveMode::Recursive)
    .unwrap();

  println!("Watcher started. Press Ctrl-C to quit.");
  println!("Watching for changes in: {}", path.to_str().unwrap());
  list_sql_files(path)?;
  generate_types(path, output_file)?;

  let path_arc = Arc::new(path.to_path_buf());
  let output_file_arc = Arc::new(output_file.to_path_buf());

  let mut last_events: HashMap<std::path::PathBuf, Instant> = HashMap::new();
  let debounce_duration = Duration::from_secs(1);

  for res in rx {
    match res {
      Ok(event) => {
        let event_type = match event.kind {
          EventKind::Create(_) => "Created",
          EventKind::Modify(_) => "Modified",
          EventKind::Remove(_) => "Removed",
          _ => continue, // Skip other event types
        };

        for path in event.paths {
          if let Some(extension) = path.extension() {
            if extension == "sql" {
              let now = Instant::now();
              if let Some(last_event_time) = last_events.get(&path) {
                if now.duration_since(*last_event_time) < debounce_duration {
                  continue; // Skip this event if it's too soon after the last one
                }
              }
              last_events.insert(path.clone(), now);

              if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                  println!(
                    "{} {} {}",
                    event_type.blue().bold(),
                    "→".yellow(),
                    file_name_str.white().underline()
                  );
                }
              }

              let path_clone = Arc::clone(&path_arc);
              let output_file_clone = Arc::clone(&output_file_arc);

              thread::spawn(move || {
                match schema_processor::process_migrations(&path_clone, &output_file_clone) {
                  Ok(_) => println!("Successfully updated TypeScript types."),
                  Err(e) => println!("Error processing migrations: {:?}", e),
                }
              });
            }
          }
        }
      }
      Err(e) => println!("{}", format!("Watch error: {:?}", e).red()),
    }
  }

  Ok(())
}

fn list_sql_files(dir: &Path) -> std::io::Result<()> {
  let mut sql_files = Vec::new();
  let mut dirs_to_search = vec![dir.to_path_buf()];

  while let Some(current_dir) = dirs_to_search.pop() {
    for entry in fs::read_dir(current_dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        dirs_to_search.push(path);
      } else if let Some(extension) = path.extension() {
        if extension == "sql" {
          sql_files.push(path);
        }
      }
    }
  }

  if sql_files.is_empty() {
    println!("{}", "No SQL files found.".yellow());
  } else {
    println!(
      "{}",
      format!("Found {} migrations:", sql_files.len()).green()
    );
    for file in sql_files {
      if let Some(file_name) = file.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
          println!("  {}", file_name_str.white());
        }
      }
    }
  }

  Ok(())
}
