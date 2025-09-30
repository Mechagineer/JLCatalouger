# Plan — Catalog Reader/Compiler + Configurator (Brand-Agnostic)

## Objectives
- Ingest diverse catalogs → emit brand-agnostic **Catalog Packs** (data + rules + assets + provenance).
- Build a **Configurator** that consumes Catalog Packs with **mutual gating**, **nearest-match**, and **explainable** validations.
- Enable competitor onboarding via **Brand Packs** (mapping, transforms, rules, curations) with minimal code changes.

## Deliverables
1. **Reader/Compiler CLI**
   - Outputs `/packs/<brand>_<catalog>_<date>/`
     - `data/*.csv` (aligned to the ontology)
       - `families.csv`       # Powertrain: family enums; optional subtype
       - `sizes.csv`          # Powertrain: size.code, size.series, torque.max (if per-size)
       - `ratios.csv`         # Powertrain: stage, ratio.i, admissible, notes
       - `shafts.csv`         # Output End: shaft.style, D, D1/D2, L1/L2, fit, optional_flag
       - `mountings.csv`      # Mounting: mounting.code, pivoted_dynamic, pivoted_stationary, notes
       - `motors.csv`         # Motor: poles, hp, kw, n_sync; optional per-size range
       - `limits.csv`         # Loads & Limits: overhung.constants, overhung/axial.limits, thermal.flags, efficiency.eta_tot (if available)
       - (optional) `geometry.csv`  # brand-mapped geometric fields not in the core ontology, used only for UI previews; contents vary by Brand Pack
     - `rules/*.yaml` (selection, mounting, loads, thermal)
     - `assets/` (cropped drawings/mounting sheets if extractable)
     - `pack.yaml` (manifest), `provenance.jsonl`, `coverage.json`
2. **Canonical Ontology v0** (this repo) and **Brand Pack template** (`mapping.yaml`, `rules.yaml`, `curations.yaml`, `transforms/*`)
3. **Configurator MVP** (desktop): mutual gating, nearest-match, rule badges, explanations, spec export
4. **Tests**: schema validators, golden samples, coverage thresholds, crossover tests
5. **Docs**: updated `research.md`, this `plan.md`, and `progress.md` workflow guidance

## Architecture

### Reader/Compiler Pipeline
- **Section Indexer** → identify headings/chapters (configuration procedure, mounting positions, selection tables, dimensions if present, loads).
- **Table Extractor** → parse structured tables (ratios; shaft D/D1/D2; L1/L2; dimensions; limits).
- **Rule & Formula Scraper** → detect selection flow, compound/torque relations, mounting caveats, overhung/axial methods, thermal guidance.
- **Field Mapper & Normalizer** → map to canonical ontology; unit normalization; enums.
- **Asset Slicer** → crop and key drawings/mounting sheets (optional, if vector quality allows).
- **Provenance Linker** → attach `{file, page, cell/line, text, confidence}` to every datum.
- **Validator & QA** → schema checks; cross-rule checks (stage legality, mounting allowances, optional-shaft effects); golden comparisons.
- **Pack Builder & Versioner** → deterministic outputs; diffs across versions.

### Performance & Reliability

- **Core**: compiled library (Rust or similar), parallelized table parsing, streaming I/O (no giant in-memory PDFs).
- **Targets**:  
  - Section indexing < 10s for 400–800 page catalog.  
  - Table extraction 20–60s (parallelized).  
  - Mapping/normalize/validate < 10s.  
  - Full import usually < 2 min; re-imports faster with caching.
- **Caching**:  
  - Section Map cache per catalog fingerprint.  
  - Parser hints from prior fixes auto-applied.  
  - Asset cache for cropped drawings.
- **Resilience**:  
  - Crash-safe checkpoints between stages.  
  - Resume button for interrupted imports.  
  - Output uses temp folders with atomic rename on success.  
- **Storage**:  
  - Packs written to `/packs/<brand>_<catalog>_<date>/`.  
  - SQLite holds provenance, coverage, compiled rule graphs.

### Configurator (Consumer)
- **Engine**: applies rules to candidate sets; performs **mutual gating** and **nearest-match** within admissible values; runs heavy checks (loads/thermal) off the UI thread.
- **UI**: facets for family/size/ratio/torque/motor/shaft/mounting; explanations with provenance; spec export.
- **Brand-Agnostic**: only reads canonical fields; brand-specific labels injected via Brand Packs for display.

### Brand Packs (Mapping Layer)
- `mapping.yaml` (synonyms, field maps, UI labels); `transforms/*`; `rules.yaml`; `curations.yaml`.
- Human-in-the-loop review UI for low-confidence parses; corrections become durable mapping rules.

### End-to-End User Flow (Standalone App)

1. **Home Screen**
   - Import Catalog button + drag-and-drop zone.
   - List of recent imports with status (Complete, In Progress, Needs Review).

2. **Preflight**
   - Show file metadata (page count, fingerprint hash).
   - Select Brand Pack (template or existing).
   - Optional: set catalog name, effective date, output folder.
   - Validation: confirm file readable, Brand Pack schema valid, space available.

3. **Import & Compile**
   - Live progress stepper:
     - Section Indexing
     - Table Extraction
     - Rule Scrape
     - Mapping & Transforms
     - Curations Overlay
     - Normalization
     - Validation
     - Asset Slicing
     - Pack Build
     - Diff Report
   - Logs visible in a detail pane.

4. **Review (if needed)**
   - Side-by-side PDF snippet vs. parsed row.
   - Quick actions: split/merge cell, override unit, add synonym.
   - Fixes persist into the Brand Pack (mapping, transform, curation).

5. **Done**
   - Coverage %, diff summary, error count.
   - Buttons: Open Pack, Open in Configurator, Export Report.

---

## Data Contracts (v0)

* `families.csv`
  * Columns: `family`, `subtype?`, `description?`

* `sizes.csv`
  * Columns: `size.code`, `size.series`, `torque.max?`, `notes?`

* `ratios.csv`
  * Columns: `family`, `stage`, `ratio.i`, `admissible`, `notes?`

* `shafts.csv`
  * Columns: `family|size`, `shaft.style`, `shaft.D`, `shaft.D1?`, `shaft.D2?`, `shaft.L1`, `shaft.L2`, `shaft.fit?`, `shaft.optional_flag`

* `mountings.csv`
  * Columns: `family|size`, `mounting.code`, `pivoted_dynamic?`, `pivoted_stationary?`, `notes?`

* `motors.csv`
  * Columns: `family|size_range?`, `motor.poles`, `motor.hp?`, `motor.kw?`, `motor.n_sync?`, `inverter_notes?`

* `limits.csv`
  * Columns: `family|size`, `overhung.constants` (a,b,c,f,d,l as JSON), `overhung.limits?`, `axial.limits?`, `thermal.flags?`, `efficiency.eta_tot?`

* (optional) `geometry.csv`
  * Columns: brand-mapped geometric fields **outside** the core ontology, used for UI previews only; schema is Brand Pack–specific

* `pack.yaml` (manifest), `provenance.jsonl` (per-row anchors), `coverage.json` (parse coverage)

## Work Plan (Phases)

**Phase A — Reader/Compiler (Core)**
- Implement section indexer, table extractor, rule scraper, field normalizer, asset slicer, provenance linker, validators.
- Exit criteria: build a pack from one real catalog with ≥90% table coverage; all rows have provenance; schema validators pass.

**Phase B — Canonicalization & Rules**
- Lock ontology v0; author shared rules DSL snippets for selection flow, compound/torque relations, mounting behaviors, load methods; Brand Packs can extend.

**Phase C — Configurator MVP**
- Desktop app (tech of choice) + headless engine.
- Mutual gating, nearest-match, rule badges, explainability, spec export.
- Heavy checks in background workers.

**Phase D — Tests**
- **Goldens**: 15–25 configurations (torque/speed-driven and geometry-driven).
- **Property tests**: nearest-match never returns inadmissible values; mutual gating never exposes illegal combos; rule explanations always carry provenance.
- **Coverage**: minimum threshold; diff report between pack versions.

**Phase E — Competitor Expansion**
- Add first Brand Pack with `mapping.yaml`, transforms, rules, curations.
- Build crossover tests for nearest-equivalent logic; verify explanations.

## Acceptance Criteria
- Reader outputs deterministic packs with provenance and ≥90% coverage.
- Configurator performs mutual gating and nearest-match strictly within admissible sets, with clear explanations.
- Every recommendation includes an explanation citing which constraints matched or were relaxed, with provenance.
- End-to-end import flow is fully offline, interactive, and resilient (pause/resume, crash-safe).
- Brand Packs enable onboarding of a new catalog with only mapping/rules/curations—no core changes.
- Tests (goldens, properties, coverage, crossover) all pass.

## Risks & Mitigations
- **Nomenclature divergence** → Brand Packs (synonyms/transforms/curations).
- **Missing data** → mark unknowns; require user acknowledgment or conservative defaults.
- **Irregular tables/OCR noise** → confidence scoring + review UI + durable parser hints.
- **Thermal/edge behaviors** → model as flags/conditions and require inputs when needed.

## Workflow & Tracking
- Spec-first loop with `research.md` → `plan.md` → `progress.md` updates per milestone.


