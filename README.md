# ts-sim — Twilight Struggle simulator and advisor

Simulator, bots and advisor for full **Twilight Struggle** (2026 Hall of Fame Edition rules, base game, 110 cards), built with the `/boardgame-learning` method, reusing the Red Sea work (`red-sea-sim`).

- `data/` — game data: `map.json` (84 countries, adjacency, superpower links), `cards.json` (110 cards: Ops, era, side, asterisk, Scoring, underlined), `tracks.json` (Space Race, DEFCON, Military Ops, scoring values, setup), `intake.json` (scope decisions).
- `docs/` — `sources.md` (sources, extraction, differences from Red Sea), `rulings.md` (contradictions, rulings, defaults).
- `scripts/extract/` — scripts that built the data from the VASSAL module and cross-checked it (need the module and source files locally; not in the repo).
- `scripts/check_examples.py` — scoring examples from the 2026 rulebook and Background Book, checked against the data: `python3 scripts/check_examples.py data`.
- Engine: **Rust** (decided 16 Nov 2026), written as the first step of the first overnight run.

Card texts and publisher files (rulebooks, card images) are not stored here.

## Bot and test runs

- Engine and bots in `engine/` (Rust): `cargo build --release`, then `engine/target/release/tssim` (`random`, `match`, `log`, `diag`).
- Default bot (`core`) since run 2: one-move evaluation plus card sequencing (δ 0.5) with the opponent's reply scan, Scoring timing, China Card holding value 1.5, DEFCON fixes, DEFCON boundaries from Twilight Strategy (`docs/defcon_boundaries.md`, run 3). The optional 8th Action Round can be declined. Options are set as `core:key=value,...` (see `Weights::set` in `engine/src/bot.rs`).
- Test runs: `scripts/run_pairs.py` (paired head-to-head, both sides), `scripts/side_effects.py` (per-side effects), `scripts/diag_summary.py`; results in `results/run1`, `results/run2`; reports in `reports/` (`stage2_run1_report.pdf`, `stage2_run2_report.pdf`). Hypotheses and verdicts: `docs/hypotheses.md`.
