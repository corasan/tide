use log::info;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use super::typescript::update_ts_interfaces;
use crate::custom_table_def::Table;

// pub fn update_ts_interfaces(content: &str, schema: &HashMap<String, Table>) -> String {
//   let mut updated_content = content.to_string();
//   let interface_regex = Regex::new(r"(?ms)export\s+interface\s+(\w+)\s*\{(.*?)\}").unwrap();

//   for (table_name, table) in schema {
//     let pscal_case_name = utils::to_pascal_case(table_name);
//     println!("table name: {}", pscal_case_name);
//     if let Some(captures) = interface_regex.captures(&updated_content) {
//       println!("captures: {:?}", captures);
//       let interface_name = captures.get(1).unwrap().as_str();
//       println!("interface name: {}", interface_name);
//       if interface_name == pscal_case_name {
//         println!("found interface");
//         let interface_content = captures.get(2).unwrap().as_str();
//         println!("interface content: {}", interface_content);
//         let mut new_interface_content = String::new();
//         let existing_cols: HashSet<_> = interface_content
//           .lines()
//           .filter_map(|line| {
//             let trimmed_line = line.trim();
//             if trimmed_line.ends_with(';') {
//               Some(trimmed_line.split(':').next().unwrap().trim().to_string())
//             } else {
//               None
//             }
//           })
//           .collect();
//         println!("existing cols: {:?}", existing_cols);
//       }
//     }
//   }
//   updated_content
// }

pub fn compare_and_update_types(
  new_content: &str,
  output_file: &Path,
  schema: &HashMap<String, Table>,
) -> std::io::Result<bool> {
  let mut current_content = String::new();
  let file_exists = output_file.exists();

  let updated_content = update_ts_interfaces(&current_content, schema);

  println!("new content: {}", updated_content);

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

    for line in new_lines.iter() {
      writeln!(file, "{}", line)?;
    }

    Ok(true)
  } else {
    Ok(false)
  }
}
