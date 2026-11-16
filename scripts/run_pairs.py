"""Run head-to-head pairings with tssim and append summaries to a JSONL file.
Usage: run_pairs.py OUT.jsonl SEEDS SEED0 'A|B|label' ['A|B|label' ...]"""
import json, subprocess, sys, os, time
out, n, seed0 = sys.argv[1], sys.argv[2], sys.argv[3]
binp = os.environ.get('TSSIM_BIN') or os.path.join(os.path.dirname(__file__), '..', 'engine', 'target', 'release', 'tssim')
for spec in sys.argv[4:]:
    a, b, label = spec.split('|')
    csv = os.path.join(os.path.dirname(out), f'{label}.csv')
    t0 = time.time()
    r = subprocess.run([binp, 'match', a, b, n, seed0, '2', csv], capture_output=True, text=True, check=True)
    res = json.loads(r.stdout.strip().splitlines()[-1]); res['label'] = label; res['secs'] = round(time.time() - t0, 1); res['seed0'] = int(seed0)
    open(out, 'a').write(json.dumps(res) + '\n')
    print(f"{label:22} score {res['score']*100:5.1f} ±{res['margin']*100:4.1f}  as US {res['a_as_us']*100:5.1f}  as USSR {res['a_as_ussr']*100:5.1f}  ({res['secs']}s)", flush=True)
