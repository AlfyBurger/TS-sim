#!/bin/sh
# Cloud runner: take one block at a time from the front of RUNDIR/jobs.txt until the queue is empty (or STOP exists).
R=$1; BIN=$2; THREADS=${3:-2}
cd "$(dirname "$0")/.."
while [ ! -e "$R/STOP" ]; do
  : > "$R/cloud_cur.txt"
  python3 -I scripts/blocks.py claim "$R" front 1 "$R/cloud_cur.txt" | grep -q "^1 claimed" || break
  python3 -I scripts/blocks.py run "$R/cloud_cur.txt" "$BIN" "$THREADS" 100000 "$R/blocks" >> "$R/cloud.log"
done
echo "cloud loop ended $(date)" >> "$R/cloud.log"
