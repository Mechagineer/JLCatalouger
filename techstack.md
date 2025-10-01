# Tech Stack — Catalog Reader/Compiler + Configurator (Windows, Offline)

This document defines the technologies and components used to build the **Catalog Reader/Compiler** and **Configurator** as a standalone, brand-agnostic desktop app.

---

## 0. Prerequisites
- Windows 10/11 x64
- ≥16 GB RAM recommended
- Local administrator rights for installing dependencies

---

## 1. Build Toolchains
- **Visual Studio Build Tools 2022** (MSVC, Windows SDK, C++ workload)
- **Rust toolchain** (stable, MSVC target)
- **Node.js LTS + pnpm** (for Tauri + React/TypeScript front-end)
- **Tauri CLI** (app bundling and dev runner)
- **CMake + Ninja** (for native libraries used by some Rust crates)

---

## 2. Project Skeleton
- **Tauri shell** (React + TypeScript UI + Rust backend)
- **Rust core crate** (shared Reader/Compiler + Configurator engine)
- **Brand Pack files** (mappings, transforms, rules, curations)
- **Output Catalog Packs** (`/packs/` folder with CSVs, YAML rules, assets, provenance)

---

## 3. PDF Ingestion
- **PDFium** (preferred) or **Poppler** for:
  - Reading native text layer
  - Extracting vector lines and images
  - Rendering pages for OCR fallback

---

## 4. OCR (Fallback)
- **Tesseract OCR (UB Mannheim build for Windows)**
  - Used only for image-only or scanned pages
  - Provides local, offline OCR with language packs

---

## 5. Optional CV & ML Assists
- **OpenCV** (Windows prebuilt DLLs):
  - Detect table lines
  - Assist with borderless table segmentation
- **ONNX Runtime (ORT)** with **DirectML**:
  - Lightweight local models for page layout detection
  - Cell proposals in messy tables
  - Optional header/label classification
- All ML features run fully **offline**.

---

## 6. Storage & Data Outputs
- **SQLite**: embedded DB for provenance, coverage, and caches
- **CSV**: normalized data tables (`families.csv`, `sizes.csv`, `ratios.csv`, etc.)
- **YAML/JSON**: rules, pack manifests, attribute metadata
- **DuckDB**: optional, for high-speed faceted queries on large catalogs

---

## 7. Front-End (Configurator UI)
- **React + TypeScript** UI inside Tauri
- **State management**: Zustand (light) or Redux Toolkit (richer ecosystem)
- **UI components**: shadcn/ui or Mantine
- **Charts**: Recharts or Chart.js
- **Features**:
  - Mutual gating
  - Nearest-match on numeric fields
  - Provenance explanations with sources
  - Spec export (PDF or CSV)

---
