use crate::custom_table_def::{Column, Table};
use std::collections::{HashMap, HashSet};

// Struct to represent a TS interface
#[derive(Debug)]
pub struct TypeScriptInterface {
  pub name: String,
  pub columns: Vec<Column>,
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
        columns: Vec::new(),
      });
      count = 1;
    } else if let Some(ref mut interface) = current_interface {
    }
  }
  interfaces
}

pub fn update_ts_interfaces(content: &str, schema: &HashMap<String, Table>) -> String {
  let mut updated_content = content.to_string();
  let interfaces = parse_typescript(content);

  println!("interfaces: {:?}", interfaces);

  // for (table_name, table) in schema {
  //   let pscal_case_name = utils::to_pascal_case(table_name);
  //   println!("table name: {}", pscal_case_name);
  //   if let Some(captures) = interface_regex.captures(&updated_content) {
  //     println!("captures: {:?}", captures);
  //     let interface_name = captures.get(1).unwrap().as_str();
  //     println!("interface name: {}", interface_name);
  //     if interface_name == pscal_case_name {
  //       println!("found interface");
  //       let interface_content = captures.get(2).unwrap().as_str();
  //       println!("interface content: {}", interface_content);
  //       let mut new_interface_content = String::new();
  //       let existing_cols: HashSet<_> = interface_content
  //         .lines()
  //         .filter_map(|line| {
  //           let trimmed_line = line.trim();
  //           if trimmed_line.ends_with(';') {
  //             Some(trimmed_line.split(':').next().unwrap().trim().to_string())
  //           } else {
  //             None
  //           }
  //         })
  //         .collect();
  //       println!("existing cols: {:?}", existing_cols);
  //     }
  //   }
  // }
  updated_content
}
