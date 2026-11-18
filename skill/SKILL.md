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

**Simulator:** Alfy's private repository `AlfyBurger/TS-sim` (compiled Rust engine, test reports in `reports/`, this skill's sources in `skill/`). The helper script in §11 reproduces the simulator's evaluation and Ops choices exactly (cross-checked on 300 random positions, 0 differences).

Contents: 1 Rules core · 2 Map · 3 Tracks and scoring · 4 Cards · 5 Rulings and open questions · 6 Playing: move-by-move decisions · 7 DEFCON limits · 8 Card notes · 9 Strategy (Stage 3, to come) · 10 Game sessions · 11 Helper script

---

## 1. Rules core

### Setup [3]
- Shuffle the 38 Early War cards (The China Card excluded); deal 8 to each player, USSR first. The China Card starts face up with the USSR. Players see their hands before placing Influence.
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

**Europe** (21 countries, 5 Battlegrounds)

| Country | Stab | | Adjacent to |
|---|---|---|---|
| Austria | 4 | EE WE | E. Germany, Hungary, Italy, W. Germany |
| Benelux | 3 | WE | U.K., W. Germany |
| Bulgaria | 3 | EE | Greece, Turkey |
| Canada | 4 | WE | U.K., USA |
| Czechoslovakia | 3 | EE | E. Germany, Hungary, Poland |
| Denmark | 3 | WE | Sweden, W. Germany |
| E. Germany | 3 | **B** EE | Austria, Czechoslovakia, Poland, W. Germany |
| Finland | 4 | EE WE | Sweden, USSR |
| France | 3 | **B** WE | Algeria, Italy, Spain/Portugal, U.K., W. Germany |
| Greece | 2 | WE | Bulgaria, Italy, Turkey, Yugoslavia |
| Hungary | 3 | EE | Austria, Czechoslovakia, Romania, Yugoslavia |
| Italy | 2 | **B** WE | Austria, France, Greece, Spain/Portugal, Yugoslavia |
| Norway | 4 | WE | Sweden, U.K. |
| Poland | 3 | **B** EE | Czechoslovakia, E. Germany, USSR |
| Romania | 3 | EE | Hungary, Turkey, Yugoslavia, USSR |
| Spain/Portugal | 2 | WE | France, Italy, Morocco |
| Sweden | 4 | WE | Denmark, Finland, Norway |
| Turkey | 2 | WE | Bulgaria, Greece, Romania, Syria |
| U.K. | 5 | WE | Benelux, Canada, France, Norway |
| W. Germany | 4 | **B** WE | Austria, Benelux, Denmark, E. Germany, France |
| Yugoslavia | 3 | EE | Greece, Hungary, Italy, Romania |

**Asia** (15 countries, 6 Battlegrounds)

| Country | Stab | | Adjacent to |
|---|---|---|---|
| Afghanistan | 2 |  | Iran, Pakistan, USSR |
| Australia | 4 |  | Malaysia |
| Burma | 2 | SEA | India, Laos/Cambodia |
| India | 3 | **B** | Burma, Pakistan |
| Indonesia | 1 | SEA | Malaysia, Philippines |
| Japan | 4 | **B** | Philippines, S. Korea, Taiwan, USA |
| Laos/Cambodia | 1 | SEA | Burma, Thailand, Vietnam |
| Malaysia | 2 | SEA | Australia, Indonesia, Thailand |
| N. Korea | 3 | **B** | S. Korea, USSR |
| Pakistan | 2 | **B** | Afghanistan, India, Iran |
| Philippines | 2 | SEA | Indonesia, Japan |
| S. Korea | 3 | **B** | Japan, N. Korea, Taiwan |
| Taiwan | 3 |  | Japan, S. Korea |
| Thailand | 2 | **B** SEA | Laos/Cambodia, Malaysia, Vietnam |
| Vietnam | 1 | SEA | Laos/Cambodia, Thailand |

**Middle East** (10 countries, 6 Battlegrounds)

| Country | Stab | | Adjacent to |
|---|---|---|---|
| Egypt | 2 | **B** | Israel, Libya, Sudan |
| Gulf States | 3 |  | Iraq, Saudi Arabia |
| Iran | 2 | **B** | Afghanistan, Iraq, Pakistan |
| Iraq | 3 | **B** | Gulf States, Iran, Jordan, Saudi Arabia |
| Israel | 4 | **B** | Egypt, Jordan, Lebanon, Syria |
| Jordan | 2 |  | Iraq, Israel, Lebanon, Saudi Arabia |
| Lebanon | 1 |  | Israel, Jordan, Syria |
| Libya | 2 | **B** | Egypt, Tunisia |
| Saudi Arabia | 3 | **B** | Gulf States, Iraq, Jordan |
| Syria | 2 |  | Israel, Lebanon, Turkey |

**Africa** (18 countries, 5 Battlegrounds)

| Country | Stab | | Adjacent to |
|---|---|---|---|
| Algeria | 2 | **B** | France, Morocco, Saharan States, Tunisia |
| Angola | 1 | **B** | Botswana, South Africa, Zaire |
| Botswana | 2 |  | Angola, South Africa, Zimbabwe |
| Cameroon | 1 |  | Nigeria, Zaire |
| Ethiopia | 1 |  | Somalia, Sudan |
| Ivory Coast | 2 |  | Nigeria, West African States |
| Kenya | 2 |  | SE African States, Somalia |
| Morocco | 3 |  | Algeria, Spain/Portugal, West African States |
| Nigeria | 1 | **B** | Cameroon, Ivory Coast, Saharan States |
| SE African States | 1 |  | Kenya, Zimbabwe |
| Saharan States | 1 |  | Algeria, Nigeria |
| Somalia | 2 |  | Ethiopia, Kenya |
| South Africa | 3 | **B** | Angola, Botswana |
| Sudan | 1 |  | Egypt, Ethiopia |
| Tunisia | 2 |  | Algeria, Libya |
| West African States | 2 |  | Ivory Coast, Morocco |
| Zaire | 1 | **B** | Angola, Cameroon, Zimbabwe |
| Zimbabwe | 1 |  | Botswana, SE African States, Zaire |

**Central America** (10 countries, 3 Battlegrounds)

| Country | Stab | | Adjacent to |
|---|---|---|---|
| Costa Rica | 3 |  | Honduras, Nicaragua, Panama |
| Cuba | 3 | **B** | Haiti, Nicaragua, USA |
| Dominican Rep | 1 |  | Haiti |
| El Salvador | 1 |  | Guatemala, Honduras |
| Guatemala | 1 |  | El Salvador, Honduras, Mexico |
| Haiti | 1 |  | Cuba, Dominican Rep |
| Honduras | 2 |  | Costa Rica, El Salvador, Guatemala, Nicaragua |
| Mexico | 2 | **B** | Guatemala, USA |
| Nicaragua | 1 |  | Costa Rica, Cuba, Honduras |
| Panama | 2 | **B** | Colombia, Costa Rica |

**South America** (10 countries, 4 Battlegrounds)

| Country | Stab | | Adjacent to |
|---|---|---|---|
| Argentina | 2 | **B** | Chile, Paraguay, Uruguay |
| Bolivia | 2 |  | Paraguay, Peru |
| Brazil | 2 | **B** | Uruguay, Venezuela |
| Chile | 3 | **B** | Argentina, Peru |
| Colombia | 1 |  | Ecuador, Panama, Venezuela |
| Ecuador | 2 |  | Colombia, Peru |
| Paraguay | 2 |  | Argentina, Bolivia, Uruguay |
| Peru | 2 |  | Bolivia, Chile, Ecuador |
| Uruguay | 2 |  | Argentina, Brazil, Paraguay |
| Venezuela | 2 | **B** | Brazil, Colombia |

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

Format: **#id Title** · `*` removed after Event · `[U]` lasting effect · Ops. Effects are short summaries in my own words; full rulings in §5. Deck: Early War 38 + The China Card, Mid War 48, Late War 23 (#103–110 included in their eras). Card #6 is The China Card.

### US cards
**Early War**
- **#4 Duck and Cover** · 3 · Degrade DEFCON by 1, then the US gains (5 − DEFCON) VP.
- **#5 Five Year Plan** · 3 · The USSR discards a random card. A US Event on it happens (implemented by the US); any other card is just discarded (a Scoring card is not scored).
- **#19 Truman Doctrine** \* · 1 · Remove all USSR Influence from one uncontrolled country in Europe.
- **#21 NATO** \* [U] · 4 · Playable only after Marshall Plan or Warsaw Pact Formed. The USSR may no longer Coup or Realign in US-Controlled European countries, and Brush War cannot target them.
- **#22 Independent Reds** \* · 2 · Add US Influence in one of Yugoslavia, Romania, Bulgaria, Hungary or Czechoslovakia until it equals the USSR Influence there.
- **#23 Marshall Plan** \* [U] · 4 · Add 1 US Influence in each of 7 Western European countries the USSR does not Control. Allows NATO.
- **#25 Containment** \* · 3 · Rest of turn: US cards played for Ops get +1 (max 4).
- **#26 CIA Created** \* · 1 · The USSR shows its hand (once, now). Then the US conducts 1 Op (not the Space Race).
- **#27 US/Japan Mutual Defense Pact** \* [U] · 4 · The US adds enough Influence to Control Japan. The USSR may no longer Coup or Realign in Japan.
- **#29 East European Unrest** · 3 · Remove 1 USSR Influence from each of 3 Eastern European countries (2 each in the Late War).
- **#35 Formosan Resolution** \* [U] · 2 · While in effect, Taiwan counts as a Battleground when the US Controls it, for Asia Scoring and Final Scoring. Cancelled when the US plays The China Card.
- **#103 Defectors** · 2 · Headlined by the US, it cancels the USSR Headline (even a Scoring card, which is discarded). If the USSR plays it in its Action Round (not on the Space Race), the US gains 1 VP.
- **#105 Special Relationship** · 2 · If the US Controls the U.K.: without NATO, add 1 US Influence in a country adjacent to the U.K.; with NATO, add 2 US Influence in a Western European country and gain 2 VP.
- **#106 NORAD** \* [U] · 3 · While in effect: at the end of any Action Round in which DEFCON dropped to 2, if the US Controls Canada, the US adds 1 Influence in a country with US Influence. Cancelled by Quagmire.

**Mid War**
- **#41 Nuclear Subs** \* · 2 · Rest of turn: US Coups in Battlegrounds do not lower DEFCON (Cuban Missile Crisis still applies; Events can still move DEFCON).
- **#44 Bear Trap** \* [U] · 3 · Mirror of Quagmire against the USSR: discard a 2+ Ops card and roll 1–4 each Action Round until it succeeds; with no such card, only Scoring cards.
- **#48 Kitchen Debates** \* · 1 · If the US Controls more Battlegrounds than the USSR: US gains 2 VP. If not, the Event does not occur and the card stays in the game.
- **#63 Colonial Rear Guards** · 2 · Add 1 US Influence in each of 4 countries in Africa and/or Southeast Asia.
- **#64 Panama Canal Returned** \* · 1 · Add 1 US Influence in Panama, Costa Rica and Venezuela.
- **#65 Camp David Accords** \* [U] · 2 · US gains 1 VP and adds 1 Influence in Israel, Jordan and Egypt. Arab-Israeli War can no longer be an Event.
- **#66 Puppet Governments** \* · 2 · Add 1 US Influence in each of 3 countries with no Influence from either side.
- **#67 Grain Sales to Soviets** · 2 · The US takes a random card from the USSR hand and may play it (any use, including the Space Race) or return it and use Grain Sales for Ops.
- **#68 John Paul II Elected Pope** \* [U] · 2 · Remove 2 USSR Influence from Poland and add 1 US Influence there. Allows Solidarity.
- **#70 OAS Founded** \* · 1 · Add 2 US Influence in Central and/or South America.
- **#71 Nixon Plays the China Card** \* · 2 · If the US holds The China Card: US gains 2 VP. Otherwise the US takes it face down (not playable this turn).
- **#72 Sadat Expels Soviets** \* · 1 · Remove all USSR Influence from Egypt and add 1 US Influence there.
- **#73 Shuttle Diplomacy** · 3 · At the next Middle East or Asia Scoring (not Southeast Asia, not Final Scoring), the US picks one USSR-Controlled Battleground there to ignore entirely; then the card is discarded.
- **#74 The Voice of America** · 2 · Remove 4 USSR Influence from countries outside Europe, at most 2 per country.
- **#76 Ussuri River Skirmish** \* · 3 · If the USSR holds The China Card, the US takes it face up; if the US already holds it, add 4 US Influence in Asia, at most 2 per country.
- **#77 “Ask Not What Your Country…”** \* · 3 · The US discards any number of cards (Scoring cards allowed) and then draws the same number.
- **#78 Alliance for Progress** \* · 3 · US gains 1 VP per US-Controlled Battleground in Central and South America.
- **#108 Our Man in Tehran** \* · 2 · If the US Controls a Middle East country: the US looks at the top 5 cards of the draw pile, discards any of them (shown, no Events), returns the rest and reshuffles the draw pile. If not, the card is discarded and stays in the game.

**Late War**
- **#83 The Iron Lady** \* [U] · 3 · US gains 1 VP; add 1 USSR Influence in Argentina; remove all USSR Influence from the U.K. Socialist Governments can no longer be an Event.
- **#84 Reagan Bombs Libya** \* · 2 · US gains 1 VP for every 2 USSR Influence in Libya.
- **#85 Star Wars** \* · 2 · If the US is ahead on the Space Race: take any non-Scoring card from the discard pile and play its Event now.
- **#86 North Sea Oil** \* [U] · 3 · OPEC can no longer be an Event. The US may take an 8th Action Round this turn.
- **#89 Soviets Shoot Down KAL-007** \* · 4 · DEFCON −1 and the US gains 2 VP. If the US Controls South Korea, it then places Influence or Realigns with 4 Ops.
- **#94 Chernobyl** \* · 3 · The US names a Region: rest of turn the USSR cannot place Influence there with Ops (Events, Coups and Realignments still allowed).
- **#96 Tear Down this Wall** \* [U] · 3 · Cancels or prevents Willy Brandt. Add 3 US Influence in E. Germany, then a free Coup or Realignment rolls (this card's Ops) in Europe, ignoring DEFCON's Europe limit.
- **#97 “An Evil Empire”** \* [U] · 3 · US gains 1 VP. Cancels or prevents Flower Power.
- **#101 Solidarity** \* · 2 · Only while John Paul II Elected Pope is in effect: add 3 US Influence in Poland.
- **#110 AWACS Sale to Saudis** \* [U] · 3 · Add 2 US Influence in Saudi Arabia. Muslim Revolution can no longer be an Event.

### USSR cards
**Early War**
- **#7 Socialist Governments** · 3 · Remove 3 US Influence in total from Western Europe, at most 2 per country. Not playable as an Event while The Iron Lady is in effect.
- **#8 Fidel** \* · 2 · Remove all US Influence from Cuba; the USSR adds enough Influence there for Control.
- **#9 Vietnam Revolts** \* · 2 · Add 2 USSR Influence in Vietnam. Rest of turn: a USSR card whose Ops are all spent in Southeast Asia gets +1 Ops.
- **#10 Blockade** \* · 1 · Unless the US at once discards a card worth 3+ Ops, remove all US Influence from W. Germany.
- **#11 Korean War** \* · 2 · War: North Korea attacks South Korea. Roll, −1 per US-Controlled country adjacent to South Korea; 4–6: USSR gains 2 VP and all US Influence in South Korea becomes USSR Influence. USSR +2 Military Ops either way.
- **#12 Romanian Abdication** \* · 1 · Remove all US Influence from Romania; the USSR adds enough for Control.
- **#13 Arab-Israeli War** · 2 · War: attack on Israel. Roll, −1 if the US Controls Israel and −1 per US-Controlled country adjacent to Israel; 4–6: USSR gains 2 VP and takes over all US Influence in Israel. USSR +2 Military Ops. Not playable as an Event after Camp David Accords.
- **#14 Comecon** \* · 3 · Add 1 USSR Influence in each of 4 Eastern European countries the US does not Control.
- **#15 Nasser** \* · 1 · Add 2 USSR Influence in Egypt and remove half the US Influence there (rounded up).
- **#16 Warsaw Pact Formed** \* [U] · 3 · Either remove all US Influence from 4 Eastern European countries, or add 5 USSR Influence in Eastern Europe (at most 2 per country). Allows NATO.
- **#17 De Gaulle Leads France** \* [U] · 3 · Remove 2 US Influence from France and add 1 USSR Influence there. NATO no longer protects France.
- **#28 Suez Crisis** \* · 3 · Remove 4 US Influence in total from France, the U.K. and Israel, at most 2 per country.
- **#30 Decolonization** · 2 · Add 1 USSR Influence in each of 4 countries in Africa and/or Southeast Asia.
- **#33 De-Stalinization** \* · 3 · The USSR moves up to 4 of its Influence from any countries to countries the US does not Control, at most 2 per destination.
- **#104 The Cambridge Five** · 2 · The US shows the Scoring cards in its hand; the USSR adds 1 Influence in one Region named on them. Not playable as an Event in the Late War.

**Mid War**
- **#42 Quagmire** \* [U] · 3 · From the US's next Action Round: the US must discard a card worth 2+ Ops and roll 1–4 to end it, every round until it succeeds; with no such card it may play only Scoring cards. Carries over into later turns; does not affect the Headline. Cancels NORAD.
- **#50 “We Will Bury You”** \* · 4 · DEFCON −1. Unless the US plays UN Intervention as an Event in its next round, the USSR gains 3 VP.
- **#51 Brezhnev Doctrine** \* · 3 · Rest of turn: USSR cards played for Ops get +1 (max 4).
- **#52 Portuguese Empire Crumbles** \* · 2 · Add 2 USSR Influence in SE African States and 2 in Angola.
- **#53 South African Unrest** · 2 · Add 2 USSR Influence in South Africa, or 1 there and 2 in countries adjacent to South Africa (may be split).
- **#54 Allende** \* · 1 · Add 2 USSR Influence in Chile.
- **#55 Willy Brandt** \* [U] · 2 · USSR gains 1 VP and adds 1 Influence in W. Germany; NATO no longer protects W. Germany. Prevented or cancelled by Tear Down This Wall.
- **#56 Muslim Revolution** · 4 · Remove all US Influence from 2 of: Sudan, Iran, Iraq, Egypt, Libya, Saudi Arabia, Syria, Jordan. Not playable as an Event after AWACS Sale.
- **#58 Cultural Revolution** \* · 3 · If the US holds The China Card, the USSR takes it face up; if the USSR already holds it, the USSR gains 1 VP.
- **#59 Flower Power** \* [U] · 4 · From now on the USSR gains 2 VP each time the US plays a War card (Arab-Israeli, Korean, Brush, Indo-Pakistani, Iran-Iraq), as Event or Ops, but not on the Space Race. Cancelled by An Evil Empire.
- **#60 U2 Incident** \* · 3 · USSR gains 1 VP, and 1 more if UN Intervention is played as an Event later this turn.
- **#61 OPEC** · 3 · USSR gains 1 VP for each of Egypt, Iran, Libya, Saudi Arabia, Iraq, Gulf States and Venezuela it Controls. Not playable as an Event after North Sea Oil.
- **#62 “Lone Gunman”** \* · 1 · The US shows its hand (once, now). Then the USSR conducts 1 Op (not the Space Race).
- **#75 Liberation Theology** · 2 · Add 3 USSR Influence in Central America, at most 2 per country.
- **#107 Che** · 3 · USSR Coup (this card's Ops) against a non-Battleground in Central America, South America or Africa; if it removed US Influence, a second Coup against another such country.

**Late War**
- **#82 Iranian Hostage Crisis** \* [U] · 3 · Remove all US Influence from Iran and add 2 USSR Influence there. Terrorism played by the USSR makes the US discard 2 cards.
- **#87 The Reformer** \* [U] · 3 · Add 4 USSR Influence in Europe (6 if the USSR leads on VP), at most 2 per country. The USSR may no longer Coup in Europe. Strengthens Glasnost.
- **#88 Marine Barracks Bombing** \* · 2 · Remove all US Influence from Lebanon and 2 more US Influence anywhere in the Middle East.
- **#90 Glasnost** \* · 4 · USSR gains 2 VP and DEFCON improves by 1. With The Reformer in effect, the USSR also places Influence or Realigns with 4 Ops.
- **#91 Ortega Elected in Nicaragua** \* · 2 · Remove all US Influence from Nicaragua; then a free USSR Coup (this card's Ops) in a country adjacent to Nicaragua.
- **#93 Iran-Contra Scandal** \* · 2 · Rest of turn: US Realignment rolls −1.
- **#95 Latin American Debt Crisis** · 2 · Unless the US at once discards a card worth 3+ Ops, the USSR doubles its Influence in 2 South American countries.
- **#98 Aldrich Ames Remix** \* · 3 · The US plays with its hand face up for the rest of the turn; the USSR discards one card of its choice from it.
- **#99 Pershing II Deployed** \* · 3 · USSR gains 1 VP; remove 1 US Influence from each of up to 3 Western European countries.
- **#109 Yuri and Samantha** \* · 2 · Rest of turn: the USSR gains 1 VP for each US Coup.

### Neutral cards (either side)
**Early War**
- **#1 Asia Scoring** · Scoring · Score Asia (Presence 3, Domination 7, Control 9; +1 per Battleground, +1 per Controlled country adjacent to the enemy superpower). Southeast Asia countries count.
- **#2 Europe Scoring** · Scoring · Score Europe (Presence 3, Domination 7; Control of Europe wins the game; same bonuses as Asia).
- **#3 Middle East Scoring** · Scoring · Score the Middle East (Presence 3, Domination 5, Control 7; +1 per Battleground; no superpower bonus).
- **#6 The China Card** · 4 · The China Card: see §1 China Card.
- **#18 Captured Nazi Scientist** \* · 1 · The player's Space Race marker advances 1 box (not a Space Race attempt).
- **#20 Olympic Games** · 2 · The opponent chooses: take part (both roll; the sponsor adds 2; higher roll gains 2 VP; re-roll ties) or boycott (DEFCON −1 and the sponsor conducts 4 Ops). At DEFCON 2 the boycott ends the game against the sponsor.
- **#24 Indo-Pakistani War** · 2 · War: India or Pakistan attacks the other (player's choice). Roll, −1 per opponent-Controlled country adjacent to the target; 4–6: 2 VP and take over the opponent's Influence in the target. +2 Military Ops.
- **#31 Red Scare/Purge** · 4 · Rest of turn: the opponent's cards are −1 Ops (min 1), also for cards they discard to Events or the Space Race.
- **#32 UN Intervention** · 1 · Play together with a card from your own hand carrying the opponent's Event: that Event is cancelled and you use its Ops; the cancelled card is discarded. Never in the Headline.
- **#34 Nuclear Test Ban** · 4 · The player gains (DEFCON − 2) VP, then DEFCON improves by 2.

**Mid War**
- **#36 Brush War** · 3 · War: attack a country of Stability 1 or 2. Roll, −1 per adjacent enemy-Controlled country (an adjacent enemy superpower counts); 3–6: 1 VP and take over the opponent's Influence there. +3 Military Ops. Not against NATO-protected countries.
- **#37 Central America Scoring** · Scoring · Score Central America (Presence 1, Domination 3, Control 5; +1 per Battleground; +1 per Controlled country adjacent to the enemy superpower).
- **#38 Southeast Asia Scoring** \* · Scoring · Score Southeast Asia: 1 VP per Controlled country (Burma, Laos/Cambodia, Vietnam, Malaysia, Indonesia, Philippines), 2 for Thailand. The only Scoring card removed after play; not scored separately at Final Scoring.
- **#39 Arms Race** · 3 · If the Phasing Player has more Military Ops than the opponent: 1 VP, or 3 VP if it has also met its requirement.
- **#40 Cuban Missile Crisis** \* · 3 · DEFCON becomes 2. Rest of turn, any Coup by the opponent loses them the game. The USSR can end it by removing 2 of its Influence from Cuba; the US by removing 2 from W. Germany or Turkey.
- **#43 SALT Negotiations** \* · 3 · DEFCON improves by 2. Rest of turn: all Coups −1. The player may take one non-Scoring card from the discard pile (shown to the opponent).
- **#45 Summit** · 1 · Both roll, +1 per Region each Dominates or Controls. The higher roll gains 2 VP and may move DEFCON 1 step either way. Ties: nothing.
- **#46 How I Learned to Stop Worrying** \* · 2 · The player sets DEFCON to any level 1–5 (1 loses for the Phasing Player) and counts 5 Military Ops for this turn.
- **#47 Junta** · 2 · Add 2 Influence in one Central or South American country, then make a free Coup or Realignment rolls (this card's Ops) in Central or South America.
- **#49 Missile Envy** · 2 · Swap this card for the opponent's highest-Ops card (they choose among ties). If its Event is yours or neutral it happens now; if it is the opponent's, use its Ops without the Event. The opponent must play Missile Envy for Ops in their next round.
- **#57 ABM Treaty** · 4 · DEFCON improves by 1, then the player conducts 4 Ops (not the Space Race).
- **#69 Latin American Death Squads** · 2 · Rest of turn: the player's Coups in Central and South America +1, the opponent's −1.
- **#79 Africa Scoring** · Scoring · Score Africa (Presence 1, Domination 4, Control 6; +1 per Battleground).
- **#80 “One Small Step…”** · 2 · If behind on the Space Race, advance 2 boxes, gaining only the second box's VP.
- **#81 South America Scoring** · Scoring · Score South America (Presence 2, Domination 5, Control 6; +1 per Battleground).

**Late War**
- **#92 Terrorism** · 2 · The opponent discards a random card (the US discards 2 if the USSR plays it with Iranian Hostage Crisis in effect). Events on discards do not happen.
- **#100 Wargames** \* · 4 · Only at DEFCON 2: the player may end the game at once, after giving the opponent 6 VP (no Final Scoring). Otherwise nothing happens; at DEFCON 3–5 the card stays in the game.
- **#102 Iran-Iraq War** \* · 2 · War: Iran or Iraq attacks the other (player's choice). Roll, −1 per enemy-Controlled country adjacent to the target; 4–6: 2 VP and take over the opponent's Influence there. +2 Military Ops.

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

This is the procedure of the simulator's best bot after Stage 2 run 5 (18 Nov 2026), written so I can apply it at the table. Each rule below was tested in simulation (1,000 deals per side unless stated, margins 95%); the evidence is given where it matters. How the bot grew, run by run: run 1 removed a harmful Final Scoring projection (56.8% vs the first bot); run 2 added card sequencing and Scoring timing (60% vs run 1); run 3 the DEFCON limits (DEFCON losses 19 → 1 in 400 games); run 4 near-Control credit and a two-move look-ahead (95% vs run 3); run 5 tuned the weights (52.5% vs run 4).

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
1. The China Card cannot be Headlined. As USSR: a US Headline of **Defectors** cancels my Headline, even a Scoring card, so while Defectors is unseen a Scoring card Headline is risky.
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

Cross-check: `tssim xcheck 300 1 > xc.jsonl` in the simulator, then `python3 tsts.py xcheck xc.jsonl`: Control, Region and Southeast Asia scores, Scoring-card values, Final projection, rounds per turn, evaluation of both sides and the best Ops choice and value of every card agree on all 300 positions (18 Nov 2026). The same file is in the repo as `skill/tsts.py`.

```python
#!/usr/bin/env python3
"""tsts.py - Twilight Struggle helper (2026 HoF rules, base game, 110 cards). Stdlib only.
State: a JSON file (game.json). Influence as [US, USSR]; VP signed, + = US; sides "US"/"USSR".
Commands (GAME = path to game.json; SIDE = US or USSR):
  init GAME MYSIDE [HANDICAP]      new game: fixed setup Influence (free placements still to add)
  add GAME COUNTRY SIDE N          add (or remove, N<0) Influence; prints Control
  report GAME                      full position report for "me": Regions, scoring now, Scoring cards, hand flags
  score GAME [REGION|all]          Region scoring if played now
  coup GAME SIDE COUNTRY OPS [--free]   exact Coup outcomes and odds
  realign GAME SIDE COUNTRY [--free]    Realignment odds
  place GAME SIDE "A,A,B" [--apply]     check legality and cost of a placement (one name per marker)
  eval GAME [SIDE] [--w=k=v,...]   evaluation with its terms (simulator's default bot weights)
  whatif GAME SIDE JSON            value of a change, e.g. '{"inf":{"Italy":[0,2]},"vp":-1,"defcon":-1,"play":[17]}'
  decide GAME [--w=k=v,...]        best Ops use of every card in my hand, ranked (no Events, no reply)
  limits GAME                      DEFCON hard limits for my hand
  roll [N]                         roll N dice
  xcheck FILE                      compare with the simulator (`tssim xcheck N seed > FILE`)
"""
COUNTRY_TABLE = '''
Austria|0|4|EW|6,10,11,19
Benelux|0|3|W|18,19
Bulgaria|0|3|E|9,17
Canada|0|4|WU|18
Czechoslovakia|0|3|E|6,10,13
Denmark|0|3|W|16,19
E. Germany|0|3|EB|0,4,13,19
Finland|0|4|EWR|16
France|0|3|WB|46,11,15,18,19
Greece|0|2|W|2,11,17,20
Hungary|0|3|E|0,4,14,20
Italy|0|2|WB|0,8,9,15,20
Norway|0|4|W|16,18
Poland|0|3|EBR|4,6
Romania|0|3|ER|10,17,20
Spain/Portugal|0|2|W|8,11,53
Sweden|0|4|W|5,7,12
Turkey|0|2|W|2,9,14,45
U.K.|0|5|W|1,3,8,12
W. Germany|0|4|WB|0,1,5,6,8
Yugoslavia|0|3|E|9,10,11,14
Afghanistan|1|2|R|38,30
Australia|1|4||28
Burma|1|2|S|24,27
India|1|3|B|23,30
Indonesia|1|1|S|28,31
Japan|1|4|BU|31,32,33
Laos/Cambodia|1|1|S|23,34,35
Malaysia|1|2|S|22,25,34
N. Korea|1|3|BR|32
Pakistan|1|2|B|21,24,38
Philippines|1|2|S|25,26
S. Korea|1|3|B|26,29,33
Taiwan|1|3||26,32
Thailand|1|2|SB|27,28,35
Vietnam|1|1|S|27,34
Egypt|2|2|B|40,43,59
Gulf States|2|3||39,44
Iran|2|2|B|21,39,30
Iraq|2|3|B|37,38,41,44
Israel|2|4|B|36,41,42,45
Jordan|2|2||39,40,42,44
Lebanon|2|1||40,41,45
Libya|2|2|B|36,60
Saudi Arabia|2|3|B|37,39,41
Syria|2|2||40,42,17
Algeria|3|2|B|8,53,56,60
Angola|3|1|B|48,58,62
Botswana|3|2||47,58,63
Cameroon|3|1||54,62
Ethiopia|3|1||57,59
Ivory Coast|3|2||54,61
Kenya|3|2||55,57
Morocco|3|3||46,15,61
Nigeria|3|1|B|49,51,56
SE African States|3|1||52,63
Saharan States|3|1||46,54
Somalia|3|2||50,52
South Africa|3|3|B|47,48
Sudan|3|1||36,50
Tunisia|3|2||46,43
West African States|3|2||51,53
Zaire|3|1|B|47,49,63
Zimbabwe|3|1||48,55,62
Costa Rica|4|3||70,72,73
Cuba|4|3|BU|69,72
Dominican Rep|4|1||69
El Salvador|4|1||68,70
Guatemala|4|1||67,70,71
Haiti|4|1||65,66
Honduras|4|2||64,67,68,72
Mexico|4|2|BU|68
Nicaragua|4|1||64,65,70
Panama|4|2|B|78,64
Argentina|5|2|B|77,80,82
Bolivia|5|2||80,81
Brazil|5|2|B|82,83
Chile|5|3|B|74,81
Colombia|5|1||79,73,83
Ecuador|5|2||78,81
Paraguay|5|2||74,75,82
Peru|5|2||75,77,79
Uruguay|5|2||74,76,80
Venezuela|5|2|B|76,78
'''
CARD_TABLE = '''
1|Asia Scoring|0|0|0|S
2|Europe Scoring|0|0|0|S
3|Middle East Scoring|0|0|0|S
4|Duck and Cover|3|0|1|
5|Five Year Plan|3|0|1|
6|The China Card|4|0|0|
7|Socialist Governments|3|0|2|
8|Fidel|2|0|2|*
9|Vietnam Revolts|2|0|2|*
10|Blockade|1|0|2|*
11|Korean War|2|0|2|*
12|Romanian Abdication|1|0|2|*
13|Arab-Israeli War|2|0|2|
14|Comecon|3|0|2|*
15|Nasser|1|0|2|*
16|Warsaw Pact Formed|3|0|2|*U
17|De Gaulle Leads France|3|0|2|*U
18|Captured Nazi Scientist|1|0|0|*
19|Truman Doctrine|1|0|1|*
20|Olympic Games|2|0|0|
21|NATO|4|0|1|*U
22|Independent Reds|2|0|1|*
23|Marshall Plan|4|0|1|*U
24|Indo-Pakistani War|2|0|0|
25|Containment|3|0|1|*
26|CIA Created|1|0|1|*
27|US/Japan Mutual Defense Pact|4|0|1|*U
28|Suez Crisis|3|0|2|*
29|East European Unrest|3|0|1|
30|Decolonization|2|0|2|
31|Red Scare/Purge|4|0|0|
32|UN Intervention|1|0|0|
33|De-Stalinization|3|0|2|*
34|Nuclear Test Ban|4|0|0|
35|Formosan Resolution|2|0|1|*U
36|Brush War|3|1|0|
37|Central America Scoring|0|1|0|S
38|Southeast Asia Scoring|0|1|0|*S
39|Arms Race|3|1|0|
40|Cuban Missile Crisis|3|1|0|*
41|Nuclear Subs|2|1|1|*
42|Quagmire|3|1|2|*U
43|SALT Negotiations|3|1|0|*
44|Bear Trap|3|1|1|*U
45|Summit|1|1|0|
46|How I Learned to Stop Worrying|2|1|0|*
47|Junta|2|1|0|
48|Kitchen Debates|1|1|1|*
49|Missile Envy|2|1|0|
50|“We Will Bury You”|4|1|2|*
51|Brezhnev Doctrine|3|1|2|*
52|Portuguese Empire Crumbles|2|1|2|*
53|South African Unrest|2|1|2|
54|Allende|1|1|2|*
55|Willy Brandt|2|1|2|*U
56|Muslim Revolution|4|1|2|
57|ABM Treaty|4|1|0|
58|Cultural Revolution|3|1|2|*
59|Flower Power|4|1|2|*U
60|U2 Incident|3|1|2|*
61|OPEC|3|1|2|
62|“Lone Gunman”|1|1|2|*
63|Colonial Rear Guards|2|1|1|
64|Panama Canal Returned|1|1|1|*
65|Camp David Accords|2|1|1|*U
66|Puppet Governments|2|1|1|*
67|Grain Sales to Soviets|2|1|1|
68|John Paul II Elected Pope|2|1|1|*U
69|Latin American Death Squads|2|1|0|
70|OAS Founded|1|1|1|*
71|Nixon Plays the China Card|2|1|1|*
72|Sadat Expels Soviets|1|1|1|*
73|Shuttle Diplomacy|3|1|1|
74|The Voice of America|2|1|1|
75|Liberation Theology|2|1|2|
76|Ussuri River Skirmish|3|1|1|*
77|“Ask Not What Your Country…”|3|1|1|*
78|Alliance for Progress|3|1|1|*
79|Africa Scoring|0|1|0|S
80|“One Small Step…”|2|1|0|
81|South America Scoring|0|1|0|S
82|Iranian Hostage Crisis|3|2|2|*U
83|The Iron Lady|3|2|1|*U
84|Reagan Bombs Libya|2|2|1|*
85|Star Wars|2|2|1|*
86|North Sea Oil|3|2|1|*U
87|The Reformer|3|2|2|*U
88|Marine Barracks Bombing|2|2|2|*
89|Soviets Shoot Down KAL-007|4|2|1|*
90|Glasnost|4|2|2|*
91|Ortega Elected in Nicaragua|2|2|2|*
92|Terrorism|2|2|0|
93|Iran-Contra Scandal|2|2|2|*
94|Chernobyl|3|2|1|*
95|Latin American Debt Crisis|2|2|2|
96|Tear Down this Wall|3|2|1|*U
97|“An Evil Empire”|3|2|1|*U
98|Aldrich Ames Remix|3|2|2|*
99|Pershing II Deployed|3|2|2|*
100|Wargames|4|2|0|*
101|Solidarity|2|2|1|*
102|Iran-Iraq War|2|2|0|*
103|Defectors|2|0|1|
104|The Cambridge Five|2|0|2|
105|Special Relationship|2|0|1|
106|NORAD|3|0|1|*U
107|Che|3|1|2|
108|Our Man in Tehran|2|1|1|*
109|Yuri and Samantha|2|2|2|*
110|AWACS Sale to Saudis|3|2|1|*U
'''

# ---------------------------------------------------------------- data
US, USSR = 0, 1
SN = ["US", "USSR"]
def sid(x): return {"US": 0, "USSR": 1}[x.upper()] if isinstance(x, str) else int(x)
def opp(s): return 1 - s
REGIONS = ["Europe", "Asia", "Middle East", "Africa", "Central America", "South America"]
C = []
for line in COUNTRY_TABLE.strip().splitlines():
    n, r, stab, fl, adj = line.split("|")
    C.append(dict(name=n, region=int(r), stab=int(stab), ee="E" in fl, we="W" in fl, sea="S" in fl, bg="B" in fl,
                  adj_us="U" in fl, adj_ussr="R" in fl, adj=[int(a) for a in adj.split(",") if a]))
NC = len(C)
IDX = {c["name"].lower(): i for i, c in enumerate(C)}
CARDS = {}
for line in CARD_TABLE.strip().splitlines():
    i, n, ops, era, side, fl = line.split("|")
    CARDS[int(i)] = dict(name=n, ops=int(ops), era=int(era), side=int(side), star="*" in fl, scoring="S" in fl, underlined="U" in fl)
def owner(k): return {1: US, 2: USSR}.get(CARDS[k]["side"])
REGION_C = [[i for i, c in enumerate(C) if c["region"] == r] for r in range(6)]
SEA = [i for i, c in enumerate(C) if c["sea"]]
def cix(name):
    k = name.strip().lower()
    if k in IDX: return IDX[k]
    hits = [i for n, i in IDX.items() if n.startswith(k)]
    if len(hits) == 1: return hits[0]
    raise SystemExit(f"unknown or ambiguous country: {name}")
THAILAND, TAIWAN, JAPAN, FRANCE, W_GERMANY, CUBA = (cix(x) for x in ["Thailand", "Taiwan", "Japan", "France", "W. Germany", "Cuba"])
EUROPE, ASIA, MIDEAST, AFRICA, CENTAM, SOUTHAM = range(6)
CHINA = 6
SCORING_OF_REGION = [2, 1, 3, 79, 37, 81]
REGION_OF_SCORING = {1: ASIA, 2: EUROPE, 3: MIDEAST, 37: CENTAM, 79: AFRICA, 81: SOUTHAM}
# box: (Ops needed, roll max, VP first, VP second)
SPACE = [(0, 0, 0, 0), (2, 3, 2, 1), (2, 4, 0, 0), (2, 3, 2, 0), (2, 4, 0, 0), (3, 3, 3, 1), (3, 4, 0, 0), (3, 3, 4, 2), (4, 2, 2, 0)]
SPACE_NAMES = ["Start", "Earth Satellite", "Animal in Space", "Man in Space", "Man in Earth Orbit", "Lunar Orbit",
               "Eagle/Bear has Landed", "Space Shuttle", "Space Station"]
# Evaluation weights: identical to the simulator's default bot after Stage 2 run 5 (Weights::core).
BASE = dict(vp=1.0, held=0.9196, region_unseen=0.3182, final_=0.0, final_from=0.0, bg=1.1783, europe_mult=1.0, asia_mult=1.0,
            access=0.0, milops=1.0961, squeeze=0.0, trap=3.0, china=1.7139, bad=0.3, space_ab=0.5, inf=0.05, auto_eu=15.0,
            trapped=3.0, h3=0.0, nc=1.0745)
FLAGS = ["marshall", "warsaw", "nato", "de_gaulle", "willy_brandt", "us_japan", "formosan", "camp_david", "jp2", "flower_power",
         "evil_empire", "iron_lady", "north_sea_oil", "reformer", "tear_down", "awacs", "norad", "iranian_hostage", "shuttle",
         "quagmire", "bear_trap", "containment", "brezhnev", "vietnam_revolts", "nuclear_subs", "salt", "yuri", "iran_contra",
         "nso_turn", "u2"]

# ---------------------------------------------------------------- state
class St:
    def __init__(self, g):
        self.g = g
        self.inf = [[0, 0] for _ in range(NC)]
        for n, v in g.get("inf", {}).items(): self.inf[cix(n)] = [int(v[0]), int(v[1])]
        self.turn = g.get("turn", 1); self.ar = g.get("ar", 0)
        self.me = sid(g.get("me", "US")); self.phasing = sid(g.get("phasing", g.get("me", "US")))
        self.defcon = g.get("defcon", 5); self.vp = g.get("vp", 0)
        self.milops = list(g.get("milops", [0, 0])); self.space = list(g.get("space", [0, 0]))
        self.space_first = [-1] * 9
        for k, s in g.get("space_first", {}).items(): self.space_first[int(k)] = sid(s)
        self.space_attempts = list(g.get("space_attempts", [0, 0]))
        self.china = sid(g.get("china", "USSR")); self.china_up = g.get("china_up", True)
        e = g.get("effects", {})
        self.ev = {f: bool(e.get(f, False)) for f in FLAGS}
        self.red_scare = [SN[s] in e.get("red_scare", []) for s in (0, 1)]   # side whose cards are -1
        self.lads = sid(e["lads"]) if e.get("lads") else None                  # side that played Latin American Death Squads
        self.chernobyl = REGIONS.index(e["chernobyl"]) if e.get("chernobyl") else None
        self.cmc = sid(e["cmc"]) if e.get("cmc") else None                     # side that played Cuban Missile Crisis
        self.hand = list(g.get("hand", [])); self.removed = set(g.get("removed", [])); self.discard = set(g.get("discard", []))
        self.opp_known = list(g.get("opp_known", [])); self.opp_hand_size = g.get("opp_hand_size", None)
        self.rounds_done = list(g.get("rounds_done", [0, 0]))
        self.winner = -1
    def copy(self):
        s = St.__new__(St); s.__dict__.update(self.__dict__)
        s.inf = [x[:] for x in self.inf]; s.milops = self.milops[:]; s.space = self.space[:]; s.space_first = self.space_first[:]
        s.hand = self.hand[:]
        return s
    def ctrl(self, c):
        u, r = self.inf[c]; s = C[c]["stab"]
        return US if (u >= s and u - r >= s) else USSR if (r >= s and r - u >= s) else -1
    def hand_of(self, side): return self.hand if side == self.me else []
    def adj_sp(self, side, c): return C[c]["adj_us"] if side == US else C[c]["adj_ussr"]
    def add_vp(self, side, n):
        if self.winner >= 0: return
        self.vp += n if side == US else -n
        if self.vp >= 20: self.winner = US
        if self.vp <= -20: self.winner = USSR
    def degrade(self):
        self.defcon = max(1, self.defcon - 1)
        if self.defcon <= 1 and self.winner < 0: self.winner = opp(self.phasing)
    def space_ability(self, side, k): return self.space_first[k] == side and self.space[opp(side)] < k
    def has_inf_region(self, side, r): return any(self.inf[c][side] > 0 for c in REGION_C[r])

def card_ops(st, side, k):
    o = CARDS[k]["ops"]
    if CARDS[k]["scoring"]: return 0
    if side == US and st.ev["containment"]: o = min(o + 1, max(4, o))
    if side == USSR and st.ev["brezhnev"]: o = min(o + 1, max(4, o))
    if st.red_scare[side]: o = max(o - 1, 1)
    return o

def rounds_for(st, side):
    base = 6 if st.turn <= 3 else 7
    return base + 1 if (st.space_ability(side, 8) or (side == US and st.ev["nso_turn"])) else base
def rounds_left(st, side): return rounds_for(st, side) - st.rounds_done[side]
def card_live(st, k):
    if k in st.removed: return False
    e = CARDS[k]["era"]
    return e == 0 or (e == 1 and st.turn >= 3) or (e == 2 and st.turn >= 7)

# ---------------------------------------------------------------- scoring
def bg_scoring(st, c): return C[c]["bg"] or (c == TAIWAN and st.ev["formosan"] and st.ctrl(c) == US)
REGION_VALS = [(3, 7, 1000), (3, 7, 9), (3, 5, 7), (1, 4, 6), (1, 3, 5), (2, 5, 6)]
def region_detail(st, r, skip=None):
    n = [0, 0]; b = [0, 0]; nb = [0, 0]; adj = [0, 0]; tb = 0
    for c in REGION_C[r]:
        if c == skip: continue
        isbg = bg_scoring(st, c); tb += isbg
        k = st.ctrl(c)
        if k >= 0:
            n[k] += 1
            if isbg: b[k] += 1
            else: nb[k] += 1
            if st.adj_sp(opp(k), c): adj[k] += 1
    p, d, ctl = REGION_VALS[r]; out = [0, 0]; lvl = ["-", "-"]
    for s in (0, 1):
        o = opp(s)
        if n[s] == 0: continue
        if n[s] > n[o] and b[s] == tb and tb > 0: L, lvl[s] = ctl, "Control"
        elif n[s] > n[o] and b[s] > b[o] and nb[s] >= 1: L, lvl[s] = d, "Domination"
        else: L, lvl[s] = p, "Presence"
        out[s] = L + b[s] + adj[s]
    return out, lvl, n, b, adj
def region_score(st, r, skip=None): return region_detail(st, r, skip)[0]
def sea_score(st):
    out = [0, 0]
    for c in SEA:
        k = st.ctrl(c)
        if k >= 0: out[k] += 2 if c == THAILAND else 1
    return out
def scoring_net(st, k):
    """Net VP (+ = US) the Scoring card would give now (Shuttle Diplomacy applied as the US would choose)."""
    if k == 38: u, s = sea_score(st); return u - s
    r = REGION_OF_SCORING[k]; u, s = region_score(st, r); base = u - s
    if st.ev["shuttle"] and r in (MIDEAST, ASIA):
        best = base
        for c in REGION_C[r]:
            if bg_scoring(st, c) and st.ctrl(c) == USSR:
                u, s = region_score(st, r, c); best = max(best, u - s)
        return best
    return base
def europe_controller(st):
    u, s = region_score(st, EUROPE)
    return US if u >= 1000 else USSR if s >= 1000 else None
def final_projection(st):
    v = 0
    for r in range(6):
        u, s = region_score(st, r); v += max(-30, min(30, u - s))
    return v + (1 if st.china == US else -1)

# ---------------------------------------------------------------- evaluation (port of the simulator's bot::eval)
def danger(k):
    if k in (16, 23, 42, 44, 50, 56): return 3.0
    if k in (7, 14, 17, 28, 30, 33, 55, 58, 59, 61, 62, 82, 87, 90, 95, 98, 99, 107, 4, 5, 21, 25, 26, 27, 29, 63, 65, 67, 71, 73, 74,
             76, 77, 85, 86, 89, 96): return 2.0
    return 1.0
def trap_for(holder, k):
    o = owner(k)
    if o == US: return holder == USSR and k in (4, 5, 26, 67, 85, 89, 96)
    if o == USSR: return holder == US and k in (50, 62, 91)
    return False

def evaluate(w, st, me, parts=None):
    def add(name, x):
        if parts is not None: parts[name] = parts.get(name, 0.0) + x
        return x
    if st.winner >= 0: return 1000.0 if st.winner == me else 0.0 if st.winner == 2 else -1000.0
    o = opp(me); sg = 1.0 if me == US else -1.0; hand = st.hand_of(me)
    v = add("VP", w["vp"] * sg * st.vp)
    sc_in = sum(1 for k in hand if CARDS[k]["scoring"])
    if st.ar > 0 and sc_in > rounds_left(st, me): v += add("Scoring card cannot be played in time", -900.0)
    ctrl = [st.ctrl(c) for c in range(NC)]
    for r in range(6):
        u, s = region_score(st, r)
        if u >= 1000 or s >= 1000:
            mine = (u >= 1000) if me == US else (s >= 1000)
            v += add("Europe Control", w["auto_eu"] if mine else -w["auto_eu"]); continue
        net = sg * (u - s); k = SCORING_OF_REGION[r]
        if k in hand: v += add("held Scoring cards", w["held"] * net)
        elif card_live(st, k): v += add("Region standing (unseen Scoring cards)", w["region_unseen"] * net)
    if 38 not in st.removed:
        u, s = sea_score(st); net = sg * (u - s)
        if 38 in hand: v += add("held Scoring cards", w["held"] * net)
        elif card_live(st, 38): v += add("Region standing (unseen Scoring cards)", w["region_unseen"] * net)
    if w["final_"] != 0 and st.turn >= w["final_from"]:
        v += add("final projection", w["final_"] * sg * final_projection(st) * (0.3 + 0.7 * st.turn / 10.0))
    h3 = 1.0 + w["h3"] if (me == USSR and st.turn <= 3) else 1.0
    def mult(c): return w["europe_mult"] if C[c]["region"] == EUROPE else w["asia_mult"] if C[c]["region"] == ASIA else 1.0
    for c in range(NC):
        if not C[c]["bg"] or ctrl[c] < 0: continue
        v += add("Battlegrounds", (w["bg"] if ctrl[c] == me else -w["bg"]) * mult(c) * h3)
    if w["nc"] != 0:
        for c in range(NC):
            if not C[c]["bg"]: continue
            stb = C[c]["stab"]
            def short(a): x, y = st.inf[c][a], st.inf[c][opp(a)]; return max(stb - x, stb + y - x)
            if ctrl[c] != me and st.inf[c][me] > 0 and short(me) == 1: v += add("near-Control", w["nc"] * w["bg"] * mult(c))
            if ctrl[c] != o and st.inf[c][o] > 0 and short(o) == 1: v += add("near-Control", -w["nc"] * w["bg"] * mult(c))
    if w["access"]:
        for r in range(6):
            if r >= AFRICA and st.turn < 3: continue
            if not st.has_inf_region(me, r): v += add("access", -w["access"])
            if not st.has_inf_region(o, r): v += add("access", 0.5 * w["access"])
    rf = lambda s: st.rounds_done[s] / max(1, rounds_for(st, s))
    v += add("Military Ops", -w["milops"] * max(0, st.defcon - st.milops[me]) * (0.25 + 0.75 * rf(me)))
    v += add("Military Ops", w["milops"] * max(0, st.defcon - st.milops[o]) * (0.25 + 0.75 * rf(o)))
    if w["squeeze"]:
        for r in (EUROPE, ASIA, MIDEAST):
            locked = (r == EUROPE and st.defcon <= 4) or (r == ASIA and st.defcon <= 3) or (r == MIDEAST and st.defcon <= 2)
            if locked:
                u, s = region_score(st, r); x = sg * (u - s); v += add("squeeze", w["squeeze"] * ((x > 0) - (x < 0)))
    tf = {2: 1.0, 3: 0.5, 4: 0.2}.get(st.defcon, 0.1)
    for k in hand:
        if owner(k) == o: v += add("opponent cards held", -w["bad"] * (1.0 + danger(k) / 2.0))
        if trap_for(me, k): v += add("DEFCON trap cards held", -w["trap"] * tf)
    if st.ar > 0:
        traps = sum(1 for k in hand if trap_for(me, k)); others = len(hand) - traps
        forced = min(max(rounds_left(st, me) - others, 0), traps)
        if forced > 0: v += add("DEFCON trap card forced out", -w["trap"] * 3.0 * forced * {2: 1.0, 3: 0.5}.get(st.defcon, 0.15))
    if (me == US and st.ev["quagmire"]) or (me == USSR and st.ev["bear_trap"]):
        v += add("Quagmire/Bear Trap", -w["trapped"] * (1.0 + sc_in))
        elig = sum(1 for k in hand if not CARDS[k]["scoring"] and card_ops(st, me, k) >= 2)
        if st.ar > 0 and sc_in > 0 and sc_in + min(elig, 1) > rounds_left(st, me): v += add("Scoring card trapped", -900.0)
    if (o == US and st.ev["quagmire"]) or (o == USSR and st.ev["bear_trap"]): v += add("Quagmire/Bear Trap", w["trapped"])
    cv = w["china"] * (1.0 if st.china_up else 0.6)
    v += add("China Card", cv if st.china == me else -cv)
    for b in (2, 4, 6, 8):
        if st.space_ability(me, b): v += add("Space Race abilities", w["space_ab"])
        if st.space_ability(o, b): v += add("Space Race abilities", -w["space_ab"])
    if w["inf"]:
        d = sum(st.inf[c][me] - st.inf[c][o] for c in range(NC)); v += add("Influence total", w["inf"] * d)
    return v

# ---------------------------------------------------------------- legality and operations
def defcon_blocks(st, c):
    r = C[c]["region"]
    return (r == EUROPE and st.defcon <= 4) or (r == ASIA and st.defcon <= 3) or (r == MIDEAST and st.defcon <= 2)
def nato_protects(st, side, c):
    return (side == USSR and st.ev["nato"] and C[c]["region"] == EUROPE and st.ctrl(c) == US
            and not (c == FRANCE and st.ev["de_gaulle"]) and not (c == W_GERMANY and st.ev["willy_brandt"]))
def can_coup(st, side, c, free=False):
    if st.inf[c][opp(side)] == 0: return False
    if not free and defcon_blocks(st, c): return False
    if nato_protects(st, side, c): return False
    if side == USSR and c == JAPAN and st.ev["us_japan"]: return False
    if side == USSR and st.ev["reformer"] and C[c]["region"] == EUROPE: return False
    return True
def can_realign(st, side, c, free=False):
    if st.inf[c][opp(side)] == 0: return False
    if not free and defcon_blocks(st, c): return False
    if nato_protects(st, side, c): return False
    if side == USSR and c == JAPAN and st.ev["us_japan"]: return False
    return True
def in_scope(c, scope): return scope == "any" or (scope == "asia" and C[c]["region"] == ASIA) or (scope == "sea" and C[c]["sea"])
def can_place(st, side, c, scope="any", start=None):
    """start: Influence at the start of the Action Round (defaults to the state's)."""
    if not in_scope(c, scope): return False
    if side == USSR and st.chernobyl == C[c]["region"]: return False
    pres = start if start is not None else st.inf
    if pres[c][side] > 0 or st.adj_sp(side, c): return True
    return any(pres[a][side] > 0 for a in C[c]["adj"])
def place_cost(st, side, c): return 2 if st.ctrl(c) == opp(side) else 1
def scope_bonus(st, side, card, scope):
    b = 0
    if card == CHINA and scope in ("asia", "sea"): b += 1
    if side == USSR and st.ev["vietnam_revolts"] and card != 0 and scope == "sea": b += 1
    return b
def coup_result(st, side, c, ops, free, d):
    s = st.copy()
    if st.cmc == opp(side): s.winner = opp(side); return s
    if side == US and st.ev["yuri"]: s.add_vp(USSR, 1)
    m = (-1 if st.ev["salt"] else 0)
    if C[c]["region"] in (CENTAM, SOUTHAM) and st.lads is not None: m += 1 if st.lads == side else -1
    x = d + ops + m - 2 * C[c]["stab"]
    if x > 0:
        o = opp(side); rem = min(x, s.inf[c][o]); s.inf[c][o] -= rem; s.inf[c][side] += x - rem
    if not free: s.milops[side] = min(5, s.milops[side] + ops)
    if C[c]["bg"] and not (side == US and st.ev["nuclear_subs"]): s.degrade()
    return s
def coup_mod(st, side, c):
    m = (-1 if st.ev["salt"] else 0)
    if C[c]["region"] in (CENTAM, SOUTHAM) and st.lads is not None: m += 1 if st.lads == side else -1
    return m
def realign_mod(st, side, c):
    m = sum(1 for a in C[c]["adj"] if st.ctrl(a) == side)
    if st.inf[c][side] > st.inf[c][opp(side)]: m += 1
    if st.adj_sp(side, c): m += 1
    if side == US and st.ev["iran_contra"]: m -= 1
    return m
def realign_outcomes(st, side, c):
    o = opp(side); ma, mb = realign_mod(st, side, c), realign_mod(st, o, c); out = []
    for a in range(1, 7):
        for b in range(1, 7):
            d = (a + ma) - (b + mb); s = st.copy()
            if d > 0: s.inf[c][o] = max(0, s.inf[c][o] - d)
            if d < 0: s.inf[c][side] = max(0, s.inf[c][side] + d)
            out.append(s)
    return out
def space_advance(st, side, n, last_vp_only=False):
    for i in range(n):
        b = st.space[side] + 1
        if b > 8: break
        st.space[side] = b; first = st.space[opp(side)] < b
        if first: st.space_first[b] = side
        if not last_vp_only or i == n - 1:
            v = SPACE[b][2] if first else SPACE[b][3]
            if v > 0: st.add_vp(side, v)
def can_space(st, side, card):
    b = st.space[side] + 1; allowed = 2 if st.space_ability(side, 2) else 1
    return b <= 8 and st.space_attempts[side] < allowed and not CARDS[card]["scoring"] and card_ops(st, side, card) >= SPACE[b][0]

def greedy_place(w, st, me, left, scope):
    s = st.copy(); start = st.inf; placed = []
    while True:
        best = None
        for c in range(NC):
            if not can_place(s, me, c, scope, start): continue
            cost = place_cost(s, me, c)
            if cost > left: continue
            t = s.copy(); t.inf[c][me] += 1; v = evaluate(w, t, me)
            if best is None or v > best[0]: best = (v, c)
        if best is None: break
        c = best[1]; left -= place_cost(s, me, c); s.inf[c][me] += 1; placed.append(C[c]["name"])
        if left <= 0: break
    return s, placed

def best_ops(w, st, me, card, base, coup=True, realign=True, place=True, space=True):
    """Best use of Ops (expected value, description). Same procedure as the simulator's Brain::best_ops."""
    best = (float("-inf"), "none", None)
    if place:
        scopes = ["any"]
        if card == CHINA: scopes.append("asia")
        if card != 0 and ((me == USSR and st.ev["vietnam_revolts"]) or card == CHINA): scopes.append("sea")
        for sc in scopes:
            n = base + scope_bonus(st, me, card, sc); s, placed = greedy_place(w, st, me, n, sc); v = evaluate(w, s, me)
            if v > best[0]:
                from collections import Counter
                best = (v, "place", f"place {n} Ops" + (f" ({sc})" if sc != "any" else "") + ": " + ", ".join(f"{k} +{m}" for k, m in Counter(placed).items()))
    if coup:
        for c in range(NC):
            if not can_coup(st, me, c): continue
            sc = ("sea" if C[c]["sea"] else "asia") if C[c]["region"] == ASIA else "any"
            ops = base + scope_bonus(st, me, card, sc)
            v = sum(evaluate(w, coup_result(st, me, c, ops, False, d), me) for d in range(1, 7)) / 6.0
            if v > best[0]:
                need = 2 * C[c]["stab"] - ops - coup_mod(st, me, c) + 1; p = max(0, min(6, 7 - need)) / 6.0
                best = (v, "coup", f"Coup {C[c]['name']} with {ops} Ops: success {p:.0%}" + (" (Battleground: DEFCON -1)" if C[c]["bg"] and not (me == US and st.ev["nuclear_subs"]) else ""))
    if realign:
        b0 = evaluate(w, st, me); gain, tgt = float("-inf"), None
        for c in range(NC):
            if can_realign(st, me, c):
                g = sum(evaluate(w, s, me) for s in realign_outcomes(st, me, c)) / 36.0 - b0
                if g > gain: gain, tgt = g, c
        if tgt is not None:
            v = b0 + gain * base * 0.8
            if v > best[0]: best = (v, "realign", f"Realignment rolls ({base}), best first target {C[tgt]['name']} (+{gain:.2f} per roll)")
    if space and card != 0 and can_space(st, me, card):
        b = st.space[me] + 1; p = SPACE[b][1] / 6.0; ok = st.copy(); space_advance(ok, me, 1)
        v = p * evaluate(w, ok, me) + (1 - p) * evaluate(w, st, me)
        if v > best[0]: best = (v, "space", f"Space Race to box {b} ({SPACE_NAMES[b]}), roll 1-{SPACE[b][1]}: {p:.0%}")
    if best[0] == float("-inf"): best = (evaluate(w, st, me), "none", "no legal use")
    return best

# ---------------------------------------------------------------- DEFCON hard limits (Twilight Strategy; simulator run 3)
def opp_can_bg_coup(st, me, europe_free=False):
    o = opp(me)
    if st.cmc == me: return False
    if o == US and st.ev["nuclear_subs"]: return False
    return any(C[c]["bg"] and st.inf[c][me] > 0 and (can_coup(st, o, c) or (europe_free and C[c]["region"] == EUROPE and can_coup(st, o, c, True)))
               for c in range(NC))
def out_of_play(st, k): return k in st.removed or k in st.discard or k in st.hand
def forbidden_ar(st, me, k, how):
    """how: 'event' or 'ops'. True = never make this play now (DEFCON 2)."""
    if st.defcon > 2 or k == CHINA: return False
    o = opp(me)
    fires = how == "event" or (how == "ops" and owner(k) == o)
    if not fires: return False
    if k in (4, 50, 89): return True
    if k in (26, 62, 67): return opp_can_bg_coup(st, me)
    if k == 96: return opp_can_bg_coup(st, me, True)
    if k == 91: return me == US and st.inf[CUBA][US] > 0 and st.cmc != US
    if k in (20, 45, 49): return how == "event"
    if k == 5:
        if me == US: return not (out_of_play(st, 4) and out_of_play(st, 89))
        return any(h != 5 and (h in (4, 89) or (h in (26, 67, 96) and opp_can_bg_coup(st, me, h == 96))) for h in st.hand)
    return False
def forbidden_headline(st, me, k):
    if st.defcon < 3 or k == CHINA: return False
    if k not in (4, 50, 89, 26, 62, 67, 96, 20, 45, 49, 5, 91): return False
    o = opp(me); ops = lambda c: 0 if CARDS[c]["scoring"] else CARDS[c]["ops"]; mine = ops(k)
    deg = [46, 40] if st.defcon >= 4 else ([4, 89, 50, 46, 40, 47, 26] if o == US else [50, 4, 89, 46, 40, 47, 62])
    def live(u):
        e = CARDS[u]["era"]
        return not out_of_play(st, u) and (e == 0 or (e == 1 and st.turn >= 4) or (e == 2 and st.turn >= 8))
    return [u for u in deg if live(u) and (ops(u) > mine or (ops(u) == mine and o == US))]
def is_suicide(st, me, k):
    if k == CHINA or CARDS[k]["scoring"]: return False
    s2 = st.copy(); s2.defcon = 2
    own_ok = owner(k) != opp(me) and not forbidden_ar(s2, me, k, "event")
    return forbidden_ar(s2, me, k, "ops") and not own_ok and not can_space(st, me, k)

# ---------------------------------------------------------------- unseen cards
def unseen_pool(st):
    eras = 0 if st.turn <= 3 else 1 if st.turn <= 7 else 2
    return [k for k in CARDS if k != CHINA and CARDS[k]["era"] <= eras and k not in st.removed and k not in st.discard
            and k not in st.hand and k not in st.opp_known]
def p_opp_holds(st):
    pool = unseen_pool(st)
    if st.opp_hand_size is None or not pool: return None
    return max(0, st.opp_hand_size - len(st.opp_known)) / len(pool)

# ---------------------------------------------------------------- commands
import json, sys, random
def load(p): return json.load(open(p))
def save(p, g): json.dump(g, open(p, "w"), indent=1, ensure_ascii=False)
def weights(args):
    w = dict(BASE)
    for a in args:
        if a.startswith("--w="): a = a[4:]
        else: continue
        for kv in a.split(","):
            k, v = kv.split("="); w[k] = float(v)
    return w
def cname(k): return f"#{k} {CARDS[k]['name']}"
def fmt_side_score(u, s): return f"US {u} - USSR {s} -> net {u - s:+d} ({'US' if u > s else 'USSR' if s > u else 'even'})"

def cmd_init(path, side, handicap="2"):
    g = {"turn": 1, "ar": 0, "me": side.upper(), "phasing": "USSR", "defcon": 5, "vp": 0, "milops": [0, 0], "space": [0, 0],
         "space_first": {}, "space_attempts": [0, 0], "china": "USSR", "china_up": True,
         "inf": {"Syria": [0, 1], "Iraq": [0, 1], "N. Korea": [0, 3], "E. Germany": [0, 3], "Finland": [0, 1],
                 "Canada": [2, 0], "Iran": [1, 0], "Israel": [1, 0], "Japan": [1, 0], "Australia": [4, 0], "Philippines": [1, 0],
                 "S. Korea": [1, 0], "Panama": [1, 0], "South Africa": [1, 0], "U.K.": [5, 0]},
         "effects": {}, "hand": [], "opp_hand_size": 8, "opp_known": [], "known_to_opp": [], "removed": [], "discard": [],
         "rounds_done": [0, 0], "handicap_us": int(handicap), "log": []}
    save(path, g)
    print(f"Created {path}. Still to place: USSR 6 anywhere in Eastern Europe (USSR first), then US 7 anywhere in Western Europe,"
          f" then the US handicap of +{handicap} (only where the US has Influence, at most 2 above what Control needs).")
    print("Add Influence with: add GAME COUNTRY SIDE N (N may be negative). Put my dealt cards in \"hand\".")

def cmd_add(path, country, side, n):
    g = load(path); c = C[cix(country)]["name"]; v = g["inf"].get(c, [0, 0]); v[sid(side)] = max(0, v[sid(side)] + int(n))
    g["inf"][c] = v
    if v == [0, 0]: del g["inf"][c]
    save(path, g); st = St(g); k = st.ctrl(cix(c))
    print(f"{c}: US {v[0]} / USSR {v[1]} (Stability {C[cix(c)]['stab']}) -> {'US' if k == 0 else 'USSR' if k == 1 else 'nobody'} Controls")

def region_lines(st, r):
    out, lvl, n, b, adj = region_detail(st, r)
    rows = []
    for c in REGION_C[r]:
        u, s = st.inf[c]; k = st.ctrl(c)
        if u or s or C[c]["bg"]:
            tag = ("*" if C[c]["bg"] else " ") + ("US " if k == 0 else "SU " if k == 1 else "   ")
            rows.append(f"    {tag}{C[c]['name']:<18} {u}/{s} (stab {C[c]['stab']})")
    head = f"  {REGIONS[r]}: {fmt_side_score(*out) if max(out) < 1000 else 'EUROPE CONTROL: automatic win for ' + SN[0 if out[0] >= 1000 else 1]}"
    head += f"   [US {lvl[0]}, {n[0]} countries, {b[0]} BG | USSR {lvl[1]}, {n[1]} countries, {b[1]} BG]"
    return [head] + rows

def cmd_report(path, *args):
    g = load(path); st = St(g); me = st.me; o = opp(me); w = weights(args)
    print(f"Turn {st.turn}, Action Round {st.ar} ({'Headline' if st.ar == 0 else 'phasing ' + SN[st.phasing]}), I am {SN[me]}")
    print(f"VP {st.vp:+d} (+ = US){'  ** within 4 of 20 **' if abs(st.vp) >= 16 else ''}   DEFCON {st.defcon}"
          f" (no Coups/Realignments in: {', '.join(r for r, x in [('Europe', st.defcon <= 4), ('Asia', st.defcon <= 3), ('Middle East', st.defcon <= 2)] if x) or 'none'})")
    print(f"Military Ops: US {st.milops[0]}, USSR {st.milops[1]}; required {st.defcon}. Shortfall now: US {max(0, st.defcon - st.milops[0])}, USSR {max(0, st.defcon - st.milops[1])}")
    print(f"Space Race: US box {st.space[0]} ({SPACE_NAMES[st.space[0]]}), USSR box {st.space[1]} ({SPACE_NAMES[st.space[1]]}); abilities: "
          + (", ".join(f"{SN[s]} box {b}" for s in (0, 1) for b in (2, 4, 6, 8) if st.space_ability(s, b)) or "none"))
    print(f"China Card: {SN[st.china]} ({'face up' if st.china_up else 'face down'})")
    print(f"Rounds: US {st.rounds_done[0]}/{rounds_for(st, 0)}, USSR {st.rounds_done[1]}/{rounds_for(st, 1)}")
    act = [f for f in FLAGS if st.ev[f]] + [f"red_scare on {SN[s]}" for s in (0, 1) if st.red_scare[s]] + \
          ([f"LADS by {SN[st.lads]}"] if st.lads is not None else []) + ([f"Chernobyl: {REGIONS[st.chernobyl]}"] if st.chernobyl is not None else []) + \
          ([f"CUBAN MISSILE CRISIS by {SN[st.cmc]}: {SN[opp(st.cmc)]} loses if it Coups"] if st.cmc is not None else [])
    print("Effects in play:", ", ".join(act) or "none")
    print("\nRegions (scoring if played now; * = Battleground; US/SU = Control):")
    for r in range(6):
        for l in region_lines(st, r): print(l)
    u, s = sea_score(st); print(f"  Southeast Asia (card #38{' removed' if 38 in st.removed else ''}): US {u} - USSR {s} -> net {u - s:+d}")
    fp = final_projection(st); ec = europe_controller(st)
    print(f"\nFinal Scoring projection (incl. China Card): net {fp:+d}" + (f"  ** {SN[ec]} Controls Europe: Europe Scoring or Final Scoring wins **" if ec is not None else ""))
    p = p_opp_holds(st); pool = unseen_pool(st)
    print("\nScoring cards:")
    for k in (2, 1, 3, 79, 37, 81, 38):
        net = scoring_net(st, k); mine = net if me == US else -net
        if k in st.hand: where = "IN MY HAND (must be played this turn)"
        elif k in st.removed: where = "removed"
        elif k in st.discard: where = "discard pile"
        elif k in st.opp_known: where = "opponent holds it (known)"
        elif CARDS[k]["era"] > (0 if st.turn <= 3 else 1 if st.turn <= 7 else 2): where = "not yet in the deck"
        else: where = "unseen" + (f": opponent holds it with p = {p:.0%}" if p is not None else "")
        print(f"  {cname(k):<28} now {mine:+d} for me   {where}")
    print("\nMy hand:")
    for k in sorted(st.hand):
        cd = CARDS[k]; side = ["neutral", "US", "USSR"][cd["side"]]
        flags = []
        if owner(k) == o: flags.append("opponent Event: fires if played for Ops")
        if st.defcon == 2:
            for how in ("event", "ops"):
                if forbidden_ar(st, me, k, how): flags.append(f"FORBIDDEN as {how} at DEFCON 2")
        if is_suicide(st, me, k): flags.append("DEFCON suicide card at DEFCON 2: dispose of it while DEFCON >= 3")
        hl = forbidden_headline(st, me, k) if st.ar == 0 else False
        if hl: flags.append("do not Headline: a degrader may resolve first (" + ", ".join(cname(u) for u in hl) + ")")
        print(f"  {cname(k):<34} {cd['ops']} Ops ({card_ops(st, me, k)} now) {side}{' *' if cd['star'] else ''}" + ("  - " + "; ".join(flags) if flags else ""))
    if st.china == me: print(f"  #6 The China Card (4 Ops, +1 all in Asia){' face down: not playable this turn' if not st.china_up else ''}")
    sui = [k for k in st.hand if is_suicide(st, me, k)]
    if sui and st.defcon >= 3:
        safe = len(st.hand) - len(sui) + (1 if st.china == me and st.china_up else 0)
        if rounds_left(st, me) > safe: print(f"  ** {len(sui)} suicide card(s) and only {safe} safe cards for {rounds_left(st, me)} rounds: play one now while DEFCON >= 3 **")
    sc = sum(1 for k in st.hand if CARDS[k]["scoring"])
    if sc and st.ar > 0 and sc >= rounds_left(st, me): print(f"  ** {sc} Scoring card(s) and {rounds_left(st, me)} rounds left: play them now **")
    print(f"\nUnseen pool: {len(pool)} cards; opponent hand {st.opp_hand_size}")
    print(f"Evaluation (core bot weights): me {evaluate(w, st, me):+.2f}")

def cmd_score(path, region="all"):
    st = St(load(path))
    rs = range(6) if region == "all" else [next(i for i, r in enumerate(REGIONS) if r.lower().startswith(region.lower()))]
    for r in rs:
        for l in region_lines(st, r): print(l)
    if region == "all":
        u, s = sea_score(st); print(f"  Southeast Asia: US {u} - USSR {s} -> net {u - s:+d}"); print(f"Final projection net {final_projection(st):+d}")

def cmd_coup(path, side, country, ops, *args):
    st = St(load(path)); s = sid(side); c = cix(country); ops = int(ops); free = "--free" in args
    print(f"{SN[s]} Coup in {C[c]['name']} (Stability {C[c]['stab']}, Influence US {st.inf[c][0]} / USSR {st.inf[c][1]}) with {ops} Ops")
    if not can_coup(st, s, c, free): print("  NOT ALLOWED (no enemy Influence, DEFCON region limit, NATO, US/Japan Pact or The Reformer)"); return
    m = coup_mod(st, s, c)
    for d in range(1, 7):
        x = d + ops + m - 2 * C[c]["stab"]; r = coup_result(st, s, c, ops, free, d)
        print(f"  roll {d}: {'success ' + str(x) if x > 0 else 'fails'} -> US {r.inf[c][0]} / USSR {r.inf[c][1]}" + (" (WIN for " + SN[r.winner] + ")" if r.winner >= 0 else ""))
    p = sum(1 for d in range(1, 7) if d + ops + m - 2 * C[c]["stab"] > 0) / 6
    print(f"  success {p:.0%}; die modifier {m:+d}; Military Ops +{0 if free else ops};"
          + (" DEFCON -1 (Battleground)" if C[c]["bg"] and not (s == US and st.ev["nuclear_subs"]) else " no DEFCON change")
          + (f"  ** DEFCON {st.defcon} -> {st.defcon - 1}: {'YOU LOSE' if st.defcon == 2 else 'check'} **" if C[c]["bg"] and st.defcon <= 3 and not (s == US and st.ev['nuclear_subs']) else ""))
    if st.cmc == opp(s): print("  ** Cuban Missile Crisis: this Coup loses the game **")

def cmd_realign(path, side, country, *args):
    st = St(load(path)); s = sid(side); c = cix(country)
    if not can_realign(st, s, c, "--free" in args): print("NOT ALLOWED"); return
    ma, mb = realign_mod(st, s, c), realign_mod(st, opp(s), c); dist = {}
    for a in range(1, 7):
        for b in range(1, 7):
            d = (a + ma) - (b + mb); dist[d] = dist.get(d, 0) + 1
    win = sum(v for d, v in dist.items() if d > 0) / 36; lose = sum(v for d, v in dist.items() if d < 0) / 36
    exp_rem = sum(min(d, st.inf[c][opp(s)]) * v for d, v in dist.items() if d > 0) / 36
    exp_lost = sum(min(-d, st.inf[c][s]) * v for d, v in dist.items() if d < 0) / 36
    print(f"{SN[s]} Realignment in {C[c]['name']}: modifiers {SN[s]} {ma:+d}, {SN[opp(s)]} {mb:+d}")
    print(f"  I remove Influence {win:.0%} of the time (expected {exp_rem:.2f}); I lose Influence {lose:.0%} (expected {exp_lost:.2f}); tie {1 - win - lose:.0%}")

def cmd_place(path, side, spec, *args):
    """spec: 'Italy,Italy,France' (one entry per marker). --apply writes it."""
    g = load(path); st = St(g); s = sid(side); start = [x[:] for x in st.inf]; cost = 0; ok = True
    for name in [x for x in spec.split(",") if x.strip()]:
        c = cix(name)
        if not can_place(st, s, c, "any", start): print(f"  {C[c]['name']}: NOT ALLOWED (no own Influence there or adjacent at the start of the round{', Chernobyl' if st.chernobyl == C[c]['region'] and s == USSR else ''})"); ok = False; continue
        k = place_cost(st, s, c); cost += k; st.inf[c][s] += 1
        print(f"  {C[c]['name']}: +1 for {k} Ops -> US {st.inf[c][0]} / USSR {st.inf[c][1]}")
    print(f"Total cost {cost} Ops" + ("" if ok else " (some markers not allowed)"))
    if "--apply" in args and ok:
        g["inf"] = {C[c]["name"]: st.inf[c] for c in range(NC) if st.inf[c] != [0, 0]}; save(path, g); print("applied")

def cmd_eval(path, *args):
    g = load(path); st = St(g); side = sid(args[0]) if args and not args[0].startswith("--") else st.me; w = weights(args)
    parts = {}; v = evaluate(w, st, side, parts)
    print(f"Evaluation for {SN[side]}: {v:+.2f}")
    for k, x in sorted(parts.items(), key=lambda t: -abs(t[1])):
        if abs(x) > 1e-9: print(f"  {k:<42} {x:+.2f}")

def apply_changes(st, ch):
    for n, d in ch.get("inf", {}).items():
        c = cix(n); st.inf[c] = [max(0, st.inf[c][0] + d[0]), max(0, st.inf[c][1] + d[1])]
    if "vp" in ch: st.vp += ch["vp"]; st.winner = US if st.vp >= 20 else USSR if st.vp <= -20 else st.winner
    if "defcon" in ch: st.defcon = max(1, min(5, st.defcon + ch["defcon"])); st.winner = opp(st.phasing) if st.defcon <= 1 else st.winner
    for i, d in enumerate(ch.get("milops", [0, 0])): st.milops[i] = min(5, max(0, st.milops[i] + d))
    for i, d in enumerate(ch.get("space", [0, 0])):
        if d: space_advance(st, i, d)
    for k in ch.get("play", []):
        if k in st.hand: st.hand.remove(k)
        if CARDS[k]["star"]: st.removed.add(k)
    for k in ch.get("removed", []): st.removed.add(k)
    for f, v in ch.get("effects", {}).items():
        if f in st.ev: st.ev[f] = bool(v)
    if "china" in ch: st.china = sid(ch["china"]); st.china_up = ch.get("china_up", False)
    return st

def cmd_whatif(path, side, changes, *args):
    g = load(path); st = St(g); s = sid(side); w = weights(args); ch = json.loads(changes)
    b = evaluate(w, st, s); st2 = apply_changes(st.copy(), ch); a = evaluate(w, st2, s)
    print(f"{SN[s]} evaluation: {b:+.2f} -> {a:+.2f}  (change {a - b:+.2f})")
    for r in range(6):
        x, y = region_score(st, r), region_score(st2, r)
        if x != y: print(f"  {REGIONS[r]}: {fmt_side_score(*x)}  ->  {fmt_side_score(*y)}")
    if final_projection(st) != final_projection(st2): print(f"  final projection {final_projection(st):+d} -> {final_projection(st2):+d}")

def cmd_decide(path, *args):
    g = load(path); st = St(g); me = st.me; o = opp(me); w = weights(args); v0 = evaluate(w, st, me); rows = []
    cards = list(st.hand) + ([CHINA] if st.china == me and st.china_up else [])
    for k in cards:
        cd = CARDS[k]
        if cd["scoring"]:
            net = scoring_net(st, k); mine = net if me == US else -net
            t = st.copy(); t.hand.remove(k); t.vp += net
            if k == 2 and europe_controller(st) is not None: rows.append((1e6 if europe_controller(st) == me else -1e6, k, "Europe Scoring: automatic win" if europe_controller(st) == me else "Europe Scoring: LOSES (opponent Controls Europe)")); continue
            rows.append((evaluate(w, t, me) - v0, k, f"score now: {mine:+d} VP for me"))
            continue
        base = card_ops(st, me, k) if k != CHINA else 4 + (1 if me == US and st.ev["containment"] else 0) + (1 if me == USSR and st.ev["brezhnev"] else 0) - (1 if st.red_scare[me] else 0)
        t = st.copy()
        if k in t.hand: t.hand.remove(k)
        v, kind, desc = best_ops(w, t, me, k, base)
        notes = []
        if owner(k) == o: notes.append("opponent Event fires: subtract its value (whatif)")
        if owner(k) != o and not cd["scoring"] and k != CHINA: notes.append("Event not valued here: compare with whatif")
        if st.defcon == 2 and forbidden_ar(st, me, k, "ops"): notes.append("FORBIDDEN for Ops at DEFCON 2")
        if st.defcon == 2 and owner(k) != o and forbidden_ar(st, me, k, "event"): notes.append("Event FORBIDDEN at DEFCON 2")
        if "DEFCON -1" in desc and st.defcon == 2: notes.append("COUP LOSES THE GAME")
        rows.append((v - v0, k, desc + ("  [" + "; ".join(notes) + "]" if notes else "")))
    print(f"{SN[me]} options (Ops only, one move, no opponent reply; value change vs now {v0:+.2f}):")
    for d, k, desc in sorted(rows, key=lambda t: -t[0]): print(f"  {d:+7.2f}  {cname(k):<34} {desc}")

def cmd_limits(path):
    st = St(load(path)); me = st.me
    print(f"DEFCON {st.defcon}. Hard limits for {SN[me]}:")
    for k in sorted(st.hand):
        out = []
        for how in ("event", "ops"):
            if forbidden_ar(st, me, k, how): out.append(f"no {how} now")
        if is_suicide(st, me, k): out.append("suicide card at DEFCON 2")
        if st.ar == 0:
            hl = forbidden_headline(st, me, k)
            if hl: out.append("no Headline (" + ", ".join(cname(u) for u in hl) + " may resolve first)")
        if out: print(f"  {cname(k)}: " + "; ".join(out))

def cmd_roll(n="1"):
    r = [random.randint(1, 6) for _ in range(int(n))]; print("roll:", " ".join(map(str, r)))

def cmd_xcheck(path):
    """Compare with the simulator: lines from `tssim xcheck N seed`."""
    bad = 0; n = 0
    for line in open(path):
        d = json.loads(line); st = St(d["state"]); me = st.me; n += 1; errs = []
        if [st.ctrl(c) for c in range(NC)] != d["ctrl"]: errs.append("ctrl")
        if [region_score(st, r) for r in range(6)] != d["regions"]: errs.append("regions")
        if sea_score(st) != d["sea"]: errs.append("sea")
        for k, v in d["nets"].items():
            if scoring_net(st, int(k)) != v: errs.append(f"net{k}")
        if final_projection(st) != d["final"]: errs.append("final")
        if [rounds_for(st, 0), rounds_for(st, 1)] != d["rounds_for"]: errs.append("rounds_for")
        for s in (0, 1):
            if abs(evaluate(BASE, st, s) - d["eval"][s]) > 1e-4: errs.append(f"eval{s}:{evaluate(BASE, st, s):.4f}/{d['eval'][s]:.4f}")
        for b in d["best_ops"]:
            t = st.copy(); v, kind, desc = best_ops(BASE, t, me, b["card"], b["ops"])
            ch = b["choice"].split(":")[0]
            if abs(v - b["value"]) > 1e-4 or kind != ch: errs.append(f"ops#{b['card']}:{kind} {v:.4f}/{ch} {b['value']:.4f}")
        if errs: bad += 1; print(d["seed"], errs[:6])
    print(f"{n} positions, {bad} with differences")

if __name__ == "__main__":
    a = sys.argv[1:]
    cmds = {"init": cmd_init, "add": cmd_add, "report": cmd_report, "score": cmd_score, "coup": cmd_coup, "realign": cmd_realign,
            "place": cmd_place, "eval": cmd_eval, "whatif": cmd_whatif, "decide": cmd_decide, "limits": cmd_limits, "roll": cmd_roll,
            "xcheck": cmd_xcheck}
    if not a or a[0] not in cmds: print(__doc__); sys.exit(0)
    cmds[a[0]](*a[1:])
```
