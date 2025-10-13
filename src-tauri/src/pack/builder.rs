//! Pack builder helpers: deterministic CSV writing + basic escaping.

use std::{fs, io::Write, path::Path};

#[allow(dead_code)]
pub fn write_csv(path: &Path, header: &str, rows: &[Vec<String>]) -> Result<(), String> {
  let mut f = fs::File::create(path).map_err(|e| format!("create {:?}: {e}", path))?;
  f.write_all(header.as_bytes()).map_err(|e| format!("write {:?}: {e}", path))?;
  for r in rows {
    let line = r.iter().map(|s| csv_escape(s)).collect::<Vec<_>>().join(",") + "\n";
    f.write_all(line.as_bytes()).map_err(|e| format!("write row {:?}: {e}", path))?;
  }
  Ok(())
}

fn csv_escape(s: &str) -> String {
  if s.contains(',') || s.contains('"') || s.contains('\n') {
    format!("\"{}\"", s.replace('"', "\"\""))
  } else { s.to_string() }
}
