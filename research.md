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
  - `motor.poles`, `motor.hp`, `motor.kw`, `motor.n_sync`

- **Output End**
  - `shaft.style` (solid/hollow/spline/block)
  - `shaft.D`, `shaft.D1/D2`, `shaft.L1/L2`, `shaft.fit`, `shaft.optional_flag`

- **Mounting**
  - `mounting.code` (e.g., M1..M6/MX/M0 or brand codes mapped to these)
  - `mounting.pivoted_dynamic`, `mounting.pivoted_stationary`
  - `mounting.breather_oil_notes` (free text)

- **Loads & Limits**
  - `overhung.constants` (e.g., a,b,c,f,d,l), `overhung/axial.limits`
  - `thermal.flags`, `efficiency.eta_tot` (if provided)

- **Metadata**
  - `brand`, `catalog_version`, `provenance` (page/line/table cell), `admissible`, `deprecation`

## Core Procedures & Rules to Capture (Brand-Agnostic)
- **Selection flow**: kinematics → torques → gear unit selection (by torque & ratio constraints) → motor selection (max/effective point) → brake selection → inverter selection.
- **Compound/very-low-speed** patterns: limit motor based on permitted output torque; compute allowable motor torque from total ratio and efficiency; brake torque caps relative to motor torque (if specified by a brand).
- **Mounting behavior**: allowed mounting codes; required adjustments (oil fill, breather position); special “pivoted/universal” modes if present.
- **Loads**: overhung and axial load determination; transmission element factors where available; constants tables and formulas.
- **Thermal & efficiency**: thermal checks and churning/immersion guidance where applicable.

## Data Field Inventory (maps to ontology)
- `family`, `stage`, `ratio.i`, `size.code`, `torque.nominal`, `torque.max`
- `motor.poles`, `motor.hp`, `motor.kw`, `motor.n_sync`
- `shaft.style`, `shaft.D`, `shaft.D1/D2`, `shaft.L1/L2`, `shaft.fit`, `shaft.optional_flag`
- `mounting.code`, `mounting.pivoted_dynamic`, `mounting.pivoted_stationary`, `mounting.breather_oil_notes`
- `overhung.constants`, `overhung/axial.limits`, `thermal.flags`, `efficiency.eta_tot`
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
- **Hard constraints**: family compatibility; ratio within available steps; shaft geometry equality/tolerance; mounting compatibility; max torque and overhung/axial limits satisfied.
- **Preferences**: minimal overspec on torque/loads; closest ratio; dimensional proximity; same or higher efficiency class.
- **Explainability**: for each candidate, list satisfied constraints, relaxations, and provenance.

## Open Questions
- Which dimension fields beyond `shaft.D/L1/L2` should be first-class facets?
- Acceptable tolerance bands for nearest-match (ratio, D, L1/L2) per family/size.
- How to represent thermal conditions (boolean flags vs. condition descriptors)?

## Deliverable (this research.md)
- Establishes the ontology, fields, ingestion approach, rules to capture, and cross-brand logic—without tying to any specific brand.
