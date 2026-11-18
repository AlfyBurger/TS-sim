---
name: "twilight-struggle"
description: "Rules, map, all 110 cards, errata, house rulings, the tested move-by-move play procedure, DEFCON limits and a helper script for Twilight Struggle (GMT, 2026 Hall of Fame edition, base game), for playing it as Alfy's opponent or answering rules questions. Use whenever Twilight Struggle (the full game, not Red Sea) is mentioned."
---

# Twilight Struggle (2026 Hall of Fame edition)

Two-player card-driven game of the Cold War, 1945–1989: the US against the USSR on a world map, 10 turns, 110 cards. **Scope (Alfy, 16 Nov 2026):** 2026 Hall of Fame (HoF) rules; base game with all 110 cards (the former optional cards 104–110 are now standard). Out of scope: the Chinese Civil War variant, Turn Zero, the Late War and other scenarios, the alternate Space Race track, promotional cards. Not to be confused with the separate game *Twilight Struggle: Red Sea* (own skill).

**Version:** first version, 18 Nov 2026 (Stage 1 rules + Stage 2 play procedure, tested in 5 simulator runs). Stage 3 (game plans, difficulty levels, advisor) and Stage 4 (session specification) are still to come.

**Sources:**
- 2026 HoF Rulebook (24 pp.) and Background Book (48 pp., including a 3-turn example of play, checked with no errors found). Rule numbers in brackets point to the HoF Rulebook.
- FAQ v5 (2010, designer-authorised), applied where the 2026 rules are silent.
- Card faces: VASSAL module (Deluxe wording) and the 7 HoF faces printed in the rulebook; cross-checked against the Twilight Strategy card list (identical Ops, side, era and asterisk on all 110).
- Map from the VASSAL module, cross-checked against an independent open-source engine (0 differences).
- Strategy: Twilight Strategy (twilightstrategy.com; card articles and the DEFCON article, CC BY-NC-SA), the HoF Background Book, and Alfy's answers to the "what wins" quiz. Everything is summarised in my own words.

**Precedence (Alfy, 16 Nov 2026):** Alfy's rulings > newer official text (2026 HoF rulebook and HoF card faces) > FAQ v5 where the 2026 rules are silent > rulebook examples > older card faces and card lists > fan sources.

**Simulator:** Alfy's private repository `AlfyBurger/TS-sim` (compiled Rust engine, test reports in `reports/`). The helper script in §11 reproduces the simulator's evaluation and Ops choices exactly (cross-checked on 300 random positions, 0 differences).

Contents: 1 Rules core · 2 Map · 3 Tracks and scoring · 4 Cards · 5 Rulings and open questions · 6 Playing: move-by-move decisions · 7 DEFCON limits · 8 Card notes · 9 Strategy (Stage 3, to come) · 10 Game sessions · 11 Helper script

---

## 1. Rules core

### Setup [3]
- Shuffle the 35 Early War cards (The China Card excluded); deal 8 to each player, USSR first. The China Card starts face up with the USSR. Players see their hands before placing Influence.
- **USSR places first** (15 Influence): Syria 1, Iraq 1, N. Korea 3, E. Germany 3, Finland 1, plus 6 anywhere in Eastern Europe.
- **US places second** (25): Canada 2, Iran 1, Israel 1, Japan 1, Australia 4, Philippines 1, S. Korea 1, Panama 1, South Africa 1, U.K. 5, plus 7 anywhere in Western Europe.
- DEFCON 5, VP 0, Military Ops 0, both Space Race markers at Start, turn 1.
- **Handicap (chosen at the start of each game):** none, bidding (13.4), or the fixed **+2 US Influence** used in all testing. Handicap Influence is placed after all other setup, only in countries where that side already has Influence, and never more than 2 above what Control needs there.

### Turn sequence [4.5]
10 turns. Turns 1–3: hands of 8, 6 Action Rounds. Turns 4–10: hands of 9, 7 Action Rounds.
A. **Improve DEFCON** by 1 (max 5).
B. **Deal** up to hand size (The China Card never counts). Mid War cards are shuffled into the draw deck at the end of turn 3, Late War cards at the end of turn 7 [4.4]; the discard pile is only reshuffled when the draw deck runs out, and all remaining deck cards are dealt first [4.3]. Starred cards played as Events never return.
C. **Headline.** Each player secretly picks a card; both are revealed together. The higher Ops Event happens first, the US first on a tie. Scoring cards count as 0 Ops; if both Headline Scoring cards, the US's goes first. Every Headline Event happens, whoever it favours; an opponent's Event is implemented by the opponent, but the card's player is the Phasing Player for DEFCON. No Ops are gained in the Headline unless the Event says so. The China Card cannot be Headlined.
D. **Action Rounds**, USSR first, one card each per round, fully resolved before the next. A player out of cards sits out the rest of the turn. Usually one card is left over and held into the next turn.
E. **Military Ops check:** each player gives the opponent 1 VP per point short of the requirement (= DEFCON); apply the net. Reset both tracks to 0.
F. **Held Scoring card:** a player holding a Scoring card now loses at once.
G. **Flip The China Card** face up if it was passed face down this turn.
H. Advance the turn marker. After turn 10: **Final Scoring**.

### Playing a card [5]
- Each card is played for its **Event** or for its **Operations**; you can never pass or simply discard (5.1).
- Own or neutral cards: either use. **An opponent's card can only be played for Ops, and its Event still happens** (implemented by the opponent). The Phasing Player chooses whether the Event comes before or after the Ops (5.2).
  - If the Event cannot happen (prerequisite not met, condition false, or prohibited by another Event), it does not, and a starred card is discarded instead of removed.
  - If it happens with no effect, it still counts as played (a starred card is removed).
- `*` after the title: removed from the game when played as an Event (2.2.4). Underlined title: lasting effect, kept beside the board (2.2.5, 7.3).
- A requirement for "a card of value X" is met by any higher value (5.3). A card discarded by an Event never triggers its Event, Scoring cards included (5.4). Card text beats rules, except that no Event can force a player to hold a Scoring card (5.5, 10.1.5).
- **Ops modifiers** from Events (Containment, Brezhnev Doctrine, Red Scare/Purge, Vietnam Revolts) stack, apply to The China Card, and count for every purpose, including Coup rolls, Military Ops and the value needed for discards (7.4, 7.4.2). An Event saying "conduct Operations as if…" is treated as a card played for Ops, so modifiers apply (7.4.3; ruling B10). A modifier belongs to one player and does not move with a card taken from the opponent (7.4.1). A cancelled or blocked Event card can still be played for Ops (7.5).
- **War cards** (Korean War, Arab-Israeli War, Indo-Pakistani War, Brush War, Iran-Iraq War) need no Influence in the countries involved; the attacker always gets the Military Ops, and the VP on success (7.6). Triggered by an opponent's Ops play, the opponent gets the Military Ops (8.2.4).

### Operations [6]
All of a card's Ops go to **one** of four uses.

**Place Influence [6.1]**
- One marker at a time, in or adjacent to a country where you had Influence **at the start of the Action Round**, or adjacent to your own superpower. Event placements ignore this unless the card says otherwise.
- Cost: 1 Op per marker in an uncontrolled or friendly-Controlled country; 2 Ops if the opponent Controls it. If Control breaks during the placement, later markers there cost 1.
- May be split across countries and Regions. Influence is unlimited.

**Control [2.1.7]:** your Influence ≥ Stability AND your Influence − opponent's ≥ Stability.

**Realignment [6.2]**
- 1 Op per roll; the target must contain enemy Influence; no Influence of your own is needed nearby. Finish each roll before choosing the next target; the same country may be targeted again.
- Both roll a die. Each adds +1 per adjacent country it Controls, +1 if it has more Influence in the target, +1 if its superpower is adjacent. The higher total removes the difference from the loser's Influence. Ties do nothing; Realignment never adds Influence.

**Coup [6.3]**
- Target must contain enemy Influence. Roll + card Ops; success if the total exceeds 2 × Stability. Remove that excess in enemy Influence, then add your own for any remainder.
- Military Ops track advances by the card's Ops, success or not (cap 5).
- **A Coup in a Battleground lowers DEFCON by 1** (8.1.4), free Coups from Events included. Free Coups from Events do not count as Military Ops, and those in a named Region ignore DEFCON's geographic limits (6.3.5, 8.2.2).

**Space Race [6.4]**
- Discard a card with Ops ≥ the next box's requirement and roll; advance if the roll is in range. The Event never happens (6.4.5): the Space Race is the main way to get rid of an opponent's Event.
- One attempt per turn (two with the box 2 ability). Event advances (Captured Nazi Scientist, One Small Step) are not attempts.
- First arrival earns the larger VP; abilities belong to the first arrival only and end when the second arrives. No more attempts after box 8. Track in §3.

### DEFCON and Military Ops [8]
- Track 5 (peace) to 1. "Improve" = up, "degrade" = down. Starts at 5; max 5.
- Restrictions on Coups and Realignments by Ops: **DEFCON 4: none in Europe; 3: none in Europe or Asia; 2: none in Europe, Asia or the Middle East.**
- **DEFCON 1: nuclear war; the Phasing Player loses** (8.1.3), whoever's card caused it. In the Headline the card's player is the Phasing Player.
- **Required Military Ops per turn = DEFCON** (checked in phase E with the DEFCON at that time). Coups and War cards count; Realignments and free Coups from Events do not.

### The China Card [9]
- 4 Ops, +1 if all its Ops are spent in Asia (Southeast Asia included). Modifiers apply.
- Playing it uses an Action Round, so the player may end the turn holding more cards. After play it passes face down to the opponent, who cannot play it until it flips face up at the end of the turn.
- It cannot be Headlined, used as a discard for an Event, or forced out by Events or a card shortage. Its holder at Final Scoring gains 1 VP.

### Victory [10.3]
- **Automatic win:**
  - reaching **20 VP** (both sides' awards from one Event or Scoring card are applied first). If reached during the Military Ops check, held cards are revealed first: a player holding a Scoring card loses instead (ruling B9);
  - **Europe Scoring** played while one player Controls Europe: that player wins;
  - holding a Scoring card in phase F loses;
  - **DEFCON 1** while the opponent is the Phasing Player;
  - Wargames (§4), Cuban Missile Crisis (§4).
- **Final Scoring** (end of turn 10): score every Region (Europe first: Control of Europe wins outright), Southeast Asia counts only as part of Asia, then +1 VP to The China Card's holder. Higher VP wins; 0 is a draw.

---

## 2. Map

84 countries, 29 Battlegrounds (marked **B**). Sub-Regions: Eastern Europe (EE), Western Europe (WE); Austria and Finland are in both. Southeast Asia (SEA) is part of Asia. Egypt and Libya are in the Middle East; Canada and Turkey in Europe. Superpower links: the **USA** is adjacent to Canada, Cuba, Japan and Mexico; the **USSR** to Afghanistan, Finland, N. Korea, Poland and Romania (they count as adjacent Controlled countries for Realignments and War Events; no Influence is placed there).

{MAP}

---

## 3. Tracks and scoring

**Regional Scoring** [10.1]

| Region | Presence | Domination | Control | Superpower bonus |
|---|---|---|---|---|
| Europe | 3 | 7 | automatic win | yes |
| Asia | 3 | 7 | 9 | yes |
| Middle East | 3 | 5 | 7 | no |
| Africa | 1 | 4 | 6 | no |
| Central America | 1 | 3 | 5 | yes |
| South America | 2 | 5 | 6 | no |
| Southeast Asia (#38, once) | 1 VP per Controlled country, Thailand 2 | | | |

- **Presence:** Control at least 1 country in the Region.
- **Domination:** Control more countries AND more Battlegrounds than the opponent, and at least 1 non-Battleground.
- **Control:** Control more countries than the opponent AND every Battleground in the Region.
- **Bonuses:** +1 per Controlled Battleground in the Region; +1 per Controlled country there adjacent to the enemy superpower (Europe, Asia, Central America only, per the cards).
- Move the VP marker by the difference between the two totals.

**VP track** [10.2]: −20 (USSR wins) to +20 (US wins). In this skill and the helper, **VP is signed: + = US**.

**Space Race** [6.4]

| Box | Name | Ops needed | Roll | VP (first/second) | Ability (first arrival only, until the other arrives) |
|---|---|---|---|---|---|
| 1 | Earth Satellite | 2 | 1–3 | 2/1 | – |
| 2 | Animal in Space | 2 | 1–4 | – | may make 2 Space Race attempts per turn |
| 3 | Man in Space | 2 | 1–3 | 2/0 | – |
| 4 | Man in Earth Orbit | 2 | 1–4 | – | the opponent must choose and reveal its Headline first |
| 5 | Lunar Orbit | 3 | 1–3 | 3/1 | – |
| 6 | Eagle/Bear has Landed | 3 | 1–4 | – | may discard 1 held card at the end of the turn (never a Scoring card) |
| 7 | Space Shuttle | 3 | 1–3 | 4/2 | – |
| 8 | Space Station | 4 | 1–2 | 2/0 | may take 8 Action Rounds per turn (optional) |

**DEFCON**: 5 no effect · 4 no Coup/Realignment in Europe · 3 also Asia · 2 also Middle East · 1 Phasing Player loses. **Military Ops**: 0–5, requirement = DEFCON.

---

## 4. Cards

Format: **#id Title** · Ops · `*` removed after Event · `[U]` lasting effect. Effects are short summaries in my own words; full rulings in §5. Deck: Early War 35 + The China Card, Mid War 46, Late War 23 (with #103–110 counted in their eras). Card #6 is The China Card.

{CARDS}

---

## 5. Rulings, errata and open questions

**Settled by the sources (contradictions between them):**
- **B1 Formosan Resolution:** Taiwan also counts as a Battleground at Final Scoring if US-Controlled (card face + FAQ; the Twilight Strategy list says Asia Scoring only).
- **B2 Shuttle Diplomacy:** not at Final Scoring; never affects Southeast Asia Scoring, Summit or Kitchen Debates; the US picks the Battleground, and that country is ignored entirely (Presence too).
- **B3 Junta:** Influence first; then one free Coup, **or** Realignment rolls equal to the card's Ops, all in Central or South America (FAQ).
- **B4 South African Unrest:** the 2 adjacent Influence may be split (card face + FAQ).
- **B5 Flower Power:** a US War card played on the Space Race gives the USSR nothing.
- **B6 Wargames:** ends the game with no Final Scoring; at DEFCON 3–5 the Event does not occur and the card is not removed.
- **B7 Red Scare/Purge:** also lowers cards the opponent discards (Quagmire/Bear Trap, Blockade, Latin American Debt Crisis, Space Race) (HoF face).
- **B8 Quagmire cancels NORAD** (HoF face).
- **B9 20 VP in the Military Ops phase while holding a Scoring card:** held cards are revealed first; holding one loses (HoF 10.3.1; the FAQ said the win stands).
- **B10 Ops modifiers apply to Event-granted Ops** (HoF 7.4.3 and its example; the FAQ said they did not).
- Also confirmed by the FAQ:
  - Truman Doctrine needs a country neither side Controls.
  - Superpower spaces count as adjacent Controlled countries for War Events such as Brush War.
  - The Cuban Missile Crisis loss applies whoever's card triggered the Coup.
  - Nuclear Subs does not stop Events from moving DEFCON.
  - Chernobyl blocks only Influence placed with Ops.
  - A player may force himself to discard his own Scoring card (Five Year Plan).
  - **Grain Sales:**
    - the card taken may go to the Space Race or be paired with UN Intervention;
    - UN Intervention works only with a card from your own hand, never in the Headline.
  - Captured Nazi Scientist and One Small Step advances are not Space Race attempts.
  - One Small Step: gives the second box's ability if first there, and VP from the second box only.
  - Eagle/Bear has Landed: discard one held card, never a Scoring card.
  - A Coup uses all of a card's Ops (one Coup per card).

**Alfy's rulings (16 Nov 2026):**
- **R1 Precedence:** where the 2010 FAQ and the 2026 rulebook disagree, the 2026 text wins; the FAQ applies where the 2026 rules are silent.
- **R2 Event-granted Ops cannot go to the Space Race:** ABM Treaty, Olympic boycott, CIA Created, Lone Gunman, Glasnost, KAL-007. Only a card discarded from hand can make an attempt. Grain Sales is not an exception: the card it takes is a real card.
- **R3 Kitchen Debates, Our Man in Tehran with their condition false:** the Event does not occur; the card is discarded and stays in the game. Partial conditions (Glasnost's and KAL-007's Ops) never stop the rest of the Event, so those cards are removed.
- **R4 Quagmire / Bear Trap carry over** into the next turn until the roll succeeds; they do not affect the Headline.

**Open (my default applies unless Alfy objects):**
- **D1 CIA Created / Lone Gunman:** the hand is shown once, when the card is played; I still remember what I saw.
- **D2 Wargames at DEFCON 2:** the Event occurs; the player either ends the game (opponent +6 VP, no Final Scoring) or does nothing; the card is removed either way.
- **D3 Both players holding a Scoring card at phase F:** the US wins (FAQ; 2026 rules silent).
- **D4 Fixed +2 US handicap:** placed as in bidding rule 13.4 (only where the US has Influence after setup; at most 2 above what Control needs).

---

## 6. Playing: move-by-move decisions

This is the procedure of the simulator's best bot after Stage 2 run 5 (18 Nov 2026), written so I can apply it at the table. Each rule below was tested in simulation (1,000 deals per side unless stated, margins 95%); the evidence is given where it matters. How the bot grew, run by run: run 1 removed a harmful Final Scoring projection (+57% vs the first bot); run 2 added card sequencing and Scoring timing (60% vs run 1); run 3 the DEFCON limits (DEFCON losses 19 → 1 in 400 games); run 4 near-Control credit and a two-move look-ahead (95% vs run 3); run 5 tuned the weights (52.5% vs run 4).

### 6.1 Tools
- At the start of a game, write the helper script from §11 to `tsts.py` and create the state: `python3 tsts.py init game.json <MY SIDE> 2` (the last number is the US handicap). Add the free setup Influence with `add`.
- **After every play**, update `game.json`: Influence, DEFCON, VP (signed, + = US), Military Ops, Space Race (`space`, `space_first`, `space_attempts`), China Card (`china`, `china_up`), `effects` (lasting and this-turn effects, names as in §11), `hand`, `removed` (starred Events played), `discard` (cards known to be in the discard pile), `opp_hand_size`, `opp_known` (opponent cards I have seen and that are still in their hand), `known_to_opp`, `turn`, `ar`, `rounds_done`.
- **Before every decision:** `python3 tsts.py report game.json`. It gives every Region's score if scored now, Control, the Southeast Asia score, the Final Scoring projection, where each Scoring card is (with the chance the opponent holds an unseen one), DEFCON zones, Military Ops shortfalls, and flags on my hand (opponent Events, DEFCON limits, suicide cards, Scoring card deadlines).
- **Coups and Realignments:** always `coup` / `realign` for the exact odds and outcomes. Never estimate them in my head.
- **Ops:** `decide` gives the best Ops use of every card in my hand, ranked, with the bot's exact evaluation and choice procedure (greedy placement, Coups with exact odds, Realignments, Space Race). It does not value Events, the opponent's reply, sequencing or timing: those are Steps 4–6.
- **Events:** describe the outcome and value it with `whatif`, e.g. `python3 tsts.py whatif game.json USSR '{"inf": {"W. Germany": [-4, 0]}, "play": [10]}'`. For an opponent's card played for Ops, its Event's value to them comes off the Ops value.
- **Dice:** `python3 tsts.py roll N`. Say what the roll is for before rolling.
- The simulator itself (repo `AlfyBurger/TS-sim`, `tssim`) is not yet wired as a move advisor; Stage 3 adds that. Until then the helper is the tool, and I apply Steps 4–6 by hand.

### 6.2 Headline
1. The China Card cannot be Headlined. As US, remember the USSR may Headline Defectors' mirror threat: **Defectors** (US card) cancels a USSR Headline; as USSR, a Headlined Scoring card can be cancelled by it.
2. **DEFCON limits first** (§7): at DEFCON 3, never Headline a DEFCON-sensitive card while a degrader that would resolve first may be in the opponent's hand; `report` and `limits` flag them.
3. Prefer an own Event worth more **before** Action Round 1 than later: whole-turn effects (Containment, Brezhnev Doctrine, Red Scare/Purge, Vietnam Revolts, Latin American Death Squads, Nuclear Subs, Chernobyl), big Influence swings the opponent cannot answer at once, or Events whose condition holds now and may not later.
4. Work out the order: higher Ops first, US first on ties, Scoring cards last (0 Ops). Ask what the opponent's likely Headline does to mine.
5. Headline an opponent's card only if its Event is close to harmless now; the card's Ops are lost entirely.
6. Space Race box 4 (first arrival): the opponent picks and shows its Headline first.

### 6.3 Action Round procedure
Do these steps in order every time.

**Step 1: Hard filters.** Strike out any play that:
- is illegal: Coup/Realignment in a DEFCON-restricted Region, Influence placed out of reach, an opponent card played as an Event;
- **could lose at DEFCON 1** on my round (§7: the DEFCON 2 table; Battleground Coups at DEFCON 2, unless the US has Nuclear Subs in effect; any Coup under the opponent's Cuban Missile Crisis);
- would leave a Scoring card I hold unplayed by the end of the turn: count my rounds left against the Scoring cards in hand (and under Quagmire/Bear Trap, a Scoring card can only be played when no 2+ Ops card is left);
- Europe Scoring while the opponent Controls Europe.

**Step 2: Emergencies.** These come first if present:
- the opponent can Control Europe: break it at once;
- the opponent is within a few VP of 20, or I am (and a Scoring card can finish it);
- a Battleground I need, or my only route into a Region, is about to fall;
- the Military Ops shortfall would cost more VP than the alternative play gains (last rounds of the turn).

**Step 3: Plan the rest of the hand.** Sort my cards:
- **Scoring cards** I hold: pick the round for each (§6.4); plan the build rounds before.
- **Strong cards to keep for later** (§6.5).
- **Opponent Events to dispose of**, each needing a slot: a moment its Event is harmless, the Space Race (one attempt per turn, 2+ Ops), the held card at turn end, UN Intervention, or the China Card's extra round letting me hold more.
- **DEFCON suicide cards** (§7): get rid of them while DEFCON is 3 or more if the rounds left would force one out later.
- **Ops fodder**: neutral cards and harmless Events.

**Step 4: List candidates.** Run `decide`. For the card chosen, or for each card when torn: the Event (own or neutral) valued with `whatif`; the 1–3 best placements; Coups (each target, `coup`); Realignments (isolated enemy Influence first); the Space Race; The China Card instead. For an opponent's card, choose Event before or after the Ops.

**Step 5: Evaluate.** The bot's evaluation, from my side (`eval` shows the terms):

| Term | Weight (run 5) | What it values |
|---|---|---|
| VP | 1 | current VP |
| Held Scoring card | 0.92 × its current score | a Scoring card in my hand is worth (almost) exactly what it scores now |
| Region standing | 0.32 × score | each Region whose Scoring card is still unseen (not removed), at its score if scored now; Southeast Asia while #38 is live |
| Battlegrounds | 1.18 each | Controlled Battlegrounds (+ mine, − theirs) |
| Near-Control | 1.07 × 1.18 each | a Battleground where a side has Influence, does not Control, and one more Influence would give Control (+ mine, − theirs) |
| Military Ops | 1.10 per missing point | my shortfall (−) and theirs (+), weighted up as the turn goes on |
| China Card | 1.71 (0.6 of it face down) | holding it |
| Europe Control | 15 | Control of Europe (an automatic win when scored) |
| Opponent cards held | −0.3 × (1 + danger/2) | liabilities in my hand |
| DEFCON trap cards | −3 × (1.0 at DEFCON 2, 0.5 at 3, 0.2 at 4) | opponent cards that can lose me the game at DEFCON 2, more if the rounds left force one out |
| Quagmire/Bear Trap | −3 (more with Scoring cards) | being trapped |
| Space Race abilities | 0.5 each | abilities held |
| Influence | 0.05 per point of difference | total Influence |

Evidence: a projection of Final Scoring was **harmful** (removed in run 1; also harmful from turn 8 in run 2, neutral from turn 9–10). Valuing Region standing, Battlegrounds, near-Control, the held Scoring card and Military Ops are essential (removing any of them collapses the bot). Removing the five small terms together (DEFCON traps, Quagmire, liabilities, Influence, Space abilities) cost 5 points (run 5), so they stay.

**Step 6: Look ahead.**
- **Opponent's reply:** for my 3 best candidates, ask what the opponent's best reply is with a typical 2–3 Ops card and any Events still live (unseen pool in `report`). Drop a candidate that hands them a big Coup, a Battleground swing before a Scoring card, Europe Control or 20 VP.
- **Two moves ahead** (run 4: +14.7 points alone, still +13 on top of near-Control): for the 3 best candidates, play out my move, their likely reply and my best next move, and give that line **half** the weight in the choice (full weight was weaker: 58.6 vs 64.7).
- Override the best move only for a clear margin; small differences are within the evaluation's error.

**Step 7: Commit.** Announce the card and its use, roll if needed (`roll`), update `game.json`, run `report`.

### 6.4 Scoring cards: timing
A Scoring card in hand must be played this turn, but I choose the round.
1. **Check every round:** `report` shows each Region's score now.
2. **Automatic results first:** never play Europe Scoring while the opponent Controls Europe; if I Control Europe, play it now. If a score takes me to 20 VP, play it now.
3. **Score now or build first?** Compare two orders of the same two plays:
   - **score now:** lock in the current score; my best other card follows next round;
   - **build first:** play my best other card now; the opponent replies and may spoil; then score.

   Charge each order the opponent's expected reply (they value Region standing, so they may suspect I hold the card). Build when my gain beats their expected spoil; score now when they can spoil more than I can add. (Adopted in run 2: costs nothing and stops leads drifting away.)
4. **Windows with no reply:** the USSR's Headline followed by Action Round 1 (no US round between); the US's last Action Round (nothing follows within the turn). Headlining the Scoring card itself is weaker: it resolves last in the Headline and Defectors can cancel a USSR one.
5. **When it scores against me:** play it when the gap is smallest, usually early, before the opponent builds further; never let it be forced out in the last round at its worst.
6. **Deadline arithmetic:** rounds left ≥ Scoring cards in hand (+ any must-play cards). Under Quagmire/Bear Trap a Scoring card waits until no 2+ Ops card is left.
7. **Shuttle Diplomacy** (US, in effect) lowers the next Middle East or Asia score for the USSR by a Battleground: the helper's scores include it.

### 6.5 Card sequencing: which card now, which later
Late in a turn the opponent holds fewer cards, so their best answer is weaker; after their last round they have none. Greedy play spends the best cards first and ends on scraps.
1. For my top 3 cards (each with its best use) compare **playing it now** (its gain minus the opponent's expected best reply now) with **keeping it** (only **0.44** of its gain trusted to survive, facing the weaker reply they will have later).
2. Play now the card that loses most by waiting: it answers a threat, takes a contested Battleground first, or its target may vanish.
3. Keep strong cards whose value will hold, especially those the opponent could answer if played now.
4. Opponent cards still go early, while their Events are least harmful. Never end the turn holding only bad cards.
5. A card the opponent knows I hold (they saw my hand: CIA Created, Lone Gunman, Aldrich Ames, Grain Sales) gets no credit for being kept.
6. Evidence: sequencing was the largest gain of run 2 (56.9 ± 1.9); trusting 0.3 or 0.7 was no better than 0.5; run 5 tuning moved it to 0.44.

### 6.6 End of turn and the China Card
- **Held card:** an opponent card I could not dispose of safely, or a strong own card for next turn. Never a Scoring card. Space Race box 6 (first arrival) lets me discard one held card.
- **Military Ops:** requirement = DEFCON at the end of the turn; Coups both count toward it and may lower it (Battleground Coups lower DEFCON, so the requirement falls with it).
- **The China Card:** hold it as a safety valve (an extra round to keep a card back, +1 VP at the end); play it for a big swing in Asia (5 Ops there) or when the extra Ops decide a Battleground or a Scoring card. Holding value confirmed in run 2 (53.1 ± 2.1 on fresh deals) and raised by run 5's tuning.
- **Turn 10:** there is no future. Play for the Final Scoring projection (`report`) in the last rounds; whole-turn Events weaken late in the turn; the US moves last.

### 6.7 Known weak spot
**Damage from opponent Events.** About 3 evaluation points per game per side are lost by firing an opponent's Event at a worse moment than one already passed (37–50% of fired opponent cards had a free moment while held). The worst offenders: Five Year Plan (for the USSR), Arab-Israeli War and Korean War (for the US). A simple rule "play it while its Event is harmless" was neutral in run 5, so judge each case: in Step 3, give each opponent card a slot, and prefer the round where `whatif` shows its Event costs least.

---

## 7. DEFCON limits (hard limits)

Basis: DEFCON 1 on my own round (Action Round, or my Headline) loses the game (8.1.3). Sources: Twilight Strategy's DEFCON article and card articles (e.g. "you can't play Olympic Games for the Event at DEFCON 2: your opponent will boycott and you lose"). The bot removes these plays before evaluating anything, and makes one only if nothing else is legal. Effect: DEFCON losses in 400 self-play games fell from 19 to 1 (run 3). `report` and `limits` apply this section to my hand.

**Action Rounds at DEFCON 2: never, on my own round:**

| Cards | Never |
|---|---|
| #4 Duck and Cover (US), #50 "We Will Bury You" (USSR), #89 Soviets Shoot Down KAL-007 (US) | trigger the Event, as my own Event or by playing the opponent's card for Ops |
| #26 CIA Created (US), #62 "Lone Gunman" (USSR), #67 Grain Sales to Soviets (US), #96 Tear Down This Wall (US; its Coup ignores the Europe limit) | trigger the Event if the opponent could then Coup a Battleground where I have Influence (no danger under my Cuban Missile Crisis, or vs a US with Nuclear Subs) |
| #91 Ortega Elected in Nicaragua (USSR) | as US, play it for Ops while I have Influence in Cuba (its free Coup next to Nicaragua) |
| #20 Olympic Games, #45 Summit, #49 Missile Envy | play for the Event (Ops are fine) |
| #5 Five Year Plan (US) | as US: the Event while Duck and Cover or KAL-007 may be in the USSR hand. As USSR: play it for Ops while my hand holds Duck and Cover, KAL-007, or CIA Created / Grain Sales / Tear Down This Wall with a Battleground Coup open to the US |
| Any Battleground Coup (by Ops or a free Coup) | unless the US has Nuclear Subs in effect |

**Headline:** a Headline can lose even at DEFCON 3: if the other Headline resolves first (higher Ops, US first on ties) and lowers DEFCON to 2, my DEFCON-sensitive Headline then takes it to 1 on my turn.
- At DEFCON 3: never Headline a card from the table while a degrader that would resolve first may be in the opponent's hand: #4, #50, #89 (either side may Headline the other's card just to drop DEFCON), #46 How I Learned to Stop Worrying, #40 Cuban Missile Crisis, #47 Junta, and the opponent's own Coup-giver (#26 for the US, #62 for the USSR).
- At DEFCON 4 or 5: the same, but only Cuban Missile Crisis and How I Learned to Stop Worrying can bring DEFCON to 2 in one Headline.

**Hand planning:**
- Get rid of suicide cards while DEFCON is still 3 or more if the rounds left exceed my safe cards (+ The China Card): one would be forced out at DEFCON 2.
- The 8th Action Round (Space Station, North Sea Oil) is optional: not playing it lets a suicide card be held to the next turn.
- Other exits: the Space Race (2+ Ops), UN Intervention, raising DEFCON first (SALT Negotiations, Nuclear Test Ban, ABM Treaty, Glasnost), Headlining it at DEFCON 4+ when no degrader can come first.

---

## 8. Card notes

Short practical notes, from the simulator diagnostics and Twilight Strategy (to be extended in Stage 3).
- **DEFCON cards:** see §7. Duck and Cover and "We Will Bury You" are also weapons: at DEFCON 3 they lock Asia and the Middle East for both sides (and Europe already) and raise everyone's Military Ops pressure.
- **Most often fired at a bad moment** (simulator): Five Year Plan (USSR holding it), Arab-Israeli War and Korean War (US holding them). Plan their slot from the start of the turn.
- **Quagmire / Bear Trap:** keep 2+ Ops cards to escape; never sit in it with a Scoring card and too few rounds.
- **Cuban Missile Crisis:** under the opponent's crisis, do not Coup at all; the defender may cancel it by removing 2 Influence (USSR: Cuba; US: W. Germany or Turkey).
- **The China Card:** see §6.6. Formosan Resolution ends when the US plays it.
- **UN Intervention:** pairs with an opponent card from my own hand; with "We Will Bury You" it also stops the 3 VP; with U2 Incident in effect it gives the USSR 1 VP.
- **Defectors (US):** its threat makes a USSR Scoring Headline risky.
- **War cards:** check the roll modifiers (adjacent enemy Control) before relying on them; the Military Ops come anyway.

---

## 9. Strategy (Stage 3, to come)

Not yet designed: game plans built from the Stage 2 evidence, Alfy's strategic hypotheses still open (H11 reading the opponent's Scoring cards from their spending; H12 Europe investment by side and era; H3 USSR attack in the Early War), difficulty levels 1–5 and the simulator as move advisor. Evidence so far:
- **What carries the bot:** Region standing, Battlegrounds with near-Control, the held Scoring card, Military Ops. Without any one of them it collapses.
- **Near-Control** (a Battleground one Influence short of Control) was the largest single gain: either side can flip such a country with one placement.
- **Patience in moderation:** keep strong cards, but trust only about half of their value to survive.
- **Self-play balance** at the current strength favours the USSR even with the +2 US handicap.

---

## 10. Game sessions (Stage 4, to come; current defaults)

- **At the start:** ask Alfy for his side, the handicap (none, bidding, or fixed +2 US: default +2 US), and whether this is a competitive game or a learning game (in a learning game I explain my reasoning; in a competitive one I keep my plans and hand private).
- **I track everything** in `game.json` and keep a short **log** of every play (turn, round, player, card, use, result). Announce each play clearly: card, Event or Ops, and every placement, Coup or roll.
- **Hidden information:** my hand stays private (never shown in replies unless an Event reveals it). Dice for both sides through `roll`; Alfy may roll physical dice for his own rolls and tell me the result. Shuffles and deals: Alfy deals from his physical copy and tells me my cards, or I deal with the helper's random draws if we play without the physical game.
- **What I know of his hand:** record it in `opp_known` (CIA Created, Lone Gunman, Aldrich Ames, Grain Sales, Missile Envy, Cambridge Five) and drop cards when played.
- **Difficulty:** levels are not calibrated yet (Stage 3). Until then I play at full strength; if Alfy asks for an easier game, I say so and play more simply (for example, skip the two-move look-ahead).
- **Saving:** after every play, save `game.json` and the log. At the end of a session, commit them to `AlfyBurger/TS-sim` under `games/<date>/` so the game can continue in another chat.

---

## 11. Helper script

Write this to `tsts.py` at the start of a game (Python 3, standard library only). The `effects` names in `game.json`: lasting `marshall, warsaw, nato, de_gaulle, willy_brandt, us_japan, formosan, camp_david, jp2, flower_power, evil_empire, iron_lady, north_sea_oil, reformer, tear_down, awacs, norad, iranian_hostage, shuttle, quagmire, bear_trap`; this turn `containment, brezhnev, vietnam_revolts, nuclear_subs, salt, yuri, iran_contra, nso_turn, u2`, plus `"red_scare": ["USSR"]` (the side whose cards are −1), `"lads": "US"` (who played Latin American Death Squads), `"chernobyl": "Europe"`, `"cmc": "USSR"` (who played Cuban Missile Crisis). Clear the this-turn effects at the end of each turn.

Cross-check: `tssim xcheck 300 1 > xc.jsonl` in the simulator, then `python3 tsts.py xcheck xc.jsonl`: Control, Region and Southeast Asia scores, Scoring-card values, Final projection, rounds per turn, evaluation of both sides and the best Ops choice and value of every card agree on all 300 positions (18 Nov 2026).

```python
{HELPER}
```
