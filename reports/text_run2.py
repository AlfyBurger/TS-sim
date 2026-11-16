TITLE = "Twilight Struggle simulator — Stage 2, test run 2"
SUBTITLE = ("For Alfy · night of 16–17 November 2026 · repository AlfyBurger/TS-sim · 2026 Hall of Fame rules, base game, fixed +2 US handicap · "
            "decision rules ported from the tested Red Sea bot, plus your hypothesis H13")
FOOTER = "TS-sim · Stage 2 run 2 test report"

VERDICTS = {
    "H13_final_from_t8": "<b>Worse</b>",
    "H13_final_from_t9": "Neutral",
    "H13_final_from_t10": "Neutral overall; US +1.7 ± 1.4, USSR −1.1 ± 1.3",
    "F_reply_scan_m2": "<b>Gain</b>",
    "F_scoring_timing": "<b>Gain</b>",
    "F_sequencing_d05": "<b>Largest gain</b>",
    "F_setup_values_04": "Neutral",
    "F_headline_v2": "Neutral (both sides)",
    "F_known_sooner": "Neutral",
    "REF_self_play": "Reference: identical bots",
    "C_combined_fresh": "<b>Confirmed</b> on fresh deals",
    "C_timing_on_top_of_seq": "Adds nothing on top of sequencing",
    "C_delta_0.3_vs_0.5": "Neutral: keep 0.5",
    "C_delta_0.7_vs_0.5": "Slightly worse: keep 0.5",
    "D_china_hold_1.5": "Gain, just past the margin",
    "E_china_hold_fresh": "<b>Confirmed</b>: adopted",
    "D_H13_t10_on_combined": "Neutral (US 0.0 ± 1.3, USSR −1.1 ± 1.5)",
    "E_defcon_fix": "Neutral overall; cuts DEFCON losses (section 6)",
    "FINAL_vs_run1_bot": "<b>Final run 2 bot vs run 1 bot</b>, new deals",
    "REF_combined_self_play": "Reference: identical bots",
}
for k in ["milops", "trap", "access", "region_unseen", "bg", "trapped", "liabilities", "held_scoring", "influence", "space_abilities"]:
    VERDICTS.setdefault("A2_" + k + "_off", "")

SECTIONS = [
    ('h', "1. Summary"),
    ('p', "Run 2 added the decision rules that made the Red Sea bot stronger, tested your new hypothesis H13, and repeated the ablation on the "
          "new bot. Every test used 1,000 deals played once from each side (2,000 games) unless stated, against the bot adopted in run 1."),
    ('p', "<b>Main findings.</b> (1) <b>Card sequencing</b> is the biggest gain (56.9 ± 1.9): the run 1 bot spent its strongest cards first "
          "(about 3.4 Ops in Action Round 1, 1.0 by round 7); the new bot keeps strong cards for later, counting only half their value as safe. "
          "(2) <b>The opponent's reply</b> (an exact scan of the cards they could hold) gains 4.1 points on its own, and <b>Scoring timing</b> 3.1; "
          "on top of sequencing, timing adds nothing more, because sequencing already charges each card the opponent's reply. "
          "(3) The combined bot scored <b>57.6 ± 1.9</b> on fresh deals. (4) <b>Your H13</b> (Final Scoring projection switched on near the end): "
          "from turn 8 it is harmful, from turn 9 or 10 neutral. Not adopted. (5) <b>Holding the China Card (H7)</b> is now confirmed: "
          "+2 to +3 in three separate tests. Adopted. (6) Headline v2, set-up values (H9) and playing known cards sooner were neutral. "
          "(7) Two DEFCON bugs were found through the new diagnostics and fixed."),
    ('p', "<b>The final run 2 bot</b> (sequencing, timing, China Card hold, DEFCON fixes) scores <b>59.9 ± 2.0</b> against the run 1 bot on new deals (300000–300999): 55.1% as US and 64.6% as USSR. Its self-play is balanced: US 49.7%, USSR 50.3% for the combined bot on deals 100000–100999."),

    ('h', "2. How to read the numbers"),
    ('p', "<b>Score</b>: the tested bot's share of points against the reference bot (win 1, draw ½), each deal played once from each side. "
          "50 = no difference. <b>±</b>: the 95% margin, from the paired results deal by deal. A result counts only outside 50 ± margin. "
          "With 1,000 deals the margin is about ±2 points, against ±3 in run 1 (400 deals). "
          "<b>Per side</b>: the 'as US' and 'as USSR' columns are win rates; section 4 converts them into effects against a self-play reference on "
          "the same deals (US wins 46.4%, USSR 53.6% there). For an ablation (a term switched off), below 50 means the term was helping."),

    ('h', "3. Your hypothesis H13: Final Scoring near the end"),
    ('p', "The projection (what Final Scoring would give if the game ended now) was switched on only from a given turn, against the run 1 bot, "
          "which does not use it at all. Up to that turn the games are identical, which is why the margins are small."),
    ('results', ['h13.jsonl'], None),
    ('p', "From turn 8 it costs 3.6 points, mostly as USSR (−5.5 ± 2.4). From turn 9 it is neutral; from turn 10 neutral overall, with a small gain "
          "as US (+1.7 ± 1.4) and a small loss as USSR (−1.1 ± 1.3). Re-tested on the run 2 bot from turn 10: 49.5 ± 0.8, neutral. "
          "<b>Why it does not help:</b> the evaluation already values every Region's standing (Battlegrounds, Region scores while their Scoring cards "
          "are in play), so near the end the projection mostly double-counts what is there. Final Scoring does decide 43–56% of these games, but the "
          "bot is already playing for it through those terms."),

    ('h', "4. New decision rules, each alone"),
    ('results', ['features.jsonl'], None),
    ('table', [
        ["Rule (from Red Sea)", "Effect as US", "Effect as USSR", "Red Sea result"],
        ["Opponent reply scan, override margin 2", "+7.5 ± 3.6", "+0.8 ± 3.8", "about +2 ± 4"],
        ["Scoring timing: score now or build first", "+3.0 ± 3.8", "+3.2 ± 3.9", "+3.3"],
        ["Card sequencing, δ 0.5", "<b>+9.6 ± 3.9</b>", "<b>+4.2 ± 3.8</b>", "+2.05"],
        ["Set-up values 0.4 (H9)", "+2.5 ± 3.5", "−0.4 ± 3.8", "+0.85"],
        ["Headline v2", "+1.9 ± 3.8", "−0.5 ± 3.8", "US −4.35, USSR +3.9"],
    ], (0.4, 0.2, 0.2, 0.2)),
    ('p', "Per-side effects are against the self-play reference on the same deals. Sequencing pays more in Twilight Struggle than in Red Sea: "
          "turns are longer and the run 1 bot front-loaded its Ops heavily. Unlike Red Sea, Headline v2 is neutral on both sides, so it is not adopted."),

    ('h', "5. Combining and tuning"),
    ('results', ['combined.jsonl'], ['C_combined_fresh', 'C_timing_on_top_of_seq', 'C_delta_0.3_vs_0.5', 'C_delta_0.7_vs_0.5', 'D_china_hold_1.5', 'E_china_hold_fresh', 'D_H13_t10_on_combined', 'E_defcon_fix', 'REF_combined_self_play', 'FINAL_vs_run1_bot']),
    ('p', "The combined bot (sequencing + timing) was confirmed on fresh deals 200000–200999. Timing is kept: it costs nothing and is the rule that "
          "times a held Scoring card when sequencing's shortlist does not include it. δ = 0.5 stays (as in Red Sea). "
          "China Card holding value 1.5 was confirmed on fresh deals and adopted."),

    ('h', "6. DEFCON losses: what the diagnostics found"),
    ('p', "The new diagnostics record every DEFCON 1 loss with its card and phase. On the run 2 bot, 13 of 200 self-play games ended at DEFCON 1 "
          "(22 for the run 1 bot). The causes were concentrated:"),
    ('table', [
        ["Cause", "Losses (run 1 bot / run 2 bot, of 200)", "Why the bot missed it", "Fix"],
        ["Grain Sales to Soviets played by the USSR for Ops", "6 / 6", "The US draws a random card from the USSR hand; if it lowers DEFCON, the USSR loses on its own round. The simulation counted dice but not card draws, so it looked at one draw only", "Card draws now count as random: 18 samples instead of 1"],
        ["Olympic Games as Headline", "5 / 4", "The other Headline can lower DEFCON first; the boycott then ends the game. The Headline was valued at the current DEFCON", "No Olympic Games or Summit Headline at DEFCON 3 or lower"],
        ["Summit as an Event", "9 / 2", "Summit's winner may lower DEFCON on the loser's round", "Partly covered by the reply scan; to check"],
        ["Lone Gunman played for Ops (US)", "2 / 0", "The USSR Coup on the US round", "Gone with the reply scan"],
        ["Missile Envy as an Event", "0 / 1", "Not yet traced (one game)", "To check"],
    ], (0.24, 0.2, 0.36, 0.2)),
    ('p', "<b>Result of the two fixes</b> (switch <i>dfix</i>): 51.6 ± 1.9 on its own, as expected for a rare event. In the final bot's 200 self-play games, DEFCON losses fell to 7, with none left from Grain Sales or Olympic Games, and the USSR's damage from opponent Events fell from 38.5 to 10.4 per game. What remains: Summit as an Event (3) and four Headline-phase losses with other cards (Ortega Elected, Lone Gunman, Grain Sales, CIA Created), where the other side's Headline lowers DEFCON first."),

    ('h', "7. Diagnostics: how the bots play (200 self-play games each)"),
    ('table', [['Measure', 'Run 1 bot', 'Sequencing + timing', 'Final run 2 bot'], ['Wins US / USSR / draws', '105 / 92 / 3', '107 / 91 / 2', '89 / 102 / 9'], ['Ends: Final Scoring / 20 VP / DEFCON 1 / Wargames / held Scoring', '86 / 70 / 22 / 18 / 4', '112 / 60 / 13 / 15 / 0', '109 / 66 / 7 / 17 / 1'], ['Average Ops played, Action Round 1 → 7 (US)', '3.4 → 1.1', '2.2 → 2.2', '2.0 → 2.3'], ['Space Race attempts per game', '3.0', '4.3', '5.4'], ['China Card plays per game', '8.0', '8.7', '2.4'], ['Opponent Events fired by own plays, per game (US / USSR)', '14.1 / 15.0', '15.7 / 15.6', '16.3 / 16.9'], ['Damage from those Events per game, evaluation units (US / USSR)', '−18.6 / −39.5', '−9.0 / −38.5', '−10.0 / −10.4'], ['Scoring cards played per game; share at a loss', '9.6; 31%', '10.7; 33%', '11.5; 32%'], ['Military Ops penalty VP per game (US / USSR)', '2.2 / 1.6', '1.8 / 1.7', '1.9 / 1.5'], ['DEFCON 1 losses (of 200)', '22', '13', '7']], (0.46, 0.18, 0.18, 0.18)),
    ('p', "Self-play statistics such as Scoring cards played at a loss do not show progress on their own: both sides improve together. "
          "What they show is behaviour. Ops are now spread over the turn, the Space Race is used more, and games last longer and are decided at "
          "Final Scoring more often. Grain Sales accounted for most of the USSR's damage from opponent Events (DEFCON losses count heavily in that measure)."),

    ('h', "8. Ablation repeated on the run 2 bot"),
    ('p', "Each term of the evaluation switched off in the run 2 bot (sequencing + timing), 400 deals × both sides, deals 30000–30399. "
          "Compare with run 1 (old bot): Military Ops 41.9, DEFCON traps 46.6."),
    ('results', ['ablation.jsonl'], None),
    ('p', "ABLATION_TEXT"),

    ('h', "9. What changed in the bot, and next steps"),
    ('p', "<b>Adopted as the new default:</b> card sequencing δ 0.5, Scoring timing, opponent reply scan (inside sequencing), China Card holding value 1.5, "
          "and the two DEFCON fixes. <b>Kept as options, not adopted:</b> Headline v2, set-up values, playing known cards sooner, the late Final Scoring projection."),
    ('p', "<b>Next:</b> (1) a full check of every Event that lets the opponent change DEFCON on my round (Summit, Grain Sales, Olympic Games, "
          "Missile Envy and the like), with exact enumeration instead of samples; (2) tune the reply scan's shortlist (top 3 cards) and margin; "
          "(3) Stage 2 skill text for Twilight Struggle from these results; (4) the helper script and its cross-check with the engine; "
          "(5) then Stage 3: game plans, side and era styles (H3, H12), difficulty levels."),
]
