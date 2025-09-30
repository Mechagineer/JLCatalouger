# DRN Gearmotor Configurator — research.md (Draft)

> Owner: Jonathan Lee
> Purpose: Map catalog sections, formulas, and data fields into structured references for downstream schema and rules.
> Version: 0.1 (skeleton)

---

## 1) Catalog Sections Overview

* **Selection basics** (procedure for valid drive system design). *(p.xx–yy)*
* **Gear unit families** (R, F, K, S, W, incl. RX, R..7). *(p.xx–yy)*
* **Ratios & stages** (2-stage, 3-stage, compound). *(p.xx–yy)*
* **Gear unit sizes & torque envelopes**. *(p.xx–yy)*
* **Motor ratings** (HP, poles, nominal speeds, inverter duty). *(p.xx–yy)*
* **Mounting positions** (M1–M6, MX, M0, variable, pivoted). *(p.xx–yy)*
* **Shaft options & dimensions** (solid/hollow, D/D1/D2, L1/L2). *(p.xx–yy)*
* **Dimension tables** (AC, L, LS, LB, flange diameters). *(p.xx–yy)*
* **Load limits** (overhung/axial, transmission element factors). *(p.xx–yy)*
* **Thermal & efficiency notes** (churning losses, run-in efficiency). *(p.xx–yy)*
* **Auxiliary information** (lubricants, painting/ECO2, NOCO-Paste, environmental). *(p.xx–yy)*

## 2) Core Procedures

* **Kinematics → Torque → Gear Unit → Motor → Brake → Inverter**
  Extract formulas and flow from catalog selection procedure. *(p.xx–yy)*
* **Controlled drives** (inverter limits, calculation of MMot, Mamax). *(p.xx–yy)*
* **Compound drives** (very low speed, torque limiting rules). *(p.xx–yy)*

## 3) Entities & Fields to Capture

* **Family**: name, subtype.
* **Size**: numeric ID, torque envelope, Mamax.
* **Ratio**: value, stage, admissibility, compound flag.
* **Motor**: hp, poles, synchronous speed, inverter notes.
* **Shaft option**: style, D/D1/D2, L1/L2, fit, optional flag.
* **Mounting**: code, allowance, breather/oil plug info.
* **Dimensions**: AC, L, LS, LB, flange diameters.
* **Limits**: overhung, axial, thermal, churning.
* **Auxiliary info**: lubricants, coatings, NOCO-Paste, temperature ranges.

## 4) Formulas & Constraints

* Kinematic equations (speed, distance, acceleration). *(p.xx–yy)*
* Torque derivations (static, dynamic, MMot). *(p.xx–yy)*
* Gear unit selection rules (Mamax vs MMot). *(p.xx–yy)*
* Brake torque ≤ 200% MMot for compound units. *(p.xx–yy)*
* Overhung/axial load derivations and factors. *(p.xx–yy)*
* Mounting-dependent lubrication/breather rules. *(p.xx–yy)*
* Thermal/churning checks. *(p.xx–yy)*

## 5) Page/Line Anchors

Each field and formula above will be referenced with:

* **Catalog page #**
* **Line or table number**
* **Source context** (selection table, dimension sheet, rule paragraph)

*(Placeholder until we annotate directly from DRN catalog PDF)*

## 6) Glossary (Catalog Terms)

* **Mamax**: maximum permissible output torque of gear unit.
* **MMot**: motor torque.
* **L1/L2**: shaft lengths (catalog-specific).
* **AC, LS, LB**: dimension codes from catalog tables.
* **M1–M6, MX, M0**: mounting positions.
* **Optional shaft**: shaft option not standard, with possible derating.
* **Transmission element factors**: multipliers for belts, chains, gears.

## 7) Open Questions

* Are all dimension codes (AC, LS, LB, etc.) needed as top-level fields or only shaft-related ones?
* For inverter duty: do we encode curves or tabular derating factors?
* How granular do we need auxiliary info (e.g., lubricants by size vs by family)?

---

## Deliverable

A complete `research.md` will:

* Contain all catalog-derived data fields with exact page/line references.
* Include extracted formulas and constraints with context.
* Provide glossary + clarifications for ambiguous terms.
* Serve as the reference input for `plan.md` (schema & rules design).
