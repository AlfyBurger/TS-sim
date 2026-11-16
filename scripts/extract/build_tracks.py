import json, sys
tracks = {
 "source": "HoF 20th Anniversary rulebook 2026 (6.4, 8.1, 8.2, 10.2, 3.x); Space Race read from the track graphic p.12",
 "space_race": [  # box: ops needed, max roll, VP first/second, ability (first player only, cancelled when second arrives)
   {"box": 1, "name": "Earth Satellite", "ops": 2, "roll_max": 3, "vp": [2, 1], "ability": None},
   {"box": 2, "name": "Animal in Space", "ops": 2, "roll_max": 4, "vp": [0, 0], "ability": "may play 2 Space Race cards per turn"},
   {"box": 3, "name": "Man in Space", "ops": 2, "roll_max": 3, "vp": [2, 0], "ability": None},
   {"box": 4, "name": "Man in Earth Orbit", "ops": 2, "roll_max": 4, "vp": [0, 0], "ability": "opponent chooses and reveals Headline first"},
   {"box": 5, "name": "Lunar Orbit", "ops": 3, "roll_max": 3, "vp": [3, 1], "ability": None},
   {"box": 6, "name": "Eagle/Bear has Landed", "ops": 3, "roll_max": 4, "vp": [0, 0], "ability": "may discard 1 held card at end of turn"},
   {"box": 7, "name": "Space Shuttle", "ops": 3, "roll_max": 3, "vp": [4, 2], "ability": None},
   {"box": 8, "name": "Space Station", "ops": 4, "roll_max": 2, "vp": [2, 0], "ability": "may take 8 Action Rounds per turn"}],
 "defcon": {"start": 5, "max": 5, "no_coup_realign": {"4": ["Europe"], "3": ["Europe", "Asia"], "2": ["Europe", "Asia", "Middle East"]}, "1": "phasing player loses"},
 "milops": {"required": "= DEFCON", "penalty": "1 VP to opponent per missing point (net)", "max": 5},
 "vp": {"auto_win": 20},
 "turns": {"count": 10, "hand": {"1-3": 8, "4-10": 9}, "action_rounds": {"1-3": 6, "4-10": 7}, "mid_war_added": "end of turn 3", "late_war_added": "end of turn 7"},
 "scoring": {
   "Europe": {"presence": 3, "domination": 7, "control": "automatic victory", "adj_superpower_bonus": True},
   "Asia": {"presence": 3, "domination": 7, "control": 9, "adj_superpower_bonus": True},
   "Middle East": {"presence": 3, "domination": 5, "control": 7, "adj_superpower_bonus": False},
   "Central America": {"presence": 1, "domination": 3, "control": 5, "adj_superpower_bonus": True},
   "South America": {"presence": 2, "domination": 5, "control": 6, "adj_superpower_bonus": False},
   "Africa": {"presence": 1, "domination": 4, "control": 6, "adj_superpower_bonus": False},
   "Southeast Asia": {"per_country": 1, "Thailand": 2, "countries": ["Burma", "Laos/Cambodia", "Vietnam", "Malaysia", "Indonesia", "Philippines", "Thailand"], "note": "card #38, removed after play; at Final Scoring it is not scored separately (part of Asia)"}},
 "setup": {"china_card": "USSR, face up",
   "USSR": {"fixed": {"Syria": 1, "Iraq": 1, "N. Korea": 3, "E. Germany": 3, "Finland": 1}, "free": {"Eastern Europe": 6}},
   "US": {"fixed": {"Canada": 2, "Iran": 1, "Israel": 1, "Japan": 1, "Australia": 4, "Philippines": 1, "S. Korea": 1, "Panama": 1, "South Africa": 1, "U.K.": 5}, "free": {"Western Europe": 7}},
   "order": "deal 8 Early War cards each (USSR first); players see their cards; USSR places first, then US",
   "bidding_optional_13_4": "extra Influence only in countries with own Influence after setup, max 2 above what Control needs"}
}
json.dump(tracks, open(sys.argv[1], 'w'), indent=1)
m = json.load(open('map.json'))['countries']
from collections import defaultdict
st = defaultdict(lambda: [0, 0])
for n, c in m.items():
    for r in [c['region']] + c['sub']:
        st[r][0] += 1; st[r][1] += c['bg']
for r, (n, b) in st.items(): print(f'{r:16} countries {n:2}  battlegrounds {b}')
print('total', len(m), 'BG', sum(c['bg'] for c in m.values()))
# setup check vs VASSAL starting values captured earlier
v = json.load(open('vassal_countries.json'))
for side, key in (('US', 'American'), ('USSR', 'Soviet')):
    for c, n in tracks['setup'][side]['fixed'].items():
        got = v.get(c, {}).get(key)
        if got is not None and got != n: print('SETUP DIFF', side, c, 'rulebook', n, 'vassal', got)
