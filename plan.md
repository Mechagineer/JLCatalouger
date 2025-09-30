# Plan — Catalog Reader/Compiler + Configurator (Brand-Agnostic)

## Objectives
- Ingest diverse catalogs → emit brand-agnostic **Catalog Packs** (data + rules + assets + provenance).
- Build a **Configurator** that consumes Catalog Packs with **mutual gating**, **nearest-match**, and **explainable** validations.
- Enable competitor onboarding via **Brand Packs** (mapping, transforms, rules, curations) with minimal code changes.

## Deliverables
1. **Reader/Compiler CLI**
   - Outputs `/packs/<brand>_<catalog>_<date>/`
     - `data/*.csv` (families, sizes, ratios, shafts, mountings, motors, dimensions, limits)
     - `rules/*.yaml` (selection, mounting, loads, thermal)
     - `assets/` (cropped drawings/mounting sheets if extractable)
     - `pack.yaml` (manifest), `provenance.jsonl`, `coverage.json`
2. **Canonical Ontology v0** (this repo) and **Brand Pack template** (`mapping.yaml`, `rules.yaml`, `curations.yaml`, `transforms/*`)
3. **Configurator MVP** (desktop): mutual gating, nearest-match, rule badges, explanations, spec export
4. **Tests**: schema validators, golden samples, coverage thresholds, crossover tests
5. **Docs**: updated `research.md`, this `plan.md`, and `progress.md` workflow guidance

## Architecture

### Reader/Compiler Pipeline
- **Section Indexer** → identify headings/chapters (configuration procedure, mounting positions, selection tables, dimensions, loads).
- **Table Extractor** → parse structured tables (ratios; shaft D/D1/D2; L1/L2; dimensions; limits).
- **Rule & Formula Scraper** → detect selection flow, compound/torque relations, mounting caveats, overhung/axial methods, thermal guidance.
- **Field Mapper & Normalizer** → map to canonical ontology; unit normalization; enums.
- **Asset Slicer** → crop and key drawings/mounting sheets (optional, if vector quality allows).
- **Provenance Linker** → attach `{file, page, cell/line, text, confidence}` to every datum.
- **Validator & QA** → schema checks; cross-rule checks (stage legality, mounting allowances, optional-shaft effects); golden comparisons.
- **Pack Builder & Versioner** → deterministic outputs; diffs across versions.

### Configurator (Consumer)
- **Engine**: applies rules to candidate sets; performs **mutual gating** and **nearest-match** within admissible values; runs heavy checks (loads/thermal) off the UI thread.
- **UI**: facets for family/size/ratio/torque/motor/shaft/mounting; explanations with provenance; spec export.
- **Brand-Agnostic**: only reads canonical fields; brand-specific labels injected via Brand Packs for display.

### Brand Packs (Mapping Layer)
- `mapping.yaml` (synonyms, field maps, UI labels); `transforms/*`; `rules.yaml`; `curations.yaml`.
- Human-in-the-loop review UI for low-confidence parses; corrections become durable mapping rules.

## Data Contracts (v0)
- `ratios.csv`: `family`, `stage`, `i`, `admissible`, `notes`
- `shafts.csv`: `family|size`, `style`, `D`, `D1/D2`, `L1/L2`, `fit`, `optional_flag`
- `mountings.csv`: `family|size`, `M1..M6`, `MX`, `M0`, `pivoted_dynamic`, `pivoted_stationary`, `notes`
- `motors.csv`: `family|size_range`, `poles`, `hp`, `kw`, `n_sync`, `inverter_notes`
- `dimensions.csv`: `variant_id`, `AC`, `L`, `LS`, `LB`, `flange_d`, …
- `limits.csv`: `family|size`, `torque.max`, `overhung.constants`, `axial_limits`, `thermal_flags`
- `pack.yaml`: manifest {name, version, effective_date, checksum, coverage}
- `provenance.jsonl`: per-row anchors; `coverage.json`: parse coverage report

## Work Plan (Phases)

**Phase A — Reader/Compiler (Core)**
- Implement section indexer, table extractor, rule scraper, field normalizer, asset slicer, provenance linker, validators.
- Exit criteria: build a pack from one real catalog with ≥90% table coverage; all rows have provenance; schema validators pass.

**Phase B — Canonicalization & Rules**
- Lock ontology v0; author shared rules DSL snippets for selection flow, compound/torque relations, mounting behaviors, and load methods (brand-agnostic defaults; brand packs can add specifics).

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
- Build crossover tests for “nearest-equivalent” logic; verify explanations.

## Acceptance Criteria
- Reader outputs deterministic packs with provenance and ≥90% coverage.
- Configurator performs mutual gating and nearest-match strictly within admissible sets, with clear explanations.
- Brand Packs enable onboarding of a new catalog with only mapping/rules/curations—no core changes.
- Tests (goldens, properties, coverage, crossover) all pass.

## Risks & Mitigations
- **Nomenclature divergence** → Brand Packs (synonyms/transforms/curations).
- **Missing data** → mark unknowns; require user acknowledgment or conservative defaults.
- **Irregular tables/OCR noise** → confidence scoring + review UI + durable parser hints.
- **Thermal/edge behaviors** → model as flags/conditions and require inputs when needed.

## Workflow & Tracking
- Spec-first loop with `research.md` → `plan.md` → `progress.md` updates per milestone.


