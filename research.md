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

## Cross-Brand “Nearest-Equivalent” Logic
- **Hard constraints**: mounting style compatibility; ratio equivalent with acceptable deviation; shaft geometry equality or within acceptable deviations; mounting compatibility; max torque and overhung/axial limits satisfied.
- **Preferences**: minimal overspec on torque/loads; closest ratio; dimensional proximity; same or higher efficiency class.
- **Explainability**: for each candidate, list satisfied constraints, relaxations, and provenance.

## Deliverable (this research.md)
- Establishes the ontology, fields, ingestion approach, rules to capture, and cross-brand logic—without tying to any specific brand.
