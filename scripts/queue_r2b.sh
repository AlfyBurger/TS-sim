#!/bin/sh
# Run 2, series B: Headline v2, known card played sooner, and the self-play reference for per-side baselines.
cd "$(dirname "$0")/.."
sleep 5
while pgrep -f "queue_r2a.sh" >/dev/null || pgrep -f "tssim_h13" >/dev/null; do sleep 20; done
TSSIM_BIN=bin/tssim_r2b python3 -I scripts/run_pairs.py results/run2/features.jsonl 1000 100000 \
  'core:hl=1|core|F_headline_v2' 'core:sq=0.5,km=1|core:sq=0.5|F_known_sooner' 'core|core|REF_self_play' \
  >> results/run2/features.log 2>&1
