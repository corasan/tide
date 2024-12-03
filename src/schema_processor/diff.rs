use log::info;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use super::typescript::update_ts_interfaces;
use crate::custom_table_def::Table;

pub fn compare_and_update_types(
  new_content: &str,
  output_file: &Path,
  schema: &HashMap<String, Table>,
) -> std::io::Result<bool> {
  let mut current_content = String::new();
  let file_exists = output_file.exists();

  if file_exists {
    let mut file = fs::File::open(output_file)?;
    file.read_to_string(&mut current_content)?;
  }

  let compared_content = update_ts_interfaces(&current_content, schema);

  if !output_file.exists() {
    info!("TypeScript types file doesn't exist. Creating a new file.");
    let mut file = fs::File::create(output_file)?;
    file.write_all(new_content.as_bytes())?;
    info!("Created new TypeScript types file.");
    return Ok(true);
  }

  let mut file = fs::File::open(output_file)?;
  file.read_to_string(&mut current_content)?;

  if new_content != current_content {
    let mut updated_lines = Vec::new();
    let current_lines: Vec<&str> = current_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    for (i, (line, new_line)) in current_lines.iter().zip(new_lines.iter()).enumerate() {
      if line != new_line {
        updated_lines.push((i, new_line));
      }
    }

    // Add new lines that don't exist in the current file
    if new_lines.len() > current_lines.len() {
      for (i, line) in new_lines.iter().enumerate().skip(current_lines.len()) {
        updated_lines.push((i, line));
      }
    }

    let mut file = fs::OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(output_file)?;

    // for line in new_lines.iter() {
    //   writeln!(file, "{}", line)?;
    // }

    write!(file, "{}", compared_content)?;
    Ok(true)
  } else {
    Ok(false)
  }
}
