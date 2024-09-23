use sqlparser::ast::{ColumnDef, DataType, ObjectName};

use super::Column;

pub fn extract_table_name(object_name: &ObjectName) -> String {
  // Get the last part of the object name (which should be the table name)
  object_name
    .0
    .last()
    .map(|ident| ident.value.clone())
    .unwrap_or_else(|| "unknown_table".to_string())
}

// Convert a SQL table name to PascalCase
pub fn to_pascal_case(s: &str) -> String {
  let replaced_str = s.replace("\"", "");
  replaced_str
    .split('_')
    .map(|word| {
      let mut chars = word.chars();
      match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
      }
    })
    .collect()
}

// Convert SQL data type to string
pub fn data_type_to_string(data_type: &DataType) -> String {
  match data_type {
    DataType::Int(Some(n)) => format!("INTEGER({})", n),
    DataType::Int(None) => "INTEGER".to_string(),
    DataType::SmallInt(Some(n)) => format!("SMALLINT({})", n),
    DataType::SmallInt(None) => "SMALLINT".to_string(),
    DataType::BigInt(Some(n)) => format!("BIGINT({})", n),
    DataType::BigInt(None) => "BIGINT".to_string(),
    DataType::Text => "TEXT".to_string(),
    DataType::Varchar(Some(n)) => format!("VARCHAR({})", n),
    DataType::Varchar(None) => "VARCHAR".to_string(),
    DataType::Float(Some(n)) => format!("FLOAT({})", n),
    DataType::Float(None) => "FLOAT".to_string(),
    DataType::Double => "DOUBLE".to_string(),
    DataType::Boolean => "BOOLEAN".to_string(),
    DataType::Date => "DATE".to_string(),
    DataType::Time(..) => "TIME".to_string(),
    DataType::Timestamp(..) => "TIMESTAMP".to_string(),
    DataType::Uuid => "UUID".to_string(),
    _ => format!("{:?}", data_type), // fallack for unhandled data types
  }
}

// Convert SQL data type to TypeScript type
pub fn sql_to_typescript_type(sql_type: &str) -> &str {
  match sql_type.split('(').next().unwrap().to_uppercase().as_str() {
    "INTEGER" | "INT" | "SMALLINT" | "BIGINT" => "number",
    "FLOAT" | "REAL" | "DOUBLE" => "number",
    "CHAR" | "VARCHAR" | "TEXT" | "UUID" => "string",
    "BOOLEAN" => "boolean",
    "DATE" | "TIME" | "TIMESTAMP" => "Date",
    _ => "any",
  }
}

// Parse a list of column definitions
pub fn parse_columns(columns: &[ColumnDef]) -> Vec<Column> {
  columns.iter().map(parse_column).collect()
}
// Parse a single column definition
pub fn parse_column(column_def: &ColumnDef) -> Column {
  Column {
    name: column_def.name.to_string(),
    data_type: data_type_to_string(&column_def.data_type),
    nullable: !column_def
      .options
      .iter()
      .any(|opt| matches!(opt.option, sqlparser::ast::ColumnOption::NotNull)),
  }
}
