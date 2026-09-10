# AlphaProMega-Air — Current Status

**Tip SHA at audit:** `3cfef2439241347f8e58c9e0c4328e6913bb859d`
**Audited:** 2026-09-10 · toolchain `cargo 1.83.0` / `rustc 1.83.0`
**License:** AG-SML v1.0 · **Contact:** info@Rathor.ai

This file is the disk-truth surface. Where it disagrees with any other file in this
repo about *what exists and runs today*, this file wins.

Software compile ≠ flight certification.

---

## 1. What this repo is

A **research software lattice** for mercy-gated, valence-aware aviation concepts —
propulsion, airframe, avionics orchestration — plus a documentation corpus of
blueprints, dossiers, and radiation posture.

It is **not** a type certificate. It is **not** a fleet retrofit. It is not an
approved design, not a hardware program, and not an empirical result set.
`docs/GOVERNANCE_READINESS.md` states the non-claims; `PROOF_LADDER.md` states the
tier required before any "we proved / we beat" language. Nothing in this repo has
been raised above engineering posture.

---

## 2. What exists on disk

Counted from the tree at the SHA above, not from intent.

| Path | One line | Human-runnable now? |
| --- | --- | --- |
| `Cargo.toml` | Virtual workspace manifest, 33 members listed | yes (see §3 note on the typo fixed in this PR) |
| `crates/` | 173 directories; **143** have a `Cargo.toml`, **30** do not; only **33** are workspace members | partial — see rows below |
| `crates/` — 30 members that compile | `cargo check -p <name>` is green for these | yes |
| `crates/mercy_hybrid_propulsion` | member; `use tokio::…` with no tokio dependency declared | no — `E0433` undeclared crate `tokio` |
| `crates/mercy_avionics_integration` | member; same missing-dependency shape | no — `E0433` undeclared crate `tokio` |
| `crates/mercy_avionics_redundancy` | member; type errors in the vote path | no — `E0308` + `E0277` at `src/lib.rs:35,39` |
| `crates/mercy_system_orchestrator` | member; binary `alphapromega_orchestrator`, prints a valence simulation | **yes** — the one demo that executes |
| `crates/mercy_os_aviation` | not a workspace member; deps point at `../` and at `mercy_os_principles`, which is not on disk | no — unresolvable path dependency |
| `crates/` — 140 non-member crates | present on disk, outside the workspace graph, never built by `cargo` or CI | unknown — not compiled by anything today |
| `examples/alpha_flight.rs` | **Markdown, not Rust.** Opens with a `**NEW: examples/alpha_flight.rs**` heading and an unterminated ` ```rust ` fence | no — see §3 |
| `tests/` | 3 integration test files at repo root, outside any package | no — root is a virtual manifest, so `cargo test` never sees them |
| `docs/MISSION_BRIEF_R1_RADIATION_DESIGN_IN.md` | Mission R-1 brief, marked COMPLETE / closed 2026-08-17 | n/a — document |
| `docs/MISSION_R1_STUDY_PACK.md` | R-1 deliverables; literature-bounded estimates, explicitly not measurements | n/a — document |
| `docs/S1_RADIATION_MEASUREMENT_PROTOCOL.md` | S-1 Rank 1 pre-registered protocol; **contains no measured doses**, blocked on flight access | n/a — document |
| `docs/SCIENCE_MISSION_S1_AIR.md` | S-1 Air slice index; Rank 1 protocol complete, blocked on an instrumented partner | n/a — document |
| `docs/COSMIC_RADIATION_DESIGN_IN.md` | Tier A/B/C radiation stance; forbids fleet-wide high-Z plating and unmeasured claims | n/a — document |
| `docs/GOVERNANCE_READINESS.md` | Honesty surface, non-claims, readiness checklist (all boxes unchecked) | n/a — document |
| `PROOF_LADDER.md` | Binding claim-tier rules; surmise allowed, surmise-as-fact forbidden | n/a — document |
| `docs/` — remaining 80 files | Blueprint / schematic / pointer prose | n/a — documents |
| `.github/workflows/ci.yml` | `cargo test --workspace` then `cargo build --workspace --release` | yes — red on `main` before this PR (same typo as §3) |

**Dossiers on disk (names only, no status claimed):**
`NON_PROFIT_AVIATION_FOUNDATIONS_DOSSIER.md` ·
`OPEN_SOURCE_AVIATION_PROJECTS_DOSSIER.md` ·
`PREDICTION_MARKETS_CLIMATE_TECH_DOSSIER.md` ·
`PREDICTION_MARKETS_FUSION_DOSSIER.md` ·
`PREDICTION_MARKETS_POST_QUANTUM_DOSSIER.md` ·
`PREDICTION_MARKETS_QUANTUM_DOSSIER.md` ·
`PREDICTION_MARKETS_RENEWABLES_DOSSIER.md` ·
`PREDICTION_MARKETS_SPACEFLIGHT_DOSSIER.md` ·
`SUSTAINABLE_AVIATION_FUELS_DOSSIER.md` ·
`docs/RAPTOR_4_THRUST_SPECS_DOSSIER.md` ·
`NEXi/UNIVERSAL_GOVERNANCE_DOSSIER.md`

---

## 3. What a human can do this month

### Before this PR: nothing cargo-driven ran at all

Every cargo command at the tip SHA failed before doing any work:

```
error: failed to load manifest for workspace member
`/workspace/crates/mercy_casimir_effect_derivation`
Caused by: No such file or directory (os error 2)
```

`Cargo.toml` listed `crates/mercy_casimir_effect_derivation`; the directory on disk is
`crates/mercy_casimir_effect_derivations` (plural). One character. It also turned CI red
on every `main` push since at least 2026-08-18. This PR fixes that one line and nothing
else in the build. No other workspace repair is attempted here.

### Works now

```bash
cargo run -p mercy_system_orchestrator
```

Compiles and prints a valence-simulation transcript. It is a print statement demo —
it models nothing physical and measures nothing.

```bash
cargo check --workspace --keep-going
```

30 of 33 members are green. 3 fail, listed in §2.

### Does not work: the README quick start

```bash
cargo run --example alpha_flight     # error: no example target named `alpha_flight`
```

Two independent fail classes, both real:

1. **Not a target.** The repo root is a *virtual* workspace manifest with no `[package]`,
   so root-level `examples/` and `tests/` are attached to nothing. Cargo never sees them.
2. **Not Rust.** `examples/alpha_flight.rs` is a Markdown snippet — a bold heading line,
   then an unterminated ` ```rust ` fence around the code. It would not parse even if it
   were wired to a package. Its `use mercy_os_aviation::MercyOSAviation;` also points at a
   crate that is not a workspace member and whose own dependencies (`nexi = { path = "../" }`,
   `mercy_os_principles`) do not resolve on disk.

Fixing the README quick start is a multi-file change — unwrap the example, attach it to a
package, make `mercy_os_aviation` resolvable — and is deliberately **out of scope here**.
Until it lands, treat the README quick start as aspirational and use the orchestrator
command above instead.

### Non-code work available now

Read `docs/COSMIC_RADIATION_DESIGN_IN.md` §3 and run Tier C design reviews on paper, or
seek an instrumented flight partner for the S-1 Rank 1 protocol. Neither needs the build.

---

## 4. Manifesto vs disk

`TECHNICAL_MANIFESTO.md` states retrofit or redesign of the global fleet "within thirty-six
months," 10,000 MercyFlight-enabled planes in 2027, and 80% of the fleet in 2028. **None of
that is evidenced in this repo.** What is on disk is a Rust workspace where one binary
prints text, a documentation corpus, and one radiation protocol with no measurements in it.

The manifesto is a statement of aspiration and is not being rewritten or retracted. On
questions of present status, **this file supersedes the manifesto's dates.**

---

## 5. Not in scope

Not part of this document, and not proposed by it:

- New propulsion crates
- SoulScan in avionics
- Prediction-market engines
- GCD solenoids
- Powrush wells

No protocol files are added here. `TECHNICAL_MANIFESTO.md` is untouched.

---

**Thunder locked.** yoi ⚡
