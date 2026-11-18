"""Region record (run 6): which side scores better in each Region, by era and at Final Scoring.

Reads the 7th CSV field of every game line in RUNDIR/blocks/LABEL__*.csv:
  S<turn>:<card>:<net, + = US>   each Scoring card played
  F<region>:<net>                Final Scoring (0 Europe, 1 Asia, 2 Middle East, 3 Africa, 4 Central America, 5 South America)
  P<turn>:<side>:<card>:<gain>   own preparation-card Events (not used here)

  region_record.py RUNDIR LABEL [--self] [--json OUT]
--self: self-play (same bot both sides); each deal's two lines are the same game, so only one is counted.
"""
import glob, json, os, sys

CARD_REGION = {2: 'Europe', 1: 'Asia', 3: 'Middle East', 79: 'Africa', 37: 'Central America', 81: 'South America', 38: 'Southeast Asia'}
FINAL_REGION = ['Europe', 'Asia', 'Middle East', 'Africa', 'Central America', 'South America']
ORDER = ['Europe', 'Asia', 'Southeast Asia', 'Middle East', 'Africa', 'Central America', 'South America']
ERAS = [('Early (1-3)', 1, 3), ('Mid (4-7)', 4, 7), ('Late (8-10)', 8, 10)]


def era(turn):
    for name, a, b in ERAS:
        if a <= turn <= b: return name


def games(rundir, label, self_play):
    for fn in sorted(glob.glob(os.path.join(rundir, 'blocks', f'{label}__*.csv'))):
        for line in open(fn).read().splitlines():
            if not line.strip(): continue
            f = line.split(',', 6)
            if self_play and f[1] != '0': continue
            yield {'seed': int(f[0]), 'flag': int(f[1]), 'winner': int(f[2]), 'vp': int(f[3]), 'end': int(f[4]),
                   'turn': int(f[5]), 'rec': f[6].split(';') if len(f) > 6 and f[6] else []}


def stats(xs):
    n = len(xs)
    if not n: return {'n': 0}
    return {'n': n, 'mean': sum(xs) / n, 'us': sum(x > 0 for x in xs) / n, 'ussr': sum(x < 0 for x in xs) / n,
            'tie': sum(x == 0 for x in xs) / n}


def analyse(rundir, label, self_play):
    sc = {}   # (region, era) -> nets
    fin = {r: [] for r in FINAL_REGION}
    ng = 0; nfinal = 0
    for g in games(rundir, label, self_play):
        ng += 1; had_final = False
        for item in g['rec']:
            if item.startswith('S'):
                t, k, n = item[1:].split(':'); reg = CARD_REGION[int(k)]
                sc.setdefault((reg, era(int(t))), []).append(int(n)); sc.setdefault((reg, 'All'), []).append(int(n))
            elif item.startswith('F'):
                r, n = item[1:].split(':'); fin[FINAL_REGION[int(r)]].append(int(n)); had_final = True
        nfinal += had_final
    return {'label': label, 'games': ng, 'final_games': nfinal,
            'scoring': {f'{r}|{e}': stats(v) for (r, e), v in sc.items()},
            'final': {r: stats(v) for r, v in fin.items()}}


def fmt(s):
    if not s or not s.get('n'): return '      -                       '
    return f"{s['n']:5d} {s['mean']:+6.2f}  {s['us']*100:4.0f}/{s['ussr']*100:4.0f}/{s['tie']*100:4.0f}"


def show(res):
    print(f"{res['label']}: {res['games']} games, {res['final_games']} reached Final Scoring")
    print('Scoring cards: plays, mean net (+ = US), % won by US / USSR / tie')
    eras = [e[0] for e in ERAS] + ['All']
    print(f"{'Region':16}" + ''.join(f'{e:>32}' for e in eras))
    for r in ORDER:
        print(f'{r:16}' + ''.join(f"{fmt(res['scoring'].get(f'{r}|{e}')):>32}" for e in eras))
    print('Final Scoring:')
    for r in FINAL_REGION:
        print(f'{r:16}{fmt(res["final"][r]):>32}')


if __name__ == '__main__':
    a = sys.argv[1:]
    self_play = '--self' in a; out = a[a.index('--json') + 1] if '--json' in a else None
    res = analyse(a[0], a[1], self_play); show(res)
    if out: json.dump(res, open(out, 'w'), indent=1)
