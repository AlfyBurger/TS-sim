TITLE = "Twilight Struggle simulator — Stage 2, test run 1"
SUBTITLE = ("For Alfy · 16 November 2026 · repository AlfyBurger/TS-sim · rules: 2026 Hall of Fame edition, base game (110 cards), "
            "fixed +2 US handicap · engine in Rust · all results reproducible from the seeds shown")
FOOTER = "TS-sim · Stage 2 run 1 test report"

VERDICTS = {
    "A_region_unseen_off": "Neutral",
    "A_final_off": "<b>Harmful term</b>: bot better without it",
    "A_bg_off": "Neutral",
    "A_access_off": "Neutral",
    "A_milops_off": "<b>Needed</b> (−8 points without it)",
    "A_trap_off": "<b>Needed</b> (−3.4, just past the margin)",
    "A_trapped_off": "Neutral (leans needed)",
    "A_liabilities_off": "Neutral",
    "A_europe_auto_off": "Inert: never changed a move",
    "A_influence_off": "Neutral",
    "A_space_abilities_off": "Almost inert",
    "A_held_scoring_off": "Neutral",
    "H1_region_0.5": "Neutral (leans worse)",
    "H2_eu_asia_1.5": "Neutral",
    "H3_ussr_early_0.5": "Neutral",
    "H4_squeeze_1": "Neutral",
    "H6_space_dispose": "Neutral",
    "H7_china_hold_1.5": "Neutral (leans better)",
    "C_final_off_fresh": "<b>Confirmed</b> on fresh seeds",
    "C_final_0.3_vs_0": "No better than off: remove it",
}

SECTIONS = [
    ('h', "1. Summary"),
    ('p', "The first test run is complete. The engine plays all 110 cards under the 2026 rules and passed every validation gate. "
          "Twelve parts of the bot's evaluation were switched off one at a time, and six of your hypotheses were tested, each over "
          "800 games (400 deals, each played once from each side)."),
    ('p', "<b>Main findings.</b> (1) One term was hurting the bot: the projection of what Final Scoring would give if the game ended now. "
          "Without it the bot scores 56–57% against its old self, confirmed on a fresh set of deals, so it has been removed. "
          "(2) Two terms clearly earn their place: meeting the Military Ops requirement, and avoiding DEFCON traps (cards whose Event would "
          "lower DEFCON at a fatal moment). (3) None of the six hypotheses changed results beyond the margin. That does not prove them wrong: "
          "most of what they describe is already in the core evaluation (regional scoring values, Battlegrounds), so an extra push on top adds little "
          "at this bot's strength. (4) With the projection removed, self-play is balanced (US 50, USSR 49, 1 draw in 100), "
          "close to what the +2 handicap is meant to produce; before, the US won 63%."),
    ('p', "<b>Main weaknesses still open:</b> 13% of games end at DEFCON 1, almost all from Events lowering DEFCON rather than the bot's own coups; "
          "a third of Scoring cards are still played at a loss; and the bot does not yet look at the opponent's reply before moving."),

    ('h', "2. How to read the numbers"),
    ('p', "<b>Score</b> is the tested bot's share of points against the reference bot (win 1, draw ½), over 400 deals played once from each side "
          "(800 games). 50 means no difference. <b>± margin</b> is the 95% range for that score, calculated from the paired results on each deal. "
          "A result is only called a real effect when it lies outside 50 ± margin; anything inside is <b>neutral</b>. "
          "For an ablation (a term switched off), a score <b>below</b> 50 means the term was helping; <b>above</b> 50 means it was hurting. "
          "The US wins more often than the USSR in the old bot's self-play, so compare the 'as US' and 'as USSR' columns with the reference rows "
          "(old bot against itself: as US 58, as USSR 42)."),

    ('h', "3. Validation (passed before any strategy test)"),
    ('table', [
        ["Check", "Purpose", "Result"],
        ["10 rule tests", "Card and rule edge cases (Containment/Red Scare stacking, event Ops, Headline, extra Action Rounds, scoring)", "All pass"],
        ["3,000 random games", "Invariants: every card accounted for, DEFCON 1–5, Influence never negative, game always ends", "No violations"],
        ["Bot vs random play", "The evaluation must beat random moves by a wide margin (target ≥ 95%)", "Old bot 99% both sides; new bot 100% (400 games)"],
        ["All weights at zero vs random", "A bot with no evaluation should play like random", "45.5 ± 4.7: as expected"],
        ["Determinism", "Same seeds give the same games", "Self-play 50.0 ± 0.0"],
    ], (0.22, 0.53, 0.25)),
    ('p', "Bugs found and fixed while building: Headline cards lost when the game ended mid-Headline; extra Action Rounds (North Sea Oil, "
          "Space Station) never used; the bot ignored the board early in the game; DEFCON traps from CIA Created, Lone Gunman and Grain Sales "
          "when they were the last card in hand; self-traps with Quagmire/Bear Trap while holding a Scoring card; Missile Envy forcing a Scoring card "
          "to be held; Summit and dice gambles that could lose on the spot (now filtered out: if any sampled outcome loses the game, the move counts as a loss)."),

    ('h', "4. Diagnostics: how the bot plays (100 self-play games)"),
    ('table', [
        ["Measure", "Old bot (with projection)", "New bot (without)"],
        ["Wins US / USSR / draw", "63 / 36 / 1", "50 / 49 / 1"],
        ["Game ends: Final Scoring / 20 VP / DEFCON 1 / Wargames / held Scoring", "36 / 47 / 9 / 7 / 1", "38 / 37 / 13 / 9 / 3"],
        ["Average last turn", "7.9", "7.8"],
        ["Scoring cards played per game; share played at a loss", "9.3; 36%", "9.5; 33%"],
        ["Average net VP for the player of a Scoring card", "+0.35", "+0.85"],
        ["Opponent Events triggered by playing their cards for Ops, per game", "31.4", "28.1"],
        ["Space Race attempts per game; share using an opponent card", "1.3; 98%", "3.1; 95%"],
        ["China Card plays per game", "7.6", "7.8"],
        ["Coups per game US / USSR", "9.8 / 11.4", "10.6 / 12.4"],
        ["Military Ops penalty VP per game US / USSR", "5.3 / 4.8", "2.3 / 1.7"],
        ["DEFCON 1 losses not caused by the loser's own Battleground coup", "8 of 9", "13 of 13"],
    ], (0.5, 0.25, 0.25)),
    ('p', "Points to note: the Space Race is used almost only to get rid of opponent cards (your H6) even without a special rule; "
          "the China Card is passed back and forth almost every turn, so 'holding' it is not yet a real choice for this bot; "
          "DEFCON losses come from Events (the bot's own or the opponent's) lowering DEFCON, so the next fix is checking DEFCON-lowering "
          "Events, not coups."),

    ('h', "5. Ablation: each part of the evaluation switched off"),
    ('p', "Reference: the old bot with every term on. Deals 30000–30399."),
    ('results', ['ablation.jsonl'], None),
    ('p', "<b>Final Scoring projection</b> (what the board would give if the game ended now, weighted more heavily as the game goes on): "
          "removing it helped by 6 points. Likely reason: it double-counts what the regional standing term already values, and pushes the bot to "
          "chase Final Scoring positions in Regions whose Scoring cards will come round again first. "
          "<b>Europe automatic-win term</b>: identical games with and without it, because no game came close to Europe Control. "
          "<b>Space abilities</b> almost inert for the same reason: the bot rarely reaches the boxes that grant them. "
          "<b>Caveat:</b> these ablations used the old bot as reference. Terms that look neutral here may matter now that the projection is gone; "
          "the next run repeats the ablation on the new bot."),
    ('h', "6. Your hypotheses (H1–H8)"),
    ('p', "Each hypothesis was turned into a change to the old bot and played against it. Deals 30000–30399."),
    ('results', ['hypotheses.jsonl'], None),
    ('table', [
        ["#", "Hypothesis (yours)", "Tested as", "Verdict"],
        ["H1", "Regional scoring over time decides most games", "Double weight on Region standing (0.25 → 0.5)", "Neutral. Diagnostics agree with the idea: 37–47% of games end at 20 VP, built by regional scoring; Final Scoring decides most of the rest"],
        ["H2", "Europe and Asia matter most", "Battleground weight ×1.5 in Europe and Asia", "Neutral: the scoring values already make them the biggest Regions"],
        ["H3", "Early War: USSR attacks, US survives", "USSR Battleground weight ×1.5 in turns 1–3 (rough version)", "Neutral; needs a real side/era rule (Stage 3)"],
        ["H4", "DEFCON is a weapon or a hazard", "Value DEFCON-locked Regions for whoever leads them", "Neutral; trap avoidance itself is confirmed (ablation)"],
        ["H5", "Losses come from bad Scoring timing, opponent Events, being shut out", "Diagnostics + access ablation", "Partly: Scoring at a loss and opponent Events are frequent (section 4); access term neutral"],
        ["H6", "Space Race is for disposing of bad cards", "Allow the Space Race only with opponent cards", "Neutral: the bot already does this 95–98% of the time"],
        ["H7", "Hold the China Card", "Holding value 1.5 VP", "Neutral (leans better, 52.8 ± 3.2)"],
        ["H8", "Test both sides", "All tests run both sides on the same deals", "Applied"],
        ["H9, H10", "Preparation pays; play order depends on the hand", "Not built yet", "Next run"],
        ["H11, H12", "Reading the opponent; Europe stalemate", "Stage 3", "Later"],
    ], (0.07, 0.27, 0.28, 0.38)),

    ('h', "7. Confirmation and adoption"),
    ('p', "The Final Scoring result was re-tested on new deals (50000–50399), and a half-weight version was tried before adopting."),
    ('results', ['confirm.jsonl'], None),
    ('p', "<b>Change made:</b> the projection weight is now 0 in the default bot. The new bot beats random play in 400 of 400 games and passes "
          "the 10 rule tests. Every other weight is unchanged."),
    ('h', "8. Limitations and next steps"),
    ('p', "<b>Limitations.</b> The bot evaluates its own move only (no look-ahead at the opponent's reply), which allows back-and-forth coups; "
          "it samples the opponent's hidden hand 6 times per decision; H9 (preparation) and H10 (play order) are not built; there is no helper "
          "script yet to cross-check the engine by hand; Stage 2 design steps 2.3–2.10 were skipped so the test could run now."),
    ('p', "<b>Next run, in order:</b> (1) check DEFCON-lowering Events before playing or triggering them, aimed at the 13% DEFCON losses; "
          "(2) a one-reply look-ahead as a blunder check; (3) Scoring timing: score now or build first, aimed at the 33% of Scoring cards "
          "played at a loss; (4) repeat the ablation on the new bot; (5) H9 preparation values and H10 sequencing; "
          "(6) re-test H7 (China Card) with more deals, since it leaned positive."),
]
