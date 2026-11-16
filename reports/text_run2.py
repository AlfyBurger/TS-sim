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
    "E_china_hold_fresh": "CHINA_FRESH_VERDICT",
    "D_H13_t10_on_combined": "Neutral",
    "REF_combined_self_play": "Reference: identical bots",
}
for k in ["milops", "trap", "access", "region_unseen", "bg", "trapped", "liabilities", "held_scoring", "influence", "space_abilities"]:
    VERDICTS.setdefault("A2_" + k + "_off", "")

SECTIONS = []  # filled in by the final edit
