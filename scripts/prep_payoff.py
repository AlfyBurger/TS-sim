"""Preparation payoff (run 6): realised core-evaluation gain of own preparation-card Events, per bot in a test.

Uses the P<turn>:<side 0 US / 1 USSR>:<card>:<gain> items of the Region record. In a line with flag 0, bot A is the US;
with flag 1, bot A is the USSR. Compares bot A (e.g. core,ss=0.2) with bot B (core) over the same deals.

  prep_payoff.py RUNDIR LABEL [LABEL...]
"""
import glob, math, os, sys


def collect(rundir, label):
    out = {'A': [], 'B': []}   # per game: (count, total gain)
    for fn in sorted(glob.glob(os.path.join(rundir, 'blocks', f'{label}__*.csv'))):
        for line in open(fn).read().splitlines():
            f = line.split(',', 6)
            if len(f) < 6: continue
            a_side = int(f[1]); per = {'A': [0, 0.0], 'B': [0, 0.0]}
            for item in (f[6].split(';') if len(f) > 6 and f[6] else []):
                if not item.startswith('P'): continue
                _, sd, _, g = item[1:].split(':')
                who = 'A' if int(sd) == a_side else 'B'
                per[who][0] += 1; per[who][1] += float(g)
            for w in 'AB': out[w].append(tuple(per[w]))
    return out


def ms(xs):
    n = len(xs); m = sum(xs) / n; sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1)) if n > 1 else 0
    return m, 1.96 * sd / math.sqrt(n)


if __name__ == '__main__':
    rundir = sys.argv[1]
    print(f"{'test':12} {'bot':4} {'Events/game':>12} {'gain/game':>16} {'gain/Event':>11}")
    for lab in sys.argv[2:]:
        d = collect(rundir, lab)
        if not d['A']: print(lab, 'no results'); continue
        res = {}
        for w in 'AB':
            cnt = [c for c, _ in d[w]]; tot = [g for _, g in d[w]]
            m, e = ms(tot); res[w] = m
            print(f"{lab:12} {w:4} {sum(cnt)/len(cnt):12.2f} {m:9.2f} ± {e:4.2f} {sum(tot)/max(1,sum(cnt)):11.2f}")
        diff = [ga - gb for (_, ga), (_, gb) in zip(d['A'], d['B'])]; m, e = ms(diff)
        print(f"{lab:12} A-B  gain/game {m:+.2f} ± {e:.2f} ({(res['A']/res['B']-1)*100:+.0f}%)")
