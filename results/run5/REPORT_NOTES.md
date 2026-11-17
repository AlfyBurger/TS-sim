# Run 5 report: extra requirements (Alfy, 17 Nov 2026)

At the end of the run 5 report, after all tests are complete, add:
1. **How the bot evolved** — a plain-language section walking through each run (1 to 5): what was added or
   changed, what the test showed, and how much stronger the bot became. No jargon; explain terms like
   "near-Control" or "look-ahead" in a phrase.
2. **A small graph of strength gains** — one chart, run by run.
   Data: chain of head-to-head scores of each run's final bot against the previous one
   (run 1: vs random play ~99-100%; run 2 vs run 1: 59.9 ± 2.0; run 3 vs run 2: DEFCON limits, see
   results/run3/TSB_defcon_boundaries.csv; run 4 vs run 3: 94.8 ± 0.9; run 5: final adopted bot vs run 4 bot).
   Convert each score to a rating gain (Elo: 400*log10(p/(1-p))) and plot the cumulative rating with
   error bars, run 1 = 0. Say in the text that chained ratings are approximate.
Report: 1.5 cm left/right margins; commit to reports/; copy to the Mac's Desktop/TS when a chat is linked there.
