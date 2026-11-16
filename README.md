# ts-sim — Twilight Struggle simulator and advisor

Simulator, bots and advisor for full **Twilight Struggle** (2026 Hall of Fame Edition rules, base game, 110 cards), built with the `/boardgame-learning` method, reusing the Red Sea work (`red-sea-sim`).

- `data/` — game data: `map.json` (84 countries, adjacency, superpower links), `cards.json` (110 cards: Ops, era, side, asterisk, Scoring, underlined), `tracks.json` (Space Race, DEFCON, Military Ops, scoring values, setup), `intake.json` (scope decisions).
- `docs/` — `sources.md` (sources, extraction, differences from Red Sea), `rulings.md` (contradictions, rulings, defaults).
- `scripts/extract/` — scripts that built the data from the VASSAL module and cross-checked it (need the module and source files locally; not in the repo).
- `scripts/check_examples.py` — scoring examples from the 2026 rulebook and Background Book, checked against the data: `python3 scripts/check_examples.py data`.
- Engine: **Rust** (decided 16 Nov 2026), written as the first step of the first overnight run.

Card texts and publisher files (rulebooks, card images) are not stored here.
