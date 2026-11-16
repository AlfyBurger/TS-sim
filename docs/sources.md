# Twilight Struggle — sources and data (Stage 1)

Full write-up, including the rule-by-rule comparison with Red Sea: Gaming project doc `claude/twilight-struggle-sources.md`.

## Scope (Alfy, 16 Nov 2026)
2026 Hall of Fame Edition rules (Alfy owns it); base game, 110 cards (104–110 in); variants, promos and scenarios out. Handicap chosen at each game's start; **all testing assumes fixed +2 US Influence**. Claude tracks the game; opponent with levels 1–5.

## Official sources
- **HoF Rulebook** (2026, 24 pp.) — baseline rules. GMT asset server, `Twilight+Struggle+20th+Anniversary+HOF+Edition/Twilight+Struggle+20th+Ann_Rule+Book_WEB.pdf`.
- **HoF Background Book** (2026, 48 pp.) — 3-turn example of play (checked: no errors), "DEFCON Traps" strategy article, designer notes, card histories.
- **FAQ v5** (2010) — 90 Q&As; applies where the 2026 rules are silent (ruling R1).
- Not used: 2015 Deluxe rules (superseded), 2010 errata cards (later printings corrected).

## Data (in `data/`)
- **Map** from the VASSAL module (code and setup data): 84 countries, 29 Battlegrounds; superpower links USA–Canada/Cuba/Japan/Mexico, USSR–Afghanistan/Finland/N. Korea/Poland/Romania. Cross-checked against an independent open-source engine (vrazix/twilight-struggle): 0 differences.
- **Cards**: Ops, era, side, asterisk from the VASSAL module, cross-checked with the Twilight Strategy card list (identical on all 110) and vrazix (identical side on 98). Underlined cards inferred from the module's 20 reminder markers.
- **Tracks**: Space Race read from the HoF graphic; DEFCON, Military Ops (cap 5), VP ±20, turns/hands/Action Rounds, scoring values, setup.

## Strategy sources (Stage 2)
- Official: HoF Background Book (DEFCON traps, example of play, designer notes); Jason Matthews' short strategy video.
- Twilight Strategy (twilightstrategy.com; free Leanpub book, CC BY-NC-SA, 2014): general strategy, regions, one article per card, annotated games. Beginner-level, partly dated.
- Current top play: "The collected musings of Sankt" (not reachable from the workspace); Legendary Tactics card videos; Action Round Zero; r/twilightstruggle.
- Competitive convention: optional cards in; USSR reckoned to win ~60% without handicap, hence +2 US Influence or bidding.
