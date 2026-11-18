# Run 6 plan (Stage 2; agreed with Alfy 18 Nov 2026)

Order of the job list: everything added today goes BEFORE the last item.
1. Distance to Control (port of Red Sea v7 `ne`, +16 there): importance x (0.5^Ops I need - 0.5^Ops they need), incl. the cost of
   breaking their Control and +1 when no adjacent Influence. Strengths 1, 2, 3; on top of the current near-Control term and replacing it;
   importance table for the 84 countries (draft: results/run6/importance_draft.csv, sent to Alfy as a Sheet for review, 18 Nov). Best variant confirmed on fresh deals.
2. Headline v2 for the USSR only (Red Sea adopted it for the USSR; TS run 2 on the old bot: neutral, sides reversed).
1b. Importance table with Alfy's corrections (18 Nov): separate Attack and Defend values. Europe: Defend = 1.3 x base (top priority),
   Attack = base with a steeper decay (0.4 per Op instead of 0.5: cheap chances count, long pushes little). Africa and South America:
   Attack = 1.25 x base. Raised: Malaysia, Spain/Portugal, Costa Rica, Uruguay, Morocco. Test the table against flat importance.
2b. Diagnostic for a later two-column (US/USSR) table (Alfy): over all run-6 games, the average score each side gets in each Region
   (at each Scoring card and at Final Scoring), by era.
3. (further items agreed today go here)
LAST. Set-up values for preparation cards (`ss`): weights 0.2 and 0.4 on the current bot, plus the payoff check
   (realised Event value of preparation cards when played, as in Red Sea v5: +8-14% there).
Not in run 6: hand plan and shaping (wait for the new Red Sea tests); Stage 3-5 items (later).

## As launched (18 Nov 2026, 12:09 Amman, Alfy's PC, folder Game Dev\TS: tssim6.exe, run6_jobs.txt, out6\, status6.txt, RUN6_COMPLETE.txt)
160 blocks of 50 deals (both sides), in this order:
- A (80): NE1, NE2, NE3 (ne on top of the one-short credit), NE1R, NE2R, NE3R (ne replacing it: nc=0), FLAT2, FLAT2R (flat importance, ne=2),
  each core:<variant> vs core, 500 deals, seeds 800000-800499, blocks interleaved.
- CONFIRM_NE (20): BEST{all 8} vs core, fresh seeds 810000-810999 (the worker picks the screening variant with the highest score, or core if none > 50%; logged in out6\\resolved.txt).
- HL_USSR (20): BEST,hl=2 vs BEST, seeds 820000-820999.
- SS_02, SS_04 (40): BEST,ss=0.2 / 0.4 vs BEST, seeds 830000-830999 (last, as agreed).
Every game line carries the Region record (Scoring cards and Final Scoring nets per Region) and the preparation-card payoffs.
Decision rules (set before the data): adopt the confirmed distance-to-Control variant if its fresh-deal score minus margin > 50;
Headline v2 (USSR) and set-up values likewise (score - margin > 50 against the adopted bot).

## Second part ("6b"), 18 Nov 2026 (restarted 18:39 after an idle gap from 16:06)
First screening: all 8 versions far below 50% (best NE1R 34.6, NE3 0.2). Cause: in the Red Sea form a Controlled country earns no
credit (Red Sea values Control elsewhere; TS only for Battlegrounds), so the bot preferred staying one short and counted the
opponent's Control as good. Fix `nv=2`: Control counts in full (need 0 -> 1.0). Inserted before the set-up values tests:
V2_05, V2_1 (on top), V2_05R, V2_1R, V2_2R (replacing the one-short credit), V2F_1R (flat), 500 deals each, seeds 840000+;
CONFIRM_V2: BEST{V2...} vs core, 1000 fresh deals, seeds 850000+. Program tssim6b.exe, job list run6b_jobs.txt, same out6\.
CONFIRM_NE ran as core vs core (no version passed): it serves as the clean self-play record for the Region-by-side analysis.
