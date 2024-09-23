use sqlparser::ast::{CreateTable, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

mod utils;

// Struct to represent a column in a table
#[derive(Debug, Clone)]
struct Column {
  name: String,
  data_type: String,
  nullable: bool,
}

// Struct to represent a table
#[derive(Debug)]
struct Table {
  name: String,
  columns: Vec<Column>,
}

// read SQL migrations in chronological order
fn read_migrations(dir: &Path) -> Vec<PathBuf> {
  let mut migrations = fs::read_dir(dir)
    .unwrap()
    .filter_map(|entry| {
      let entry = entry.ok()?;
      let path = entry.path();
      if path.extension().and_then(|s| s.to_str()) == Some("sql") {
        Some(path)
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  migrations.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
  migrations
}

// Parse a SQL migration file and add the parsed schema to the schema map
fn parse_migration(content: &str, schema: &mut HashMap<String, Table>) {
  let dialect = GenericDialect {}; // or use the appropriate dialect for your SQL flavor
  let ast = match Parser::parse_sql(&dialect, content) {
    Ok(ast) => ast,
    Err(e) => {
      println!(
        "Warning: Error parsing SQL (this statement will be ignored): {:?}",
        e
      );
      return;
    }
  };

  for statement in ast {
    match statement {
      Statement::CreateTable(CreateTable { name, columns, .. }) => {
        let table_name = utils::extract_table_name(&name);
        let pascal_case_name = utils::to_pascal_case(&table_name);
        let columns = utils::parse_columns(&columns);
        schema.insert(
          table_name.clone(),
          Table {
            name: pascal_case_name,
            columns,
          },
        );
      }
      // Statement::AlterTable { name, operation } => {
      //   let table_name = name.to_string();
      //   if let Some(table) = schema.get_mut(&table_name) {
      //     match operation {
      //       AlterTableOperation::AddColumn { column_def, .. } => {
      //         let column = parse_column(&column_def);
      //         table.columns.push(column);
      //       }
      //       AlterTableOperation::DropColumn { name, .. } => {
      //         table.columns.retain(|c| c.name != name.to_string());
      //       }
      //       _ => {} // Handle other alter table operations as needed
      //     }
      //   }
      // }
      _ => {} // Ignore other types of statements
    }
  }
}

// Generate TypeScript types from the schema
fn generate_typescript_types(schema: &HashMap<String, Table>) -> String {
  let mut typescript = String::new();

  for (_, table) in schema {
    typescript.push_str(&format!("interface {} {{\n", table.name));
    for column in &table.columns {
      let ts_type = utils::sql_to_typescript_type(&column.data_type);
      typescript.push_str(&format!(
        "  {}{}: {}\n",
        column.name,
        if column.nullable { "?" } else { "" },
        ts_type
      ));
    }
    typescript.push_str("}\n\n");
  }

  typescript
}

// Main function to process migrations and generate TypeScript types
pub fn process_migrations(migrations_dir: &Path, output_file: &Path) -> std::io::Result<()> {
  let migrations = read_migrations(migrations_dir);

  let mut schema = HashMap::new();

  println!("creating types...");
  for migration in migrations {
    let content = fs::read_to_string(&migration)?;
    parse_migration(&content, &mut schema);
  }

  let typescript = generate_typescript_types(&schema);

  fs::write(output_file, typescript)?;
  println!("TypeScript written to file: {:?}", output_file);

  Ok(())
}
