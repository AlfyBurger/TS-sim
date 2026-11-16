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
# --- run 2 diagnostics
if 'ops_by_ar' in d[0]:
    for s, nm in ((0, 'US'), (1, 'USSR')):
        row = []
        for r in range(1, 9):
            o = sum(g['ops_by_ar'][s][r][0] for g in d); c = sum(g['ops_by_ar'][s][r][1] for g in d)
            if c: row.append('AR%d %.2f' % (r, o / c))
        print('average Ops by Action Round', nm, ' '.join(row))
    names = ['Space Race', 'played, Event fired', 'played, Event void', 'Headlined']
    for s, nm in ((0, 'US'), (1, 'USSR')):
        print('opponent cards leaving the hand per game', nm, ', '.join('%s %.2f' % (names[k], sum(g['exits'][s][k] for g in d) / n) for k in range(4)))
    dm = [x for g in d for x in g['damage']]
    for s, nm in ((0, 'US'), (1, 'USSR')):
        v = [x[3] for x in dm if x[1] == s]
        print('opponent Events fired by own plays', nm, 'per game %.2f, mean eval change %.2f, total per game %.2f' % (len(v) / n, sum(v) / max(1, len(v)), sum(v) / n))
    worst = collections.defaultdict(list)
    for x in dm: worst[x[2]].append(x[3])
    top = sorted(worst.items(), key=lambda kv: sum(kv[1]))[:8]
    print('costliest fired opponent Events (card: times, total change)', '; '.join('%d: %d, %.1f' % (k, len(v), sum(v)) for k, v in top))
    dl = [g['defcon_loss'] for g in d if g.get('defcon_loss')]
    how = {0: 'own Ops/coup', 1: 'own Event', 2: 'Headline', 3: 'opponent Event via Ops'}
    print('DEFCON losses: ' + '; '.join('T%d AR%d loser %s card %d (%s)' % (x[0], x[1], 'US' if x[2] == 0 else 'USSR', x[3], how[x[4]]) for x in dl))
