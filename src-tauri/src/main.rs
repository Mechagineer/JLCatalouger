
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use tauri::{Manager};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use chrono::Local;
use anyhow::{Result, Context};

#[tauri::command]
fn build_smoke_pack(input_pdf: String, packs_root: Option<String>) -> Result<String, String> {
  // Resolve output root
  let root = packs_root.unwrap_or_else(|| "packs".to_string());
  let ts = Local::now().format("%Y-%m-%d_%H%M%S").to_string();
  let out_dir = Path::new(&root).join(format!("Smoke_{}", ts));

  // Create folders
  let data_dir = out_dir.join("data");
  let assets_dir = out_dir.join("assets");
  fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
  fs::create_dir_all(&assets_dir).map_err(|e| e.to_string())?;

  // Copy input into assets
  let src = PathBuf::from(&input_pdf);
  let dest = assets_dir.join("source.pdf");
  fs::copy(&src, &dest).map_err(|e| format!("Copy failed: {e}"))?;

  // Write CSV headers (no rows yet)
  write_file(data_dir.join("families.csv"), "family\n")?;
  write_file(data_dir.join("sizes.csv"), "family,size,mamax\n")?;
  write_file(data_dir.join("ratios.csv"), "family,stage,i,admissible,notes\n")?;
  write_file(data_dir.join("shafts.csv"), "family,size,style,D,D1,D2,L1,L2,fit,optional_flag\n")?;
  write_file(data_dir.join("mountings.csv"), "family,size,mounting_code,pivoted,notes\n")?;
  write_file(data_dir.join("motors.csv"), "poles,hp,kw,n_sync,notes\n")?;
  write_file(data_dir.join("limits.csv"), "family,size,overhung,axial,thermal_flag\n")?;

  // coverage.json
  let coverage = r#"{
    "tables_found": 0,
    "tables_parsed": 0,
    "rows_emitted": 0,
    "notes": "smoke run only; no parsing",
    "started_at": null,
    "finished_at": null
  }"#;
  write_file(out_dir.join("coverage.json"), coverage)?;

  // provenance.jsonl (empty stub for now)
  write_file(out_dir.join("provenance.jsonl"), "")?;

  // pack.yaml
  let pack_yaml = format!(
    "name: SmokePack\nversion: 0.0.0\ncreated: {}\nsource_pdf: assets/source.pdf\nschema: v0\n",
    Local::now().to_rfc3339()
  );
  write_file(out_dir.join("pack.yaml"), &pack_yaml)?;

  Ok(out_dir.to_string_lossy().to_string())
}

fn write_file(p: PathBuf, content: &str) -> Result<(), String> {
  let mut f = fs::File::create(&p).map_err(|e| format!("create {:?}: {e}", p))?;
  f.write_all(content.as_bytes()).map_err(|e| format!("write {:?}: {e}", p))?;
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![build_smoke_pack])
    .setup(|_app| Ok(()))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
