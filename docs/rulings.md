# Twilight Struggle — contradictions, rulings and open questions (Stage 1.4–1.5)

16 Nov 2026. Sources compared: 2026 Hall of Fame (HoF) rulebook and Background Book; FAQ v5 (2010, designer-authorised); VASSAL card faces (Deluxe wording, ~2010; used instead of scans at Alfy's request); Twilight Strategy card list (later digital-style wording); the 7 HoF card faces printed in the 2026 rulebook.

**Source precedence (Alfy, 16 Nov 2026):** Alfy's rulings > newer official text (2026 HoF rulebook, HoF card faces) > FAQ v5 where the 2026 rules are silent > rulebook examples > older card faces and card lists > fan sources.

## A. What was checked, and the result
- **Card faces:** all 110 VASSAL faces against the Twilight Strategy list. Ops, side, era and asterisk agree on every card. Most wording differences are phrasing only; those that change the effect are B1–B6 below, CIA Created/Lone Gunman (D1), and two trivial ones (Chernobyl "may" vs "must" designate a Region; Pershing II "up to three" vs "three" countries).
- **HoF card faces vs VASSAL faces (7 cards shown in the 2026 rulebook):** 5 identical in effect; 2 updated — **Red Scare/Purge** now also lowers cards the opponent *discards*, and **Quagmire** now states that it cancels NORAD. So the VASSAL faces are a good stand-in, but a HoF card can carry newer wording.
- **2026 rulebook examples:** all consistent with the rules text; Central America scoring example checked in code.
- **Background Book example of play (3 full turns, over 40 card plays):** replayed by hand, step by step — setup, every placement's adjacency and cost, every Coup and Realignment, both wars, DEFCON, Military Ops, the Space Race, card counts, held cards, the reshuffle at turn 3; three scorings also checked in code. **No errors found.**

## B. Contradictions settled by the sources
| # | Point | Sources disagree | Settled as | By |
|---|---|---|---|---|
| B1 | Formosan Resolution at Final Scoring | Twilight Strategy list: only when Asia Scoring is played | Taiwan also counts as a Battleground at Final Scoring if US-Controlled | Deluxe card face + FAQ |
| B2 | Shuttle Diplomacy at Final Scoring | list silent | Does not apply at Final Scoring; never affects SE Asia Scoring, Summit or Kitchen Debates; the US picks the Battleground removed, and that country is ignored entirely (presence too) | card face + FAQ |
| B3 | Junta | card face: "a Realignment roll" (singular) | Influence first; then one free Coup **or** Realignment rolls equal to the card's Ops, all in one of the two Regions | FAQ |
| B4 | South African Unrest | list: 2 Influence in a single adjacent country | The 2 may be split among countries adjacent to South Africa | card face + FAQ |
| B5 | Flower Power | list omits it | A US War card played on the Space Race gives the USSR nothing | card face + FAQ |
| B6 | Wargames | list omits it | Game ends with no Final Scoring; at DEFCON 3–5 the Event does not occur, so the card is not removed | card face + FAQ |
| B7 | Red Scare/Purge on discards | VASSAL face: "played" only | Also lowers cards the opponent discards (Quagmire/Bear Trap, Blockade, Latin American Debt Crisis, Space Race) | HoF card face (agrees with FAQ under Missile Envy) |
| B8 | Quagmire vs NORAD | VASSAL face omits it | Quagmire cancels NORAD | HoF card face |
| B9 | Reaching 20 VP in the Military Ops phase while holding a Scoring card | FAQ: the win stands (the held-card check is never reached) | Held cards are revealed first; holding a Scoring card loses | HoF 10.3.1 (ruling R1) |
| B10 | Ops modifiers on Event-granted Ops | FAQ: "Ops from Events are not affected" | They are affected: an Event's "conduct Operations as if…" counts as a card played for Ops (Containment + CIA Created = 2 Ops) | HoF 7.4.3 and its Example 3 (ruling R1) |

Also confirmed by the FAQ (no conflict): Truman Doctrine needs a country neither side Controls; superpower spaces count as adjacent Controlled countries for War Events like Brush War; the Cuban Missile Crisis loss applies whoever's card triggered the Coup; Nuclear Subs does not stop Events from moving DEFCON; Chernobyl blocks only Influence placed with Ops (Coups, Realignments, Events still allowed); a player may force himself to discard his own Scoring card (Five Year Plan); Grain Sales: the card taken may go to the Space Race or be paired with UN Intervention; UN Intervention only works with a card from your own hand, never in the Headline; Captured Nazi Scientist / One Small Step advances are not Space Race attempts; One Small Step gives the second box's ability if first there, VP from the second box only; Eagle/Bear Landed lets you discard one held card, never a Scoring card; a Coup uses all of a card's Ops (one Coup per card).

## C. Rulings settled with Alfy (16 Nov 2026)
- **R1. Precedence:** where the 2010 FAQ and the 2026 rulebook disagree, the newer 2026 text wins; the FAQ applies where the 2026 rules are silent (settles B9, B10).
- **R2. Event-granted Ops cannot go to the Space Race** (ABM Treaty, Olympic boycott, CIA Created, Lone Gunman, Glasnost, KAL-007): only a card discarded from hand can make a Space Race attempt (FAQ). Grain Sales is not an exception to this: the card it takes is a real card.
- **R3. Kitchen Debates and Our Man in Tehran with their condition false:** the Event does not occur; the card is discarded and stays in the game (same as Star Wars in the rulebook example; Wargames already settled the same way). Partial conditions (Glasnost's and KAL-007's Ops) never stop the rest of the Event, so those cards are removed.
- **R4. Quagmire / Bear Trap carry over** into the next turn until the die roll succeeds; they do not affect the Headline (FAQ).

## D. Defaults still open (applied unless Alfy objects)
- **D1. CIA Created / Lone Gunman:** the hand is revealed once, when the card is played (FAQ); Claude still remembers what it saw.
- **D2. Wargames at DEFCON 2:** the Event occurs; the player chooses to end the game (opponent +6 VP, no Final Scoring) or to do nothing; the card is removed either way.
- **D3. Both players caught holding a Scoring card:** the US wins (FAQ; 2026 rules silent).
- **D4. Fixed +2 US handicap:** placed as in the bidding rule 13.4 — only where the US has Influence after setup, and no more than 2 above what Control needs.
