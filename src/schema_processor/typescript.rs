use super::utils;
use crate::custom_table_def::{Column, Table};
use std::collections::{HashMap, HashSet};

// Struct to represent a TS interface
#[derive(Debug)]
pub struct TypeScriptInterface {
  pub name: String,
  pub content: Vec<String>,
}

fn parse_typescript(content: &str) -> Vec<TypeScriptInterface> {
  let mut interfaces = Vec::new();
  let mut current_interface: Option<TypeScriptInterface> = None;
  let mut count = 0;

  for line in content.lines() {
    let trimmed_line = line.trim();

    if trimmed_line.starts_with("export interface") {
      let name = trimmed_line
        .split_whitespace()
        .nth(2)
        .unwrap_or("")
        .trim_end_matches('{')
        .to_string();
      current_interface = Some(TypeScriptInterface {
        name,
        content: Vec::new(),
      });
      count = 1;
    } else if let Some(ref mut interface) = current_interface {
      interface.content.push(line.to_string());
      count += trimmed_line.matches('{').count();
      count -= trimmed_line.matches('}').count();

      if count == 0 {
        interfaces.push(current_interface.take().unwrap());
      }
    }
  }
  interfaces
}

fn is_property_line(line: &str) -> bool {
  let trimmed = line.trim();
  trimmed.contains(':')
}

pub fn update_ts_interfaces(content: &str, schema: &HashMap<String, Table>) -> String {
  let mut interfaces = parse_typescript(content);
  let mut updated_content = String::new();

  for (table_name, table) in schema {
    let pscal_case_name = utils::to_pascal_case(table_name);
    let interface_idx = interfaces.iter().position(|i| i.name == pscal_case_name);

    if let Some(interface_idx) = interface_idx {
      let interface = &mut interfaces[interface_idx];
      let mut updated_lines = Vec::new();
      let mut existing_cols = HashSet::new();

      for line in &interface.content {
        if is_property_line(line) {
          let col_name = line.split(':').next().unwrap().trim().to_string();
          existing_cols.insert(col_name);
        }
        updated_lines.push(line.clone());
      }

      for column in &table.columns {
        let col_name = column.name.replace("\"", "");
        let ts_type = utils::sql_to_typescript_type(&column.data_type);
        let nullabe_suffix = if column.nullable { "?" } else { "" };
        let new_line = format!("  {}{}: {};\n", col_name, nullabe_suffix, ts_type);

        if !existing_cols.contains(&col_name) {
          updated_lines.push(new_line);
        }
      }
      interface.content = updated_lines;
    } else {
      let mut new_interface = TypeScriptInterface {
        name: pscal_case_name.clone(),
        content: Vec::new(),
      };
      new_interface.content.push("{".to_string());
      for col in &table.columns {
        let col_name = col.name.replace("\"", "");
        let ts_type = utils::sql_to_typescript_type(&col.data_type);
        let nullabe_suffix = if col.nullable { "?" } else { "" };
        let new_line = format!("  {}{}: {};\n", col_name, nullabe_suffix, ts_type);
        new_interface.content.push(new_line);
      }
      new_interface.content.push("}".to_string());
      interfaces.push(new_interface);
    }
  }

  for interface in interfaces {
    updated_content.push_str(&format!("export interface {} {{\n", interface.name));
    updated_content.push_str(&interface.content.join("\n"));
    updated_content.push_str("}\n\n");
  }

  updated_content
}
