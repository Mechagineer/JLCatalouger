//! Ruled-table extractor skeleton: detect grid lines & extract cells.
// TODO: PDF vector ops + text; return rows as Vec<Vec<String>>.

#[allow(dead_code)]
pub struct Table { pub page: u32, pub rows: Vec<Vec<String>> }

#[allow(dead_code)]
pub fn extract_simple_tables(_pdf_path: &str) -> Vec<Table> {
  // TODO: implement minimal ruled-table detection.
  vec![]
}
