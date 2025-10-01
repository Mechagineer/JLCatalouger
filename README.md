# JLCatalouger — Catalog Reader/Compiler + Configurator (Windows, Offline)

**JLCatalouger** is a brand-agnostic, offline desktop toolchain that:
- **Reads** industrial gearmotor-style **catalog PDFs**,
- **Compiles** them into versioned, explainable **Catalog Packs** (structured data + rules + assets), and
- **Configures** valid product selections with **mutual gating**, **nearest-match**, and **explanations**.

> Current target: **Windows 10/11** (offline).  
> Design is **brand-agnostic** via **Brand Packs** (mappings, transforms, rules, curations).

---

## What this does

- **Reader/Compiler**: Deterministic PDF ingestion → table extraction → rule scraping → mapping & transforms → normalization → validation → **Pack** build (with provenance & coverage).
- **Configurator**: Loads Packs, applies rules, enables any-order filtering (**mutual gating**) and **nearest-match** on numeric inputs, with explain-why and provenance.

Everything runs **locally**. No cloud accounts, APIs, or subscriptions required.

---

## Key Features

- **Deterministic PDF parsing**, **OCR fallback** for scans; optional **local ML (ONNX Runtime/DirectML)** to assist on messy, borderless tables.
- **Brand Packs** for system-agnostic mapping: `mapping.yaml`, `transforms/*`, `rules/*.yaml`, `curations.yaml`.
- **Canonical Ontology** (brand-agnostic) feeding normalized CSVs.
- **Explainability**: every output has **provenance** (page/table/cell) and **confidence**; coverage report for acceptance.
- **Configurator UX**: mutual gating across all facets; nearest-match for numeric fields; spec export.
- **Offline-first packaging** for Windows (MSI).

---

## Architecture (at a glance)

**Reader/Compiler Pipeline**
1. Section Indexing  
2. Table Extraction  
3. Narrative Rule Scraping  
4. Brand Mapping & Transforms  
5. Curations Overlay  
6. Normalization & Validation  
7. (Optional) Asset Slicing  
8. Pack Build + Diff Report  
9. Provenance & Coverage

**Configurator (Consumer)**
- Loads canonical CSVs + rules; builds facet indexes.
- Enforces hard constraints; proposes nearest admissible values.
- Shows pass/fail rule badges and provenance-backed explanations.

See **plan.md** for the full architecture, performance targets, and end-to-end user flow.

---

## Data Contracts (Packs)

Packs are versioned folders that include:

- `data/*.csv` (aligned to the ontology)
  - `families.csv` — Powertrain family enums
  - `sizes.csv` — size.code / series / optional torque.max per size
  - `ratios.csv` — stage, ratio.i, admissible, notes
  - `shafts.csv` — shaft.style, D, D1/D2, L1/L2, fit, optional_flag
  - `mountings.csv` — mounting.code, pivoted flags, notes
  - `motors.csv` — poles, hp/kw, n_sync, optional notes
  - `limits.csv` — overhung/axial limits/consts, thermal flags, optional efficiency
  - *(optional)* `geometry.csv` — brand-mapped extras for UI previews
- `rules/*.yaml` — selection, mounting, loads, thermal
- `assets/` — optional cropped drawings/mounting sheets
- `provenance.jsonl` — per-field anchors + confidence
- `coverage.json` — parse coverage summary
- `pack.yaml` — manifest (name, version, effective_date, checksum, coverage)

See **plan.md → Data Contracts** for column details.

---

## Tech Stack (Windows, Offline)

- **Shell**: Tauri (React + TypeScript front-end, Rust backend)
- **Core**: Rust (engine for parsing/rules/selection)
- **PDF**: PDFium (preferred)
- **OCR**: Tesseract (fallback for scanned pages)
- **Storage**: SQLite (provenance/cache), CSV/YAML for pack data; DuckDB optional
- **Optional ML**: ONNX Runtime (DirectML) for small layout/table assists

See **techstack.md** for the full stack and rationale.

---

## Status & Roadmap

We follow a **spec-first** loop (research → plan → implement → test):

- **Phase A**: Reader/Compiler (core)  
- **Phase B**: Canonicalization & Rules  
- **Phase C**: Configurator MVP  
- **Phase D**: Tests (goldens, property, coverage)  
- **Phase E**: Competitor expansion via Brand Packs

See **plan.md → Work Plan** and **Acceptance Criteria**.

---

## Security & Privacy

- 100% **offline**; no uploads.  
- Deterministic builds with versioned outputs.  
- Optional pack signing via checksums in `pack.yaml`.

---

## License

TBD.
