# Catalog Reader/Compiler — Design Doc

## 1) Goal & Scope

Transform SEW-style gearmotor catalogs (PDF) into versioned **Catalog Packs** consumable by the DRN Configurator:

* **Data**: families, sizes, ratios, shaft/dimension tables, mountings, motors, limits.
* **Rules**: selection flow, compound/torque formulas, mounting & load rules.
* **Assets**: cropped drawings and mounting sheets.
* **Provenance**: page/line/table anchors for every datum.

Ground truths from the DRN catalog the reader must capture:

* **Selection flow** (kinematics → torque → gear unit → motor → brake → inverter).
* **Compound gear units**: limit motor by Mamax; compute **MMot**; **MB ≤ 200%·MMot**.
* **Families & ratio spans** (e.g., R series i≈1.30→289.74, up to ≈27001 compound) and stage legality by family.
* **Shaft dimensions & optional shafts** affecting Mamax/overhung.
* **Mounting positions** M1–M6 (+ MX, M0, variable/pivoted) with breather/oil-fill adjustments.
* **Overhung/axial method** including transmission factors and constants tables.

---

## 2) Inputs & Outputs

### Inputs

* One or more PDFs (vector text+tables+drawings), e.g., **31964079/EN – 05/2025**.

### Outputs (Catalog Pack)

```
/DRN-IE3_31964079_2025-05/
  pack.yaml                # manifest {name, version, effective_date, checksum, coverage}
  data/
    families.csv           # family, subtype, description
    sizes.csv              # size, family, Mamax, torque_envelope_id
    ratios.csv             # family, stage, i, admissible, notes
    shafts.csv             # family|size, style, D, D1, D2, L1, L2, fit, optional_flag
    mountings.csv          # family|size, M1..M6/MX/M0 allowed, special_notes
    motors.csv             # family|size (range), poles, hp, n_sync, inverter_notes
    dimensions.csv         # variant_id, AC, L, LS, LB, flange_d, ...
    limits.csv             # family|size, overhung_consts, axial_limits, thermal_flags
  rules/
    selection.yaml         # flow and formulas (see §6)
    mounting.yaml          # breather/oil rules, pivoted/MX/M0 specifics
    loads.yaml             # overhung/axial method + transmission factors
    thermal.yaml           # checks and references
  assets/
    drawings/...           # cropped dimension/mounting sheets
  provenance.jsonl         # per-row source anchors {file, page, lines|table_cell, text}
  coverage.json            # parse coverage by section/table
```

---

## 3) Architecture

**Pipeline with human-in-the-loop review** (modular stages, idempotent):

1. **PDF Section Indexer**

   * Detect chapter/heading boundaries (e.g., “Configuration procedure”, “Mounting positions”, “Selection tables”).
   * Produce a **Section Map** with page ranges for targeted parsers.

2. **Table Extractor**

   * Parse structured tables (shaft dims with **D/D1/D2/L1/L2**, dimension sheets, ratios/steps, etc.).
   * Handle merged cells, multi-page table continuation, units. The DRN dimension tables explicitly label L1/L2 and show consistent column headers.

3. **Rule & Formula Scraper**

   * Extract and normalize formula blocks and rule statements for the **selection flow**, **compound limits** (MMot, MB limit), **mounting adjustments**, **overhung/axial** method.

4. **Field Mapper & Normalizer**

   * Map parsed content to the pack schema with unit canonicalization (mm, Nm, rpm, °C) and enums (families R/F/K/S/W; stages 1/2/3; M1–M6/MX/M0).
   * Enforce *family–stage* legality from the catalog’s product characteristics matrices.

5. **Asset Slicer**

   * Crop vector drawings for **dimension/mounting sheets** to SVG/PNG, generate `variant_id` → asset mapping for UI previews.

6. **Provenance Linker**

   * For every CSV/YAML row, attach `{pdf, page, line-range | table-id/cell, phrase}` so the app can “explain source.”

7. **Validator & QA**

   * Schema validation, completeness checks, and **golden sample** comparisons.
   * Cross-rule checks: e.g., **optional shafts** set `optional_flag` and may **derate Mamax/overhung**; ensure the limits rows reflect that.
   * Overhung constants tables round-trip correctly into `limits.csv`.

8. **Pack Builder & Versioner**

   * Deterministic file ordering and hashes; semantic version from `pack.yaml` (`_2025-05`).
   * Diff engine vs. previous pack to surface lifecycle changes (added/removed ratios, dimensions, mountings).

---

## 4) Data Contracts (key CSVs)

### `ratios.csv`

* **Columns**: `family`, `stage` (1/2/3), `i` (float), `admissible` (bool), `notes`
* **Derivation**: From family ratio sections & selection tables; include R-series range i≈1.30..289.74; compound noted in `notes`.

### `shafts.csv`

* **Columns**: `family|size`, `style` (solid/hollow/spline), `D`, `D1`, `D2`, `L1`, `L2`, `fit`, `optional_flag`
* **Notes**: Set `optional_flag=true` when table/paragraph marks “optional output shafts”; propagate warning to `limits`.

### `mountings.csv`

* **Columns**: `family|size`, `M1..M6` (bools), `MX` (bool), `M0` (bool), `pivoted_dynamic`, `pivoted_stationary`, `notes`
* **Rules YAML** handles: oil fill changes, breather location, and “contact SEW” cases for certain changes (e.g., M4).

### `limits.csv`

* **Columns**: `family|size`, `Mamax`, `overhung_consts` (a,b,c,f,d,l JSON blob), `axial_limits`, `thermal_flags`
* **Source**: Overhung constant tables; thermal/compound notes; set flags for MX/M0 requirements.

### `selection.yaml` (Rules)

* **Flow**: Kinematics → dynamic/static torque → **select gear unit by max torque** → choose **ratio** (motor type known) → select **motor** by max/effective operating point → **brake** selection → **inverter** (rated power & currents).
* **Compound**: `MMot = Mamax * i_tot / η_tot` and **MB ≤ 200%·MMot**.

---

## 5) Algorithms & Heuristics

* **Section Detection**: regex on heading fonts/TOC, plus page anchors (e.g., “Configuration procedure”, “Mounting positions”).
* **Table Parsing**:

  * Column header detection via repeated patterns (e.g., `A1, D, D1, D2, D3, F1, I2, L1, L2`)—consistent in DRN tables.
  * Multi-line cell merge; hyphen ranges (e.g., `25 / 30`) normalized to lists.
* **Unit Normalization**: mm/Nm/rpm/°C; infer from headers/footers (appendix/glossary).
* **Rule Extraction**: sentence patterns for “must/shall/requires” + inline math capture (e.g., MB ≤ 200% MMot).
* **Confidence Scoring**: per cell/paragraph; low scores routed to review UI.

---

## 6) Human Review UI (lightweight)

* Side-by-side **PDF snippet** ↔ parsed row.
* Quick fixes (cell split/merge, unit override, optional_flag toggle).
* Every fix produces a **parser hint** (rule) persisted and re-applied on future runs—keeps the pipeline stable as catalogs evolve.

---

## 7) Validation & QA

* **Schema checks**: required columns present, enums valid.
* **Cross-checks**:

  * Family ↔ stage legality per catalog (e.g., RX 1-stage; R/F 2–3; K 2–3; S 2; W 1–3).
  * Optional shaft rows must set `optional_flag` and generate a **limits** delta for Mamax/overhung.
* **Goldens**: hand-verified samples from representative tables (e.g., RF/KAF/SAF pages).
* **Coverage**: report percent of expected tables/sections parsed; fail build if below threshold.

---

## 8) Performance & Reliability

* Streaming PDF parse; memory-bounded table buffering.
* Deterministic outputs (stable row ordering, canonical float formatting).
* Incremental re-builds: only re-parse changed sections; reuse prior **parser hints**.

---

## 9) Versioning & Lifecycle

* Pack version from catalog edition/date (e.g., `DRN-IE3_31964079_2025-05`).
* **Diff report**: added/removed ratios, mountings, sizes, shaft options; changed constants.
* Deprecations captured as `admissible=false` rows rather than deletes—preserves historical configs.

---

## 10) Risks & Mitigations

* **OCR noise / scanned pages**: fall back to table tracing + manual hints; keep confidence scores.
* **Irregular tables**: expand the hints DSL (row/col spans, header aliases).
* **Rule ambiguity** (e.g., mounting change caveats): store exact paragraphs in rules YAML `source` fields; surface to user at selection time.

---

## 11) Deliverables

1. **Parser binary** (CLI): `catalog-reader ingest NEW_DRN_GEARMOTOR_CATALOG_31964079.pdf --out ./packs/`
2. **Pack folder** (as above).
3. **Coverage & diff reports**.
4. **Parser hints file** (grows with human fixes).

---

## 12) Acceptance Criteria

* For DRN 31964079, produce a pack where:

  * `ratios.csv` covers documented ranges/stages for each family.
  * `shafts.csv` includes **D/D1/D2/L1/L2** with optional flags where applicable.
  * `mountings.csv` encodes M1–M6/MX/M0 and notes for breather/oil changes & pivoted options.
  * `rules/selection.yaml` contains the calculation flow and compound MMot/MB constraints.
  * `limits.csv` includes overhung constants tables and thermal flags.

---
