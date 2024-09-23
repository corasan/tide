use std::fs;
use std::path::{Path, PathBuf};

// read SQL migrations in chronological order
fn read_migrations(dir: &Path) -> Vec<PathBuf> {
  let mut migrations = fs::read_dir(dir)
    .unwrap()
    .filter_map(|entry| {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.extension().and_then(|s| s.to_str()) == Some("sql") {
        Some(path)
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  migrations.sort_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));
  migrations
}

// Main function to process migrations and generate TypeScript types
pub fn process_migrations(migrations_dir: &Path) -> std::io::Result<()> {
  let migrations = read_migrations(migrations_dir);

  for migration in migrations {
    let content = fs::read_to_string(migration)?;
    println!("{}", content);
  }

  Ok(())
}
