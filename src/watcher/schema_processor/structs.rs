// Struct to represent a column in a table
#[derive(Debug, Clone)]
pub struct Column {
  pub name: String,
  pub data_type: String,
  pub nullable: bool,
}

// // Struct to represent a table
#[derive(Debug)]
pub struct Table {
  pub name: String,
  pub columns: Vec<Column>,
}
