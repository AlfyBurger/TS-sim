#!/bin/sh
# Run 2, series A: each new decision rule alone against the current bot (1,000 deals, both sides).
cd "$(dirname "$0")/.."
while pgrep -f "tssim_h13" >/dev/null; do sleep 20; done
TSSIM_BIN=bin/tssim_r2a python3 -I scripts/run_pairs.py results/run2/features.jsonl 1000 100000 \
  'core:la=2|core|F_reply_scan_m2' 'core:tm=1|core|F_scoring_timing' 'core:sq=0.5|core|F_sequencing_d05' 'core:ss=0.4|core|F_setup_values_04' \
  >> results/run2/features.log 2>&1
