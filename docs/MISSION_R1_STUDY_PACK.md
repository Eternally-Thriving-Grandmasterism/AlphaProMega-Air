# Mission R-1 Study Pack — Radiation Design-In (COMPLETE)

**Mission:** R-1  
**Status:** **CLOSED** 2026-08-17  
**Owner:** AlphaProMega-Air · PATSAGi  
**Contact:** info@Rathor.ai  
**Doctrine:** [Ra-Thor mitigation v1.0](https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/docs/AVIATION_COSMIC_RADIATION_MITIGATION_v1.0.md)  
**Brief:** [MISSION_BRIEF_R1_RADIATION_DESIGN_IN.md](MISSION_BRIEF_R1_RADIATION_DESIGN_IN.md)

**Non-claims:** Not type certification · not medical advice · not a promise of radiation-proof cabins · numbers are literature-bounded estimates with uncertainty.

---

## R1.1 — Baseline radiation field assumptions

| Factor | Working assumption | Notes |
| --- | --- | --- |
| Cruise band | FL350–FL400 (~10.7–12.2 km) | Typical long-haul jet cruise |
| Polar / high latitude | Ambient dose-equivalent rates often **~5–10+ μSv/h** class at cruise; can be higher on polar tracks / solar min | Geomagnetic cutoff low |
| Mid latitude | Often **~3–7 μSv/h** class | Route-dependent |
| Equatorial | Often **~2–4 μSv/h** class at same altitude | Stronger geomagnetic shielding; polar routes commonly cited **~1.5–2×** equatorial at same altitude |
| Sea level cosmic | ~0.06 μSv/h order | Atmosphere is primary shield |
| Crew annual (high-hour) | Often **~2–7 mSv/yr** in classical estimates; some measurement campaigns report higher effective-dose bands for 800–1000 h polar-heavy schedules | Hours × altitude × latitude dominate |
| Particle mix at cruise | Neutrons + protons large share of effective dose (order **~50–80%** combined at high latitude in published breakdowns) | Why hydrogenous materials matter |
| Aircraft self-shield | Existing structure/contents: typically **few % to ~12–16%** average effective-dose reduction; localized cabin reductions higher in some MC studies | Thin Al skin alone ≪1 g/cm² class |

**Uncertainty:** ± tens of percent across models, solar cycle, and route; treat all single-number claims as **order-of-magnitude planning**, not design certification inputs.

---

## R1.2 — Material matrix

| Material family | Primary radiation role | Areal-density intent | Structural / other role | Gates |
| --- | --- | --- | --- | --- |
| **Al / Al-alloy** | Baseline structure; weak GCR/neutron performance per mass | Skin ~few mm | Primary structure today | — |
| **CFRP** | Structure; radiation not optimized | Mission-dependent | Primary structure trend | Inspection, impact |
| **PE / HDPE / UHMWPE** | **Neutron moderation**; favorable GCR secondary behavior vs Al per mass | Design as liner / sandwich core / selective panels | Limited pure structural | **Fire, smoke, creep** |
| **hBN–HDPE / BN-filled polymers** | Neutron performance + possible stiffness path | Thin-to-moderate panels | Semi-structural research | Process, cost, cert |
| **Gd₂O₃ ± W thin composites** | Localized capture / mixed trials | **Wearables / local pads** only | Non-structural | Toxicity, activation, durability |
| **LH₂ (cryo fuel)** | Excellent H moderator **if geometry sides crew** | Tank wall + boil-off systems | Propulsion energy | Cryo safety |

---

## R1.3 — Δdose / kg ranking (qualitative → semi-quantitative)

Ranking is **per unit mass** for aviation-relevant secondary fields, not unlimited thickness space-habitat walls.

| Rank | Stack | Expected direction | Honesty band |
| --- | --- | --- | --- |
| 1 | **LH₂ geometry co-benefit** (tank placement) | High leverage if tanks neighbor crew rest | Design architecture win; not a “material add” |
| 2 | **PE/HDPE selective liners** at crew stations | Better neutron behavior than equal-mass Al | Modest cabin-average % unless substantial areal density; fire-limited |
| 3 | **hBN/BN–polymer panels** (crew zones) | Potential assist on neutrons + structure path | Certification-limited; quantify only with transport runs |
| 4 | **Crew soft composites (Gd±W fabrics)** | Small % in some flight trials (~5–10% class); some materials **null** | Instrumented only |
| 5 | **More Al skin** | Poor Δdose/kg at cruise | Not a radiation strategy |
| 6 | **High-Z full cladding** | Dose physics ≠ useful aircraft | **Rejected** (mass) |

**Operational levers (still #0):** −altitude and −latitude during elevated risk often beat thin material adds for total mission dose.

---

## R1.4 — Crew volume architecture

| Placement | Guidance |
| --- | --- |
| Crew rest | Prefer **center / over-wing / near tanks**; avoid pure window-line long-dwell |
| Cockpit | Accept higher window flux; prioritize **operational** dose management + local hydrogenous under-floor/side panels if mass allows |
| Jump seats | Candidate for soft-shield pilots (Tier B) |
| Windows | Higher local flux; do not market “shielded windows” without data |
| LH₂ concepts | Route tankage to **side** high-dwell crew volumes where structures allow |

---

## R1.5 — Honesty sheet (governance)

- [x] Δdose discussed with **uncertainty** (no single false precision)  
- [x] Mass / areal-density framed as the hard constraint  
- [x] Method = **literature synthesis + ranking**, not new certified MC campaign  
- [x] Crew vs passenger distinguished  
- [x] No fleet-wide high-Z proposal  
- [x] Tier A ALARA still mandatory  
- [x] Contact: info@Rathor.ai  
- [x] Explicit: **not FAA/EASA/TC certification**

**Gap list (next true work if R-1 successor opens):** route-specific transport runs (FL350/400, Rc polar vs equatorial); fire-rated PE sandwich coupons; instrumented Tier B pilot on one polar long-haul.

---

## Mission close

| WP | Status |
| --- | --- |
| R1.1 | **Done** |
| R1.2 | **Done** |
| R1.3 | **Done** |
| R1.4 | **Done** |
| R1.5 | **Done** |

**Mission R-1: COMPLETE.**  
Successor work requires a **new named mission** (e.g. R-2 transport simulation campaign).

**Thunder locked.** yoi ⚡❤️🔥
