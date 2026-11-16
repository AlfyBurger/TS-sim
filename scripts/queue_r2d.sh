#!/bin/sh
# Run 2, series D: ablation on the combined bot, Headline v2 for the USSR only, China Card hold (H7), diagnostics.
cd "$(dirname "$0")/.."
sleep 5
while pgrep -f "queue_r2c.sh" >/dev/null; do sleep 20; done
B=bin/tssim_r2b; N='core:sq=0.5,tm=1'
TSSIM_BIN=$B python3 -I scripts/run_pairs.py results/run2/combined.jsonl 1000 100000 \
  "$N,hl=2|$N|D_headline_v2_ussr_only" "$N,china=1.5|$N|D_china_hold_1.5" >> results/run2/combined.log 2>&1
$B diag 200 5000 core > results/run2/diag_core.json 2>/dev/null
$B diag 200 5000 "$N" > results/run2/diag_combined.json 2>/dev/null
TSSIM_BIN=$B python3 -I scripts/run_pairs.py results/run2/ablation.jsonl 400 30000 \
  "$N,milops=0|$N|A2_milops_off" "$N,trap=0|$N|A2_trap_off" "$N,access=0|$N|A2_access_off" "$N,region_unseen=0|$N|A2_region_unseen_off" \
  "$N,bg=0|$N|A2_bg_off" "$N,trapped=0|$N|A2_trapped_off" "$N,bad=0|$N|A2_liabilities_off" "$N,held=0|$N|A2_held_scoring_off" \
  "$N,inf=0|$N|A2_influence_off" "$N,space_ab=0|$N|A2_space_abilities_off" >> results/run2/ablation.log 2>&1
