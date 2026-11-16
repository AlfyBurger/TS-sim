#!/bin/sh
# Run 2, series C: combined bot on fresh deals, timing on top of sequencing, delta sweep.
cd "$(dirname "$0")/.."
sleep 5
while pgrep -f "queue_r2b.sh" >/dev/null; do sleep 20; done
TSSIM_BIN=bin/tssim_r2b python3 -I scripts/run_pairs.py results/run2/combined.jsonl 1000 200000 \
  'core:sq=0.5,tm=1|core|C_combined_fresh' >> results/run2/combined.log 2>&1
TSSIM_BIN=bin/tssim_r2b python3 -I scripts/run_pairs.py results/run2/combined.jsonl 1000 100000 \
  'core:sq=0.5,tm=1|core:sq=0.5|C_timing_on_top_of_seq' 'core:sq=0.3,tm=1|core:sq=0.5,tm=1|C_delta_0.3_vs_0.5' 'core:sq=0.7,tm=1|core:sq=0.5,tm=1|C_delta_0.7_vs_0.5' \
  >> results/run2/combined.log 2>&1
