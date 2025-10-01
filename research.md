# Research — Catalog Reader/Compiler + Configurator (Brand-Agnostic)

## Purpose
Define the domain, entities, formulas, and data needed to ingest industrial gearmotor-style catalogs (PDFs or similar) into structured, brand-agnostic **Catalog Packs**, and specify the behavior the **Configurator** needs (mutual gating, nearest match, explainability).

## Program Scope & Sequence
1) Build a **Catalog Reader/Compiler** that ingests catalogs → emits **Catalog Packs** (data + rules + assets + provenance).
2) Build a **Configurator** that consumes Catalog Packs with **mutual gating** and **nearest-match** behaviors.
3) Validate MVP with golden samples; then add competitor “Brand Packs” via mappings and curations to enable crossover.
4) Iterate.

## Canonical Ontology (v0) — Brand-Agnostic Target Schema
Keep the ontology compact and durable. All brands map into this vocabulary.

- **Powertrain**
  - `family` (enum: helical; parallel-shaft helical; helical-bevel; helical-worm; right-angle/other)
  - `stage` (1/2/3; `compound_flag`)
  - `ratio.i` (float), `ratio.series_steps` (list)
  - `size.code` (string), `size.series` (int)
  - `torque.nominal`, `torque.max`

- **Motor**
  - `motor.poles`, `motor.hp`, `motor.kw`

- **Output End**
  - `shaft.style` (solid/hollow/shrink_disc)
  - `shaft.dim`, `shaft.D1/D2`, `shaft.L1/L2`

- **Mounting**
  - `mounting.code` (e.g., M1..M6/MX/M0 or brand codes mapped to these)
  - `mounting.style` (e.g., Flanged, Footed, Torque Arm or brand codes mapped to these)

- **Metadata**
  - `brand`, `catalog_version`, `provenance` (page/line/table cell), `admissible`, `deprecation`

## Core Procedures & Rules to Capture (Brand-Agnostic)
- **Selection flow gearbox**: Output end → ratio → mounting style 
- **Selection flow motor**: motor hp/kw → motor poles

## Data Field Inventory (maps to ontology)
- `family`, `stage`, `ratio.i`, `size.code`, `torque.nominal`, `torque.max`
- `motor.poles`, `motor.hp`, `motor.kw`
- `shaft.style`, `shaft.dim`
- `mounting.code`, `mounting.style` 
- `brand`, `catalog_version`, `provenance`, `admissible`, `deprecation`

## Brand-Agnostic Ingestion Strategy
- The Reader/Compiler never hard-codes brand nouns.
- Introduce **Brand Packs** (mapping + transforms + rules + curations) to map source catalogs into the canonical ontology.

**Brand Pack structure (concept)**
- `mapping.yaml`: term/code → canonical field mapping; synonyms for enums (e.g., mounting codes); UI labels.
- `transforms/*`: small transformations (e.g., split “25/30” into `[25,30]`, normalize units).
- `rules.yaml`: brand-specific constraints expressed in a common rules DSL.
- `curations.yaml`: human-approved overrides for edge cases.
- All values carry `{file, page, cell/line, confidence}` provenance.

## PDF Ingestion & Extraction (Research Overview)

The Reader/Compiler must be able to ingest catalogs delivered as PDFs in a repeatable and offline-safe way:

1. **Deterministic parsing first**
   - Use embedded PDF libraries to extract native text, vector lines, and images.
   - Reconstruct words, paragraphs, and headings based on positioning and font data.

2. **Table detection**
   - Identify ruled tables via vector line grids.
   - Identify borderless tables via whitespace and alignment clustering.
   - Resolve merged cells and multi-page continuation.

3. **Narrative rule scraping**
   - Identify paragraphs with selection rules or formulas (must/shall/limited).
   - Extract these into normalized rule snippets with provenance anchors.

4. **OCR fallback (only when needed)**
   - If a page has no text layer, render at high DPI and run local OCR.
   - Use simple vision ops for cell segmentation on scanned tables.
   - Flag OCR-derived data with lower confidence.

5. **Brand mapping & transforms**
   - Apply mapping.yaml (field headers → canonical), synonyms, transforms, and curations.
   - All transformations are data-driven and auditable.

6. **Optional local ML assist**
   - For very messy borderless tables, a small on-device model can suggest cell boxes.
   - Used only for geometry detection; semantics still come from mappings and rules.

7. **Provenance & coverage**
   - Every extracted field is logged with `{file, page, section, cell/line, original_text, confidence}`.
   - Coverage reports summarize what proportion of expected fields were successfully parsed.

This ensures catalogs can be ingested deterministically, with explainable outputs, and without reliance on remote services.

## Cross-Brand "Nearest-Equivalent" Logic
- **Hard constraints**: mounting style compatibility; ratio equivalent with acceptable deviation; shaft geometry equality or within acceptable deviations; mounting compatibility; max torque and overhung/axial limits satisfied.
- **Preferences**: minimal overspec on torque/loads; closest ratio; dimensional proximity; same or higher efficiency class.
- **Explainability**: for each candidate, list satisfied constraints, relaxations, and provenance.

## Standalone App Workflow (Research Perspective)

As part of the research, we define how a catalog should flow through the Reader/Compiler in a **user-facing desktop app**:

1. **Input**
   - User drops catalog PDF(s) into the app or selects via file picker.
   - User chooses a Brand Pack (template or brand-specific).
   - Optional metadata: catalog name, effective date, output folder.

2. **Preflight**
   - App fingerprints the file (hash, page count).
   - Checks Brand Pack validity and storage availability.
   - Displays info back to the user before starting.

3. **Processing Pipeline**
   - Section indexing
   - Table extraction
   - Rule scraping
   - Brand mapping and transforms
   - Curations overlay
   - Normalization & validation
   - Asset slicing
   - Pack build
   - Diff reporting

4. **Review (if needed)**
   - Low-confidence or flagged rows shown with PDF snippet side-by-side.
   - User can correct values; fixes persist into the Brand Pack.

5. **Completion**
   - App reports coverage, errors (if any), and a diff vs. prior packs.
   - Buttons: Open Pack, Open in Configurator, Export report.

This workflow ensures the Reader/Compiler is **usable and interactive** while still adhering to the spec-first approach.

---

## Deliverable (this research.md)
- Establishes the ontology, fields, ingestion approach, rules to capture, and cross-brand logic—without tying to any specific brand.
- Document not only the ontology and ingestion rules, but also the **standalone workflow** expected in the Reader/Compiler app.
 - Document the **PDF ingestion strategy** (deterministic parsing, OCR fallback, optional local ML assist) as part of the research baseline.
