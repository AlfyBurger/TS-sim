"""Summarise `tssim diag` output (Rust debug tuples converted to JSON)."""
import json, sys, collections
raw = open(sys.argv[1]).read().replace('(', '[').replace(')', ']')
d = json.loads(raw)
n = len(d)
print('games', n, 'US wins', sum(g['winner'] == 0 for g in d), 'USSR wins', sum(g['winner'] == 1 for g in d), 'draws', sum(g['winner'] == 2 for g in d))
print('ends', dict(collections.Counter(g['end'] for g in d)), 'avg final turn %.1f' % (sum(g['turn'] for g in d) / n))
sa = [x for g in d for x in g['space']]
print('space attempts/game %.2f, share with opponent Event %.2f' % (len(sa) / n, sum(1 for x in sa if x[3]) / max(1, len(sa))))
print('China Card plays/game %.2f' % (sum(len(g['china']) for g in d) / n))
print('opponent Events triggered by Ops/game %.2f' % (sum(len(g['opp_events']) for g in d) / n))
print('coups/game US %.1f USSR %.1f; realign rolls/game US %.1f USSR %.1f' % tuple(sum(g[k][i] for g in d) / n for k in ('coups', 'realigns') for i in (0, 1)))
print('Military Ops penalty/game US %.2f USSR %.2f' % tuple(sum(g['milops_pen'][i] for g in d) / n for i in (0, 1)))
sc = [x for g in d for x in g['scoring']]
print('Scoring plays/game %.2f, mean net for the player %.2f, share played at a loss %.2f' % (len(sc) / n, sum(x[3] for x in sc) / max(1, len(sc)), sum(1 for x in sc if x[3] < 0) / max(1, len(sc))))
print('DEFCON-loss cards', dict(collections.Counter(g['defcon_card'] for g in d if g['end'] == 'DEFCON 1')))
