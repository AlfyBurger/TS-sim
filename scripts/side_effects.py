"""Per-side effects against a reference run on the same deals.
Usage: side_effects.py REF.csv TEST.csv [TEST.csv ...]
REF must be the reference bot against itself (or against the same opponent as TEST's B) on the same seeds.
CSV rows: seed, flag (0 = tested bot is US), winner (0 US, 1 USSR, 2 draw), vp, end, turn."""
import csv, math, sys

def load(p):
    out = {}
    for seed, flag, win, *_ in csv.reader(open(p)):
        side = 0 if flag == '0' else 1
        w = int(win)
        out[(int(seed), side)] = 0.5 if w == 2 else (1.0 if w == side else 0.0)
    return out

ref = load(sys.argv[1])
for p in sys.argv[2:]:
    t = load(p)
    parts = []
    for side, nm in ((0, 'US'), (1, 'USSR')):
        d = [t[k] - ref[k] for k in t if k[1] == side and k in ref]
        n = len(d); m = sum(d) / n
        sd = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1))
        base = sum(ref[k] for k in t if k[1] == side and k in ref) / n
        parts.append(f"as {nm} {m*100:+.1f} ± {1.96*sd/math.sqrt(n)*100:.1f} (reference {base*100:.1f})")
    print(f"{p.split('/')[-1][:-4]:28} " + ';  '.join(parts))
