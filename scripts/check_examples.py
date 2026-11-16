"""Check rulebook and Background Book scoring examples against map.json/tracks.json."""
import json, sys
W = sys.argv[1]
M = json.load(open(W + '/map.json')); C, SP = M['countries'], M['superpowers']
T = json.load(open(W + '/tracks.json'))['scoring']
def ctrl(inf, c):
    u, r = inf.get(c, (0, 0)); s = C[c]['stab']
    return 'US' if u >= s and u - r >= s else 'USSR' if r >= s and r - u >= s else None
def score(inf, region):
    reg = [c for c in C if C[c]['region'] == region]
    bgs = [c for c in reg if C[c]['bg']]
    out = {}
    for side, enemy_sp in (('US', 'U.S.S.R.'), ('USSR', 'U.S.A.')):
        opp = 'USSR' if side == 'US' else 'US'
        mine = [c for c in reg if ctrl(inf, c) == side]; theirs = [c for c in reg if ctrl(inf, c) == opp]
        mb = [c for c in mine if C[c]['bg']]; tb = [c for c in theirs if C[c]['bg']]
        v = T[region]; pts = 0
        if mine:
            if len(mine) > len(theirs) and set(bgs) <= set(mine): pts = v['control']
            elif len(mine) > len(theirs) and len(mb) > len(tb) and len(mine) > len(mb): pts = v['domination']
            else: pts = v['presence']
        pts += len(mb)
        if v.get('adj_superpower_bonus'): pts += sum(1 for c in mine if c in SP[enemy_sp])
        out[side] = pts
    return out
ok = True
def check(name, got, want):
    global ok
    print(('OK  ' if got == want else 'FAIL') + f' {name}: got {got}, expected {want}'); ok &= got == want
# Rulebook 10.1 example (Central America)
inf = {'Cuba': (0, 3), 'Haiti': (0, 1), 'Dominican Rep': (0, 1), 'Guatemala': (1, 0), 'Panama': (1, 0)}
check('Rulebook 10.1 Central America', score(inf, 'Central America'), {'US': 1, 'USSR': 5})
# Background Book T1 AR4 Europe Scoring
inf = {'W. Germany': (4, 0), 'Italy': (2, 0), 'U.K.': (5, 0), 'Canada': (2, 0), 'France': (0, 0), 'Poland': (0, 3), 'E. Germany': (0, 3), 'Hungary': (0, 1), 'Czechoslovakia': (0, 1), 'Finland': (0, 1)}
check('Example T1 Europe Scoring', score(inf, 'Europe'), {'US': 5, 'USSR': 5})
# T1 AR6 Middle East Scoring
inf = {'Syria': (0, 2), 'Iraq': (0, 2), 'Iran': (0, 2), 'Israel': (1, 0)}
check('Example T1 Middle East Scoring', score(inf, 'Middle East'), {'US': 0, 'USSR': 6})
# T3 AR3 Asia Scoring
inf = {'Japan': (4, 0), 'S. Korea': (3, 0), 'Thailand': (3, 1), 'Australia': (4, 0), 'Philippines': (1, 0), 'Malaysia': (1, 1), 'Pakistan': (1, 0),
       'N. Korea': (0, 3), 'Vietnam': (0, 2), 'Laos/Cambodia': (0, 1)}
check('Example T3 Asia Scoring', score(inf, 'Asia'), {'US': 10, 'USSR': 4})
sys.exit(0 if ok else 1)
