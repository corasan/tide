use super::utils;
use crate::custom_table_def::Table;
use std::collections::HashMap;

#[derive(Debug)]
struct InterfaceLocation {
  start: usize,
  end: usize,
  // content: Vec<String>,
}

pub fn update_ts_interfaces(content: &str, schema: &HashMap<String, Table>) -> String {
  // Split content into lines, filtering out empty lines at the end
  let mut lines: Vec<String> = content
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  // Remove trailing empty lines
  while lines.last().map_or(false, |line| line.trim().is_empty()) {
    lines.pop();
  }

  let interface_locations = find_interface_locations(&lines);
  let mut updated_interfaces = Vec::new();
  let mut processed_names = std::collections::HashSet::new();

  // Process existing interfaces
  for (name, location) in &interface_locations {
    if let Some(table) = schema.values().find(|t| t.name == *name) {
      let interface_content = generate_interface_content(name, table);
      updated_interfaces.push((location.start, location.end, interface_content));
      processed_names.insert(name.clone());
    }
  }

  // Sort updates from last to first to avoid invalidating indices
  updated_interfaces.sort_by(|a, b| b.0.cmp(&a.0));

  // Apply updates
  for (start, end, content) in updated_interfaces {
    lines.splice(start..=end, content);
  }

  // Add new interfaces
  for (_, table) in schema {
    if !processed_names.contains(&table.name) {
      if !lines.is_empty() {
        lines.push(String::new()); // Add spacing
      }
      lines.extend(generate_interface_content(&table.name, table));
    }
  }

  // Ensure proper ending
  if !lines.is_empty() {
    lines.push(String::new());
  }

  lines.join("\n")
}

fn generate_interface_content(name: &str, table: &Table) -> Vec<String> {
  let mut content = Vec::new();
  content.push(format!("export interface {} {{", name));

  // Add columns
  for column in &table.columns {
    let col_name = column.name.replace("\"", "");
    let ts_type = utils::sql_to_typescript_type(&column.data_type);
    let nullable_suffix = if column.nullable { "?" } else { "" };
    content.push(format!("  {}{}: {}", col_name, nullable_suffix, ts_type));
  }

  content.push("}".to_string());

  println!("{}", content.join("\n"));
  content
}

fn find_interface_locations(lines: &[String]) -> HashMap<String, InterfaceLocation> {
  let mut locations = HashMap::new();
  let mut i = 0;

  while i < lines.len() {
    let line = &lines[i];
    if line.trim().starts_with("export interface") {
      let name = line
        .split_whitespace()
        .nth(2)
        .unwrap_or("")
        .trim_end_matches('{')
        .to_string();

      let start = i;
      let mut brace_count = line.matches('{').count() as i32;
      let mut content = vec![line.clone()];
      let mut end = i;

      while brace_count > 0 && end < lines.len() {
        end += 1;
        if end < lines.len() {
          let current_line = &lines[end];
          content.push(current_line.clone());
          brace_count += current_line.matches('{').count() as i32;
          brace_count -= current_line.matches('}').count() as i32;
        }
      }

      if brace_count == 0 {
        locations.insert(
          name,
          InterfaceLocation {
            start,
            end,
            // content,
          },
        );
      }
    }
    i += 1;
  }

  locations
}
