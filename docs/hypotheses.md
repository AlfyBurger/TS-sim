# Twilight Struggle — hypotheses to test (Stage 2.2)

Alfy's answers to the "what wins" quiz, 16 Nov 2026. Alfy knows the fundamentals well but not which individual cards win games; he gave no confidence levels, so each is treated as medium until tested. Every factor and decision rule below goes into the first overnight run's configuration and gets a verdict: confirmed, neutral or harmful.

| # | Hypothesis (Alfy) | Becomes | How it is tested |
|---|---|---|---|
| H1 | Regional scoring over time decides most games (not automatic wins or Final Scoring alone). | **Factor** + scoring timing (2.6). Unlike Red Sea, Scoring cards (except SE Asia) are reshuffled and come back, so standing in a Region matters all game. | Ablation: value current Region standing (expected score per Region, weighted by how likely its card is to be played soon) vs. no such term. Diagnostics: how games end (scoring, 20 VP, Europe Control, DEFCON, Final Scoring). |
| H2 | Europe and Asia matter most: Europe is a brutal, costly fight that must pay off; Asia is the highest regular scorer. | **Factor weights** per Region (Battlegrounds, Region standing) | Raise Europe/Asia weights vs. equal weights by scoring value. |
| H3 | Early War: the USSR attacks, the US survives until the Mid War. | **Decision rule** (side/era-dependent aggression: USSR Coup preference turns 1–3; US defensive Influence) and a **Stage 3 plan** | Era/side weight profile vs. one profile for all. |
| H4 | DEFCON is a weapon or a hazard depending on side and situation. | **Factor**: value of DEFCON restrictions by who leads each restricted Region (like Red Sea's "squeeze"), plus hard filters and DEFCON-trap checks | Ablation of the squeeze term; trap diagnostics (games lost to DEFCON, by cause). |
| H5 | Games are lost by bad Scoring timing, by triggering the opponent's strong Events at bad moments, and by being shut out of a Region. | Core: scoring timing (2.6), liabilities and exits (2.8), access factor | Diagnostics: Scoring cards played at a loss; opponent Events triggered and their cost; shut-out turns. Ablation of the access factor. |
| H6 | The Space Race is mainly for disposing of bad cards. | **Decision rule**: reserve the Space Race for the worst opponent card; VP and abilities secondary | Rule vs. racing for VP/abilities when no bad card is held. |
| H7 | Hold The China Card as a safety valve (extra Action Round, room for bad cards, +1 VP at the end); play it only for a big swing in Asia. | **Factor**: holding value of the China Card; **decision rule** for its play | Holding value on vs. off; diagnostics: when it is played, and what it gained. |
| H8 | No side preference: test both sides equally. | Testing rule | All tournaments run both sides on the same seeds. |
| H9 | Scoring cards, Coups/Realignments and War Events pay more when prepared. | **Preparation / set-up values** (2.7) | Set-up values on vs. off; count how often each is played with its condition met. |
| H10 | The order of play within a turn depends on the hand (no fixed rule). | Red Sea's tested sequencing rule (trust half a kept card's value) is retested here | Sequencing on vs. greedy. |
| H11 | Signs the opponent holds a Scoring card: heavy spending in a Region, breaking my Battlegrounds there. | **Stage 3** reading signals | Calibration of the read before any win-rate test. |
| H12 | Europe: keep investing or accept a stalemate depends on side and era. | **Stage 3** plans (era/side profiles) | Plan tests after Stage 2. |

## Verdicts after Stage 2 run 1 (16 Nov 2026)

Full results: `reports/stage2_run1_report.pdf`, raw data in `results/run1/`. 800 games per test (400 deals × both sides); margins are 95%.

| # | Tested as | Score vs old bot | Verdict |
|---|---|---|---|
| H1 | Region standing weight 0.25 → 0.5 | 47.8 ± 3.2 | Neutral (leans worse) |
| H2 | Europe/Asia Battleground weight ×1.5 | 49.1 ± 2.9 | Neutral |
| H3 | USSR Battleground weight ×1.5, turns 1–3 (rough) | 52.2 ± 3.0 | Neutral; real side/era rule in Stage 3 |
| H4 | DEFCON squeeze term on | 51.2 ± 3.2 | Neutral; trap avoidance confirmed by ablation (46.6 ± 2.8 without it) |
| H5 | Diagnostics + access ablation | — | Partly: Scoring at a loss 33–36%, ~30 opponent Events/game; access term neutral |
| H6 | Space Race only with opponent cards | 51.9 ± 2.9 | Neutral: bot already does it 95–98% of the time |
| H7 | China Card holding value 1.5 | 52.8 ± 3.2 | Neutral (leans better); re-test with more deals |
| H8 | Both sides on same deals | — | Applied |
| H9, H10 | — | — | Not built yet (next run) |

Change adopted: the Final Scoring projection term (`final`) is now 0 — removing it scored 55.9 ± 3.1 and 56.8 ± 3.1 on fresh deals; half weight was no better than off (49.1 ± 2.9).
