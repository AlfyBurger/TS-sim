TITLE = "Twilight Struggle simulator — Stage 2, test run 4"
SUBTITLE = ("For Alfy · 17 November 2026 · repository AlfyBurger/TS-sim · 2026 Hall of Fame rules, base game, fixed +2 US handicap · "
            "features from the Red Sea v6 run · first run split between the cloud and Alfy's Mac")
FOOTER = "TS-sim · Stage 2 run 4 test report"
REF_NAME = "run 3 bot"

VERDICTS = {
    "NC_0.5": "<b>Large gain</b>",
    "NC_1.0": "<b>Large gain</b>",
    "L2_0.5": "<b>Gain</b>",
    "L2_1.0": "<b>Gain</b>, smaller than 0.5",
    "NC_1.0_fresh": "<b>Confirmed</b> on fresh deals",
    "NC_1.0_vs_0.5": "1.0 better",
    "NC_0.75_vs_1.0": "1.0 better",
    "NC_1.25_vs_1.0": "1.0 better",
    "NC_2.0_vs_1.0": "Collapse: near-Control worth more than Control",
    "L2_on_NC": "<b>Look-ahead still adds</b> on top of near-Control",
    "FINAL_vs_run3": "<b>New bot vs run 3 bot</b>, new deals",
    "A4_milops_off": "<b>Needed</b>",
    "A4_trap_off": "Neutral (run 2: needed)",
    "A4_access_off": "<b>Harmful</b>: bot better without it",
    "A4_region_unseen_off": "<b>Essential</b>",
    "A4_bg_off": "<b>Essential</b> (also switches near-Control off)",
    "A4_trapped_off": "Neutral",
    "A4_bad_off": "Neutral",
    "A4_held_off": "<b>Essential</b>",
    "A4_inf_off": "Neutral (run 2: needed)",
    "A4_space_ab_off": "Neutral",
    "ACCESS_off_fresh": "<b>Confirmed</b> on fresh deals: access term removed",
}

SECTIONS = [
    ('h', "1. Summary"),
    ('p', "Run 4 brought over the two findings of the Red Sea v6 run: credit for a Battleground <b>one Influence short of Control</b> "
          "(from the review of the Egypt decision in Alfy's Red Sea game) and the <b>selective two-move look-ahead</b>. It also added the "
          "avoidable-damage diagnostic, the move-change pre-check required by the updated learning skill, and a block queue that splits every "
          "test between the cloud and Alfy's Mac."),
    ('p', "<b>Main findings.</b> (1) <b>Near-Control credit is the largest gain so far</b>: 90.3 ± 1.3 against the run 3 bot, 92.1 ± 1.2 on fresh "
          "deals. The run 3 evaluation counted a Battleground only once controlled (plus 0.05 per Influence point), so it could not see a Battleground "
          "one placement away from flipping, which is a central fact of Twilight Struggle. The best weight is 1.0 (a near-Controlled Battleground "
          "counts as much as a controlled one on that term; Control still wins on Region scoring); 2.0 collapses, because the bot then prefers "
          "being one short to completing Control. (2) <b>The two-move look-ahead</b> gains 64.7 ± 1.9 on its own and 63.2 ± 1.8 on top of "
          "near-Control; 0.5 beats 1.0, as in Red Sea. (3) <b>The new bot (near-Control 1.0 + look-ahead 0.5) scores 94.8 ± 0.9 against the "
          "run 3 bot</b> on new deals. (4) The repeated ablation shows the evaluation now rests on four terms: Region standing, Battlegrounds "
          "(with near-Control), the held Scoring card and Military Ops. One term now hurts: the penalty for having no Influence in a Region (<i>access</i>); without it the bot scores 55.1 ± 1.8 on fresh deals, so it is removed."),

    ('h', "2. Set-up: cloud and Mac"),
    ('p', "Tests are cut into blocks of 50 deals (100 games). The cloud (2 cores) takes blocks from the front of a shared queue, the Mac "
          "(Linux VM, 4 ARM cores, Rust installed there today) from the back, in calls of under 3 minutes; results are merged per test. "
          "The Mac plays exactly the same games as the cloud (identical result files on the same deals, checked before and after the new features) "
          "and is about 3× faster. A test of 1,000 deals took 15–40 minutes instead of 1–3 hours. The look-ahead makes the bot about 6× slower per move."),
    ('table', [
        ["Check", "Result"],
        ["Rule tests (11)", "All pass on both machines"],
        ["3,000 random games, rule-state checks", "No violations"],
        ["Bot vs random play (100 deals)", "100%"],
        ["All-zero bot vs random", "51.3 ± 7.1: plays like random"],
        ["Move-change pre-check, bot vs itself", "0 moves changed (the tool works)"],
        ["Pre-check, near-Control 1.0", "43% of moves change: a full test is warranted"],
        ["Same deals, cloud vs Mac", "Identical results"],
    ], (0.55, 0.45)),

    ('h', "3. Feature tests (1,000 deals × both sides)"),
    ('results', ['results.jsonl'], ['NC_0.5', 'NC_1.0', 'L2_0.5', 'L2_1.0', 'NC_1.0_fresh']),
    ('h', "4. Tuning and combination"),
    ('results', ['results.jsonl'], ['NC_1.0_vs_0.5', 'NC_0.75_vs_1.0', 'NC_1.25_vs_1.0', 'NC_2.0_vs_1.0', 'L2_on_NC', 'FINAL_vs_run3']),
    ('p', "Near-Control weight: 1.0 beats 0.5 (63.5), 0.75 (53.8) and 1.25 (63.0, read from the 1.25 row as 100 − 37.0); 2.0 is far worse (8.5). "
          "Look-ahead weight: 0.5 beats 1.0 against the same opponent (64.7 vs 58.6). Both are now the default."),

    ('h', "5. Diagnostics: run 3 bot and new bot (60 self-play games each)"),
    ('table', [
        ["Measure", "Run 3 bot", "New bot"],
        ["Wins US / USSR / draws", "29 / 30 / 1", "26 / 34 / 0"],
        ["Ends: Final Scoring / 20 VP / Wargames / DEFCON 1", "30 / 18 / 12 / 0", "25 / 26 / 9 / 0"],
        ["Scoring cards: share played at a loss; mean net VP for the player", "31%; +0.50", "17%; +2.69"],
        ["Average Ops played, Action Round 1 → 7 (US)", "2.0 → 2.3", "3.0 → 2.1"],
        ["Space Race attempts per game", "—", "5.9"],
        ["Opponent Events fired by own plays per game (US / USSR)", "16.3 / 16.3", "14.0 / 15.2"],
        ["Avoidable damage per game (US / USSR)", "3.00 / 2.30", "3.13 / 3.11"],
        ["Fired opponent cards that had a cost-0 moment while held (US / USSR)", "36% / 47%", "37% / 50%"],
    ], (0.5, 0.25, 0.25)),
    ('p', "<b>Avoidable damage</b> (new diagnostic, as in Red Sea v6): at every Action Round the cost of firing each held opponent card is measured; "
          "when a card fires, its cost is compared with its cheapest moment while held. About 3 evaluation points per game per side are lost by "
          "firing opponent Events at a worse moment than one already passed; 37–50% of fired cards had a free moment. The most avoidable: Five Year Plan "
          "for the USSR, Arab-Israeli War and Korean War for the US. Neither bot handles this yet; it is the next target, as it is in Red Sea."),
    ('p', "A complete self-play game of the new bot was read (seed 5003): CIA Created played early at DEFCON 4, Scoring cards timed into favourable "
          "boards, Quagmire handled by the rules, a 20 VP win in turn 7. No anomalies found."),

    ('h', "6. Ablation repeated on the new bot (400 deals × both sides)"),
    ('p', "Reference: the new bot (near-Control 1.0 + look-ahead 0.5). Below 50 means the term was helping."),
    ('results', ['results.jsonl'], ['A4_milops_off', 'A4_trap_off', 'A4_access_off', 'A4_region_unseen_off', 'A4_bg_off', 'A4_trapped_off', 'A4_bad_off', 'A4_held_off', 'A4_inf_off', 'A4_space_ab_off', 'ACCESS_off_fresh']),
    ('p', "Four terms carry the evaluation: Region standing (8.8 without it), Battlegrounds with near-Control (9.0), the held Scoring card (29.4) and Military Ops (44.2). Without Region standing or Battlegrounds the bot collapses, as it did without near-Control. Influence (+0.05 per point) and the DEFCON trap penalty were needed in run 2 but are neutral now: near-Control and the DEFCON limits do their work. The access penalty (an Influence point in every Region) is harmful (56.1 ± 2.7; 55.1 ± 1.8 on fresh deals): it spends Ops on token presence where near-Control and Region standing already value a real presence. The as-US / as-USSR columns follow the self-play balance of the new bot, which favours the USSR (US 26, USSR 34 of 60 games)."),

    ('h', "7. What changed, and next steps"),
    ('p', "<b>Adopted as default:</b> near-Control credit 1.0 (<i>nc</i>), two-move look-ahead 0.5 (<i>l2</i>) and the access term switched off (<i>access</i> 0). "
          "<b>Added tools:</b> the move-change pre-check (<i>tssim precheck</i>), the avoidable-damage cost log (TSSIM_COSTLOG=1), the block queue "
          "for cloud + Mac (<i>scripts/blocks.py</i>)."),
    ('p', "<b>Next:</b> (1) avoidable damage: time the firing of opponent Events (Red Sea's shaping and hand plan did not help there, so start "
          "from the cost log and position tests); (2) the neutral terms (Quagmire/Bear Trap, liabilities, Influence, Space abilities) are candidates "
          "for simplification; (3) the TS game skill (first version), its helper script and the cross-check with the engine; (4) Stage 3 plans, "
          "after Red Sea's v7 result on choosing a plan from the opening hand."),
]
