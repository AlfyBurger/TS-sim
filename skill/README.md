# Game skill: twilight-struggle (first version, 18 Nov 2026)
- `SKILL.md` — the saved skill (built from `template.md` + `cards_text.py` + engine data + `tsts.py`).
- `tsts.py` — helper script embedded in §11; cross-check: `engine/target/release/tssim xcheck 300 1 > xc.jsonl && python3 skill/tsts.py xcheck xc.jsonl` (300 positions, 0 differences).
