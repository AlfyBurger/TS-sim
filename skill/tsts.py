#!/usr/bin/env python3
"""tsts.py - Twilight Struggle helper (2026 HoF rules, base game, 110 cards). Stdlib only.
State: a JSON file (game.json). Influence as [US, USSR]; VP signed, + = US; sides "US"/"USSR".
Commands (GAME = path to game.json; SIDE = US or USSR):
  init GAME MYSIDE [HANDICAP]      new game: fixed setup Influence (free placements still to add)
  add GAME COUNTRY SIDE N          add (or remove, N<0) Influence; prints Control
  report GAME                      full position report for "me": Regions, scoring now, Scoring cards, hand flags
  score GAME [REGION|all]          Region scoring if played now
  coup GAME SIDE COUNTRY OPS [--free]   exact Coup outcomes and odds
  realign GAME SIDE COUNTRY [--free]    Realignment odds
  place GAME SIDE "A,A,B" [--apply]     check legality and cost of a placement (one name per marker)
  eval GAME [SIDE] [--w=k=v,...]   evaluation with its terms (simulator's default bot weights)
  whatif GAME SIDE JSON            value of a change, e.g. '{"inf":{"Italy":[0,2]},"vp":-1,"defcon":-1,"play":[17]}'
  decide GAME [--w=k=v,...]        best Ops use of every card in my hand, ranked (no Events, no reply)
  limits GAME                      DEFCON hard limits for my hand
  roll [N]                         roll N dice
  xcheck FILE                      compare with the simulator (`tssim xcheck N seed > FILE`)
"""
COUNTRY_TABLE = '''
Austria|0|4|EW|6,10,11,19
Benelux|0|3|W|18,19
Bulgaria|0|3|E|9,17
Canada|0|4|WU|18
Czechoslovakia|0|3|E|6,10,13
Denmark|0|3|W|16,19
E. Germany|0|3|EB|0,4,13,19
Finland|0|4|EWR|16
France|0|3|WB|46,11,15,18,19
Greece|0|2|W|2,11,17,20
Hungary|0|3|E|0,4,14,20
Italy|0|2|WB|0,8,9,15,20
Norway|0|4|W|16,18
Poland|0|3|EBR|4,6
Romania|0|3|ER|10,17,20
Spain/Portugal|0|2|W|8,11,53
Sweden|0|4|W|5,7,12
Turkey|0|2|W|2,9,14,45
U.K.|0|5|W|1,3,8,12
W. Germany|0|4|WB|0,1,5,6,8
Yugoslavia|0|3|E|9,10,11,14
Afghanistan|1|2|R|38,30
Australia|1|4||28
Burma|1|2|S|24,27
India|1|3|B|23,30
Indonesia|1|1|S|28,31
Japan|1|4|BU|31,32,33
Laos/Cambodia|1|1|S|23,34,35
Malaysia|1|2|S|22,25,34
N. Korea|1|3|BR|32
Pakistan|1|2|B|21,24,38
Philippines|1|2|S|25,26
S. Korea|1|3|B|26,29,33
Taiwan|1|3||26,32
Thailand|1|2|SB|27,28,35
Vietnam|1|1|S|27,34
Egypt|2|2|B|40,43,59
Gulf States|2|3||39,44
Iran|2|2|B|21,39,30
Iraq|2|3|B|37,38,41,44
Israel|2|4|B|36,41,42,45
Jordan|2|2||39,40,42,44
Lebanon|2|1||40,41,45
Libya|2|2|B|36,60
Saudi Arabia|2|3|B|37,39,41
Syria|2|2||40,42,17
Algeria|3|2|B|8,53,56,60
Angola|3|1|B|48,58,62
Botswana|3|2||47,58,63
Cameroon|3|1||54,62
Ethiopia|3|1||57,59
Ivory Coast|3|2||54,61
Kenya|3|2||55,57
Morocco|3|3||46,15,61
Nigeria|3|1|B|49,51,56
SE African States|3|1||52,63
Saharan States|3|1||46,54
Somalia|3|2||50,52
South Africa|3|3|B|47,48
Sudan|3|1||36,50
Tunisia|3|2||46,43
West African States|3|2||51,53
Zaire|3|1|B|47,49,63
Zimbabwe|3|1||48,55,62
Costa Rica|4|3||70,72,73
Cuba|4|3|BU|69,72
Dominican Rep|4|1||69
El Salvador|4|1||68,70
Guatemala|4|1||67,70,71
Haiti|4|1||65,66
Honduras|4|2||64,67,68,72
Mexico|4|2|BU|68
Nicaragua|4|1||64,65,70
Panama|4|2|B|78,64
Argentina|5|2|B|77,80,82
Bolivia|5|2||80,81
Brazil|5|2|B|82,83
Chile|5|3|B|74,81
Colombia|5|1||79,73,83
Ecuador|5|2||78,81
Paraguay|5|2||74,75,82
Peru|5|2||75,77,79
Uruguay|5|2||74,76,80
Venezuela|5|2|B|76,78
'''
CARD_TABLE = '''
1|Asia Scoring|0|0|0|S
2|Europe Scoring|0|0|0|S
3|Middle East Scoring|0|0|0|S
4|Duck and Cover|3|0|1|
5|Five Year Plan|3|0|1|
6|The China Card|4|0|0|
7|Socialist Governments|3|0|2|
8|Fidel|2|0|2|*
9|Vietnam Revolts|2|0|2|*
10|Blockade|1|0|2|*
11|Korean War|2|0|2|*
12|Romanian Abdication|1|0|2|*
13|Arab-Israeli War|2|0|2|
14|Comecon|3|0|2|*
15|Nasser|1|0|2|*
16|Warsaw Pact Formed|3|0|2|*U
17|De Gaulle Leads France|3|0|2|*U
18|Captured Nazi Scientist|1|0|0|*
19|Truman Doctrine|1|0|1|*
20|Olympic Games|2|0|0|
21|NATO|4|0|1|*U
22|Independent Reds|2|0|1|*
23|Marshall Plan|4|0|1|*U
24|Indo-Pakistani War|2|0|0|
25|Containment|3|0|1|*
26|CIA Created|1|0|1|*
27|US/Japan Mutual Defense Pact|4|0|1|*U
28|Suez Crisis|3|0|2|*
29|East European Unrest|3|0|1|
30|Decolonization|2|0|2|
31|Red Scare/Purge|4|0|0|
32|UN Intervention|1|0|0|
33|De-Stalinization|3|0|2|*
34|Nuclear Test Ban|4|0|0|
35|Formosan Resolution|2|0|1|*U
36|Brush War|3|1|0|
37|Central America Scoring|0|1|0|S
38|Southeast Asia Scoring|0|1|0|*S
39|Arms Race|3|1|0|
40|Cuban Missile Crisis|3|1|0|*
41|Nuclear Subs|2|1|1|*
42|Quagmire|3|1|2|*U
43|SALT Negotiations|3|1|0|*
44|Bear Trap|3|1|1|*U
45|Summit|1|1|0|
46|How I Learned to Stop Worrying|2|1|0|*
47|Junta|2|1|0|
48|Kitchen Debates|1|1|1|*
49|Missile Envy|2|1|0|
50|“We Will Bury You”|4|1|2|*
51|Brezhnev Doctrine|3|1|2|*
52|Portuguese Empire Crumbles|2|1|2|*
53|South African Unrest|2|1|2|
54|Allende|1|1|2|*
55|Willy Brandt|2|1|2|*U
56|Muslim Revolution|4|1|2|
57|ABM Treaty|4|1|0|
58|Cultural Revolution|3|1|2|*
59|Flower Power|4|1|2|*U
60|U2 Incident|3|1|2|*
61|OPEC|3|1|2|
62|“Lone Gunman”|1|1|2|*
63|Colonial Rear Guards|2|1|1|
64|Panama Canal Returned|1|1|1|*
65|Camp David Accords|2|1|1|*U
66|Puppet Governments|2|1|1|*
67|Grain Sales to Soviets|2|1|1|
68|John Paul II Elected Pope|2|1|1|*U
69|Latin American Death Squads|2|1|0|
70|OAS Founded|1|1|1|*
71|Nixon Plays the China Card|2|1|1|*
72|Sadat Expels Soviets|1|1|1|*
73|Shuttle Diplomacy|3|1|1|
74|The Voice of America|2|1|1|
75|Liberation Theology|2|1|2|
76|Ussuri River Skirmish|3|1|1|*
77|“Ask Not What Your Country…”|3|1|1|*
78|Alliance for Progress|3|1|1|*
79|Africa Scoring|0|1|0|S
80|“One Small Step…”|2|1|0|
81|South America Scoring|0|1|0|S
82|Iranian Hostage Crisis|3|2|2|*U
83|The Iron Lady|3|2|1|*U
84|Reagan Bombs Libya|2|2|1|*
85|Star Wars|2|2|1|*
86|North Sea Oil|3|2|1|*U
87|The Reformer|3|2|2|*U
88|Marine Barracks Bombing|2|2|2|*
89|Soviets Shoot Down KAL-007|4|2|1|*
90|Glasnost|4|2|2|*
91|Ortega Elected in Nicaragua|2|2|2|*
92|Terrorism|2|2|0|
93|Iran-Contra Scandal|2|2|2|*
94|Chernobyl|3|2|1|*
95|Latin American Debt Crisis|2|2|2|
96|Tear Down this Wall|3|2|1|*U
97|“An Evil Empire”|3|2|1|*U
98|Aldrich Ames Remix|3|2|2|*
99|Pershing II Deployed|3|2|2|*
100|Wargames|4|2|0|*
101|Solidarity|2|2|1|*
102|Iran-Iraq War|2|2|0|*
103|Defectors|2|0|1|
104|The Cambridge Five|2|0|2|
105|Special Relationship|2|0|1|
106|NORAD|3|0|1|*U
107|Che|3|1|2|
108|Our Man in Tehran|2|1|1|*
109|Yuri and Samantha|2|2|2|*
110|AWACS Sale to Saudis|3|2|1|*U
'''

# ---------------------------------------------------------------- data
US, USSR = 0, 1
SN = ["US", "USSR"]
def sid(x): return {"US": 0, "USSR": 1}[x.upper()] if isinstance(x, str) else int(x)
def opp(s): return 1 - s
REGIONS = ["Europe", "Asia", "Middle East", "Africa", "Central America", "South America"]
C = []
for line in COUNTRY_TABLE.strip().splitlines():
    n, r, stab, fl, adj = line.split("|")
    C.append(dict(name=n, region=int(r), stab=int(stab), ee="E" in fl, we="W" in fl, sea="S" in fl, bg="B" in fl,
                  adj_us="U" in fl, adj_ussr="R" in fl, adj=[int(a) for a in adj.split(",") if a]))
NC = len(C)
IDX = {c["name"].lower(): i for i, c in enumerate(C)}
CARDS = {}
for line in CARD_TABLE.strip().splitlines():
    i, n, ops, era, side, fl = line.split("|")
    CARDS[int(i)] = dict(name=n, ops=int(ops), era=int(era), side=int(side), star="*" in fl, scoring="S" in fl, underlined="U" in fl)
def owner(k): return {1: US, 2: USSR}.get(CARDS[k]["side"])
REGION_C = [[i for i, c in enumerate(C) if c["region"] == r] for r in range(6)]
SEA = [i for i, c in enumerate(C) if c["sea"]]
def cix(name):
    k = name.strip().lower()
    if k in IDX: return IDX[k]
    hits = [i for n, i in IDX.items() if n.startswith(k)]
    if len(hits) == 1: return hits[0]
    raise SystemExit(f"unknown or ambiguous country: {name}")
THAILAND, TAIWAN, JAPAN, FRANCE, W_GERMANY, CUBA = (cix(x) for x in ["Thailand", "Taiwan", "Japan", "France", "W. Germany", "Cuba"])
EUROPE, ASIA, MIDEAST, AFRICA, CENTAM, SOUTHAM = range(6)
CHINA = 6
SCORING_OF_REGION = [2, 1, 3, 79, 37, 81]
REGION_OF_SCORING = {1: ASIA, 2: EUROPE, 3: MIDEAST, 37: CENTAM, 79: AFRICA, 81: SOUTHAM}
# box: (Ops needed, roll max, VP first, VP second)
SPACE = [(0, 0, 0, 0), (2, 3, 2, 1), (2, 4, 0, 0), (2, 3, 2, 0), (2, 4, 0, 0), (3, 3, 3, 1), (3, 4, 0, 0), (3, 3, 4, 2), (4, 2, 2, 0)]
SPACE_NAMES = ["Start", "Earth Satellite", "Animal in Space", "Man in Space", "Man in Earth Orbit", "Lunar Orbit",
               "Eagle/Bear has Landed", "Space Shuttle", "Space Station"]
# Evaluation weights: identical to the simulator's default bot after Stage 2 run 5 (Weights::core).
BASE = dict(vp=1.0, held=0.9196, region_unseen=0.3182, final_=0.0, final_from=0.0, bg=1.1783, europe_mult=1.0, asia_mult=1.0,
            access=0.0, milops=1.0961, squeeze=0.0, trap=3.0, china=1.7139, bad=0.3, space_ab=0.5, inf=0.05, auto_eu=15.0,
            trapped=3.0, h3=0.0, nc=1.0745)
FLAGS = ["marshall", "warsaw", "nato", "de_gaulle", "willy_brandt", "us_japan", "formosan", "camp_david", "jp2", "flower_power",
         "evil_empire", "iron_lady", "north_sea_oil", "reformer", "tear_down", "awacs", "norad", "iranian_hostage", "shuttle",
         "quagmire", "bear_trap", "containment", "brezhnev", "vietnam_revolts", "nuclear_subs", "salt", "yuri", "iran_contra",
         "nso_turn", "u2"]

# ---------------------------------------------------------------- state
class St:
    def __init__(self, g):
        self.g = g
        self.inf = [[0, 0] for _ in range(NC)]
        for n, v in g.get("inf", {}).items(): self.inf[cix(n)] = [int(v[0]), int(v[1])]
        self.turn = g.get("turn", 1); self.ar = g.get("ar", 0)
        self.me = sid(g.get("me", "US")); self.phasing = sid(g.get("phasing", g.get("me", "US")))
        self.defcon = g.get("defcon", 5); self.vp = g.get("vp", 0)
        self.milops = list(g.get("milops", [0, 0])); self.space = list(g.get("space", [0, 0]))
        self.space_first = [-1] * 9
        for k, s in g.get("space_first", {}).items(): self.space_first[int(k)] = sid(s)
        self.space_attempts = list(g.get("space_attempts", [0, 0]))
        self.china = sid(g.get("china", "USSR")); self.china_up = g.get("china_up", True)
        e = g.get("effects", {})
        self.ev = {f: bool(e.get(f, False)) for f in FLAGS}
        self.red_scare = [SN[s] in e.get("red_scare", []) for s in (0, 1)]   # side whose cards are -1
        self.lads = sid(e["lads"]) if e.get("lads") else None                  # side that played Latin American Death Squads
        self.chernobyl = REGIONS.index(e["chernobyl"]) if e.get("chernobyl") else None
        self.cmc = sid(e["cmc"]) if e.get("cmc") else None                     # side that played Cuban Missile Crisis
        self.hand = list(g.get("hand", [])); self.removed = set(g.get("removed", [])); self.discard = set(g.get("discard", []))
        self.opp_known = list(g.get("opp_known", [])); self.opp_hand_size = g.get("opp_hand_size", None)
        self.rounds_done = list(g.get("rounds_done", [0, 0]))
        self.winner = -1
    def copy(self):
        s = St.__new__(St); s.__dict__.update(self.__dict__)
        s.inf = [x[:] for x in self.inf]; s.milops = self.milops[:]; s.space = self.space[:]; s.space_first = self.space_first[:]
        s.hand = self.hand[:]
        return s
    def ctrl(self, c):
        u, r = self.inf[c]; s = C[c]["stab"]
        return US if (u >= s and u - r >= s) else USSR if (r >= s and r - u >= s) else -1
    def hand_of(self, side): return self.hand if side == self.me else []
    def adj_sp(self, side, c): return C[c]["adj_us"] if side == US else C[c]["adj_ussr"]
    def add_vp(self, side, n):
        if self.winner >= 0: return
        self.vp += n if side == US else -n
        if self.vp >= 20: self.winner = US
        if self.vp <= -20: self.winner = USSR
    def degrade(self):
        self.defcon = max(1, self.defcon - 1)
        if self.defcon <= 1 and self.winner < 0: self.winner = opp(self.phasing)
    def space_ability(self, side, k): return self.space_first[k] == side and self.space[opp(side)] < k
    def has_inf_region(self, side, r): return any(self.inf[c][side] > 0 for c in REGION_C[r])

def card_ops(st, side, k):
    o = CARDS[k]["ops"]
    if CARDS[k]["scoring"]: return 0
    if side == US and st.ev["containment"]: o = min(o + 1, max(4, o))
    if side == USSR and st.ev["brezhnev"]: o = min(o + 1, max(4, o))
    if st.red_scare[side]: o = max(o - 1, 1)
    return o

def rounds_for(st, side):
    base = 6 if st.turn <= 3 else 7
    return base + 1 if (st.space_ability(side, 8) or (side == US and st.ev["nso_turn"])) else base
def rounds_left(st, side): return rounds_for(st, side) - st.rounds_done[side]
def card_live(st, k):
    if k in st.removed: return False
    e = CARDS[k]["era"]
    return e == 0 or (e == 1 and st.turn >= 3) or (e == 2 and st.turn >= 7)

# ---------------------------------------------------------------- scoring
def bg_scoring(st, c): return C[c]["bg"] or (c == TAIWAN and st.ev["formosan"] and st.ctrl(c) == US)
REGION_VALS = [(3, 7, 1000), (3, 7, 9), (3, 5, 7), (1, 4, 6), (1, 3, 5), (2, 5, 6)]
def region_detail(st, r, skip=None):
    n = [0, 0]; b = [0, 0]; nb = [0, 0]; adj = [0, 0]; tb = 0
    for c in REGION_C[r]:
        if c == skip: continue
        isbg = bg_scoring(st, c); tb += isbg
        k = st.ctrl(c)
        if k >= 0:
            n[k] += 1
            if isbg: b[k] += 1
            else: nb[k] += 1
            if st.adj_sp(opp(k), c): adj[k] += 1
    p, d, ctl = REGION_VALS[r]; out = [0, 0]; lvl = ["-", "-"]
    for s in (0, 1):
        o = opp(s)
        if n[s] == 0: continue
        if n[s] > n[o] and b[s] == tb and tb > 0: L, lvl[s] = ctl, "Control"
        elif n[s] > n[o] and b[s] > b[o] and nb[s] >= 1: L, lvl[s] = d, "Domination"
        else: L, lvl[s] = p, "Presence"
        out[s] = L + b[s] + adj[s]
    return out, lvl, n, b, adj
def region_score(st, r, skip=None): return region_detail(st, r, skip)[0]
def sea_score(st):
    out = [0, 0]
    for c in SEA:
        k = st.ctrl(c)
        if k >= 0: out[k] += 2 if c == THAILAND else 1
    return out
def scoring_net(st, k):
    """Net VP (+ = US) the Scoring card would give now (Shuttle Diplomacy applied as the US would choose)."""
    if k == 38: u, s = sea_score(st); return u - s
    r = REGION_OF_SCORING[k]; u, s = region_score(st, r); base = u - s
    if st.ev["shuttle"] and r in (MIDEAST, ASIA):
        best = base
        for c in REGION_C[r]:
            if bg_scoring(st, c) and st.ctrl(c) == USSR:
                u, s = region_score(st, r, c); best = max(best, u - s)
        return best
    return base
def europe_controller(st):
    u, s = region_score(st, EUROPE)
    return US if u >= 1000 else USSR if s >= 1000 else None
def final_projection(st):
    v = 0
    for r in range(6):
        u, s = region_score(st, r); v += max(-30, min(30, u - s))
    return v + (1 if st.china == US else -1)

# ---------------------------------------------------------------- evaluation (port of the simulator's bot::eval)
def danger(k):
    if k in (16, 23, 42, 44, 50, 56): return 3.0
    if k in (7, 14, 17, 28, 30, 33, 55, 58, 59, 61, 62, 82, 87, 90, 95, 98, 99, 107, 4, 5, 21, 25, 26, 27, 29, 63, 65, 67, 71, 73, 74,
             76, 77, 85, 86, 89, 96): return 2.0
    return 1.0
def trap_for(holder, k):
    o = owner(k)
    if o == US: return holder == USSR and k in (4, 5, 26, 67, 85, 89, 96)
    if o == USSR: return holder == US and k in (50, 62, 91)
    return False

def evaluate(w, st, me, parts=None):
    def add(name, x):
        if parts is not None: parts[name] = parts.get(name, 0.0) + x
        return x
    if st.winner >= 0: return 1000.0 if st.winner == me else 0.0 if st.winner == 2 else -1000.0
    o = opp(me); sg = 1.0 if me == US else -1.0; hand = st.hand_of(me)
    v = add("VP", w["vp"] * sg * st.vp)
    sc_in = sum(1 for k in hand if CARDS[k]["scoring"])
    if st.ar > 0 and sc_in > rounds_left(st, me): v += add("Scoring card cannot be played in time", -900.0)
    ctrl = [st.ctrl(c) for c in range(NC)]
    for r in range(6):
        u, s = region_score(st, r)
        if u >= 1000 or s >= 1000:
            mine = (u >= 1000) if me == US else (s >= 1000)
            v += add("Europe Control", w["auto_eu"] if mine else -w["auto_eu"]); continue
        net = sg * (u - s); k = SCORING_OF_REGION[r]
        if k in hand: v += add("held Scoring cards", w["held"] * net)
        elif card_live(st, k): v += add("Region standing (unseen Scoring cards)", w["region_unseen"] * net)
    if 38 not in st.removed:
        u, s = sea_score(st); net = sg * (u - s)
        if 38 in hand: v += add("held Scoring cards", w["held"] * net)
        elif card_live(st, 38): v += add("Region standing (unseen Scoring cards)", w["region_unseen"] * net)
    if w["final_"] != 0 and st.turn >= w["final_from"]:
        v += add("final projection", w["final_"] * sg * final_projection(st) * (0.3 + 0.7 * st.turn / 10.0))
    h3 = 1.0 + w["h3"] if (me == USSR and st.turn <= 3) else 1.0
    def mult(c): return w["europe_mult"] if C[c]["region"] == EUROPE else w["asia_mult"] if C[c]["region"] == ASIA else 1.0
    for c in range(NC):
        if not C[c]["bg"] or ctrl[c] < 0: continue
        v += add("Battlegrounds", (w["bg"] if ctrl[c] == me else -w["bg"]) * mult(c) * h3)
    if w["nc"] != 0:
        for c in range(NC):
            if not C[c]["bg"]: continue
            stb = C[c]["stab"]
            def short(a): x, y = st.inf[c][a], st.inf[c][opp(a)]; return max(stb - x, stb + y - x)
            if ctrl[c] != me and st.inf[c][me] > 0 and short(me) == 1: v += add("near-Control", w["nc"] * w["bg"] * mult(c))
            if ctrl[c] != o and st.inf[c][o] > 0 and short(o) == 1: v += add("near-Control", -w["nc"] * w["bg"] * mult(c))
    if w["access"]:
        for r in range(6):
            if r >= AFRICA and st.turn < 3: continue
            if not st.has_inf_region(me, r): v += add("access", -w["access"])
            if not st.has_inf_region(o, r): v += add("access", 0.5 * w["access"])
    rf = lambda s: st.rounds_done[s] / max(1, rounds_for(st, s))
    v += add("Military Ops", -w["milops"] * max(0, st.defcon - st.milops[me]) * (0.25 + 0.75 * rf(me)))
    v += add("Military Ops", w["milops"] * max(0, st.defcon - st.milops[o]) * (0.25 + 0.75 * rf(o)))
    if w["squeeze"]:
        for r in (EUROPE, ASIA, MIDEAST):
            locked = (r == EUROPE and st.defcon <= 4) or (r == ASIA and st.defcon <= 3) or (r == MIDEAST and st.defcon <= 2)
            if locked:
                u, s = region_score(st, r); x = sg * (u - s); v += add("squeeze", w["squeeze"] * ((x > 0) - (x < 0)))
    tf = {2: 1.0, 3: 0.5, 4: 0.2}.get(st.defcon, 0.1)
    for k in hand:
        if owner(k) == o: v += add("opponent cards held", -w["bad"] * (1.0 + danger(k) / 2.0))
        if trap_for(me, k): v += add("DEFCON trap cards held", -w["trap"] * tf)
    if st.ar > 0:
        traps = sum(1 for k in hand if trap_for(me, k)); others = len(hand) - traps
        forced = min(max(rounds_left(st, me) - others, 0), traps)
        if forced > 0: v += add("DEFCON trap card forced out", -w["trap"] * 3.0 * forced * {2: 1.0, 3: 0.5}.get(st.defcon, 0.15))
    if (me == US and st.ev["quagmire"]) or (me == USSR and st.ev["bear_trap"]):
        v += add("Quagmire/Bear Trap", -w["trapped"] * (1.0 + sc_in))
        elig = sum(1 for k in hand if not CARDS[k]["scoring"] and card_ops(st, me, k) >= 2)
        if st.ar > 0 and sc_in > 0 and sc_in + min(elig, 1) > rounds_left(st, me): v += add("Scoring card trapped", -900.0)
    if (o == US and st.ev["quagmire"]) or (o == USSR and st.ev["bear_trap"]): v += add("Quagmire/Bear Trap", w["trapped"])
    cv = w["china"] * (1.0 if st.china_up else 0.6)
    v += add("China Card", cv if st.china == me else -cv)
    for b in (2, 4, 6, 8):
        if st.space_ability(me, b): v += add("Space Race abilities", w["space_ab"])
        if st.space_ability(o, b): v += add("Space Race abilities", -w["space_ab"])
    if w["inf"]:
        d = sum(st.inf[c][me] - st.inf[c][o] for c in range(NC)); v += add("Influence total", w["inf"] * d)
    return v

# ---------------------------------------------------------------- legality and operations
def defcon_blocks(st, c):
    r = C[c]["region"]
    return (r == EUROPE and st.defcon <= 4) or (r == ASIA and st.defcon <= 3) or (r == MIDEAST and st.defcon <= 2)
def nato_protects(st, side, c):
    return (side == USSR and st.ev["nato"] and C[c]["region"] == EUROPE and st.ctrl(c) == US
            and not (c == FRANCE and st.ev["de_gaulle"]) and not (c == W_GERMANY and st.ev["willy_brandt"]))
def can_coup(st, side, c, free=False):
    if st.inf[c][opp(side)] == 0: return False
    if not free and defcon_blocks(st, c): return False
    if nato_protects(st, side, c): return False
    if side == USSR and c == JAPAN and st.ev["us_japan"]: return False
    if side == USSR and st.ev["reformer"] and C[c]["region"] == EUROPE: return False
    return True
def can_realign(st, side, c, free=False):
    if st.inf[c][opp(side)] == 0: return False
    if not free and defcon_blocks(st, c): return False
    if nato_protects(st, side, c): return False
    if side == USSR and c == JAPAN and st.ev["us_japan"]: return False
    return True
def in_scope(c, scope): return scope == "any" or (scope == "asia" and C[c]["region"] == ASIA) or (scope == "sea" and C[c]["sea"])
def can_place(st, side, c, scope="any", start=None):
    """start: Influence at the start of the Action Round (defaults to the state's)."""
    if not in_scope(c, scope): return False
    if side == USSR and st.chernobyl == C[c]["region"]: return False
    pres = start if start is not None else st.inf
    if pres[c][side] > 0 or st.adj_sp(side, c): return True
    return any(pres[a][side] > 0 for a in C[c]["adj"])
def place_cost(st, side, c): return 2 if st.ctrl(c) == opp(side) else 1
def scope_bonus(st, side, card, scope):
    b = 0
    if card == CHINA and scope in ("asia", "sea"): b += 1
    if side == USSR and st.ev["vietnam_revolts"] and card != 0 and scope == "sea": b += 1
    return b
def coup_result(st, side, c, ops, free, d):
    s = st.copy()
    if st.cmc == opp(side): s.winner = opp(side); return s
    if side == US and st.ev["yuri"]: s.add_vp(USSR, 1)
    m = (-1 if st.ev["salt"] else 0)
    if C[c]["region"] in (CENTAM, SOUTHAM) and st.lads is not None: m += 1 if st.lads == side else -1
    x = d + ops + m - 2 * C[c]["stab"]
    if x > 0:
        o = opp(side); rem = min(x, s.inf[c][o]); s.inf[c][o] -= rem; s.inf[c][side] += x - rem
    if not free: s.milops[side] = min(5, s.milops[side] + ops)
    if C[c]["bg"] and not (side == US and st.ev["nuclear_subs"]): s.degrade()
    return s
def coup_mod(st, side, c):
    m = (-1 if st.ev["salt"] else 0)
    if C[c]["region"] in (CENTAM, SOUTHAM) and st.lads is not None: m += 1 if st.lads == side else -1
    return m
def realign_mod(st, side, c):
    m = sum(1 for a in C[c]["adj"] if st.ctrl(a) == side)
    if st.inf[c][side] > st.inf[c][opp(side)]: m += 1
    if st.adj_sp(side, c): m += 1
    if side == US and st.ev["iran_contra"]: m -= 1
    return m
def realign_outcomes(st, side, c):
    o = opp(side); ma, mb = realign_mod(st, side, c), realign_mod(st, o, c); out = []
    for a in range(1, 7):
        for b in range(1, 7):
            d = (a + ma) - (b + mb); s = st.copy()
            if d > 0: s.inf[c][o] = max(0, s.inf[c][o] - d)
            if d < 0: s.inf[c][side] = max(0, s.inf[c][side] + d)
            out.append(s)
    return out
def space_advance(st, side, n, last_vp_only=False):
    for i in range(n):
        b = st.space[side] + 1
        if b > 8: break
        st.space[side] = b; first = st.space[opp(side)] < b
        if first: st.space_first[b] = side
        if not last_vp_only or i == n - 1:
            v = SPACE[b][2] if first else SPACE[b][3]
            if v > 0: st.add_vp(side, v)
def can_space(st, side, card):
    b = st.space[side] + 1; allowed = 2 if st.space_ability(side, 2) else 1
    return b <= 8 and st.space_attempts[side] < allowed and not CARDS[card]["scoring"] and card_ops(st, side, card) >= SPACE[b][0]

def greedy_place(w, st, me, left, scope):
    s = st.copy(); start = st.inf; placed = []
    while True:
        best = None
        for c in range(NC):
            if not can_place(s, me, c, scope, start): continue
            cost = place_cost(s, me, c)
            if cost > left: continue
            t = s.copy(); t.inf[c][me] += 1; v = evaluate(w, t, me)
            if best is None or v > best[0]: best = (v, c)
        if best is None: break
        c = best[1]; left -= place_cost(s, me, c); s.inf[c][me] += 1; placed.append(C[c]["name"])
        if left <= 0: break
    return s, placed

def best_ops(w, st, me, card, base, coup=True, realign=True, place=True, space=True):
    """Best use of Ops (expected value, description). Same procedure as the simulator's Brain::best_ops."""
    best = (float("-inf"), "none", None)
    if place:
        scopes = ["any"]
        if card == CHINA: scopes.append("asia")
        if card != 0 and ((me == USSR and st.ev["vietnam_revolts"]) or card == CHINA): scopes.append("sea")
        for sc in scopes:
            n = base + scope_bonus(st, me, card, sc); s, placed = greedy_place(w, st, me, n, sc); v = evaluate(w, s, me)
            if v > best[0]:
                from collections import Counter
                best = (v, "place", f"place {n} Ops" + (f" ({sc})" if sc != "any" else "") + ": " + ", ".join(f"{k} +{m}" for k, m in Counter(placed).items()))
    if coup:
        for c in range(NC):
            if not can_coup(st, me, c): continue
            sc = ("sea" if C[c]["sea"] else "asia") if C[c]["region"] == ASIA else "any"
            ops = base + scope_bonus(st, me, card, sc)
            v = sum(evaluate(w, coup_result(st, me, c, ops, False, d), me) for d in range(1, 7)) / 6.0
            if v > best[0]:
                need = 2 * C[c]["stab"] - ops - coup_mod(st, me, c) + 1; p = max(0, min(6, 7 - need)) / 6.0
                best = (v, "coup", f"Coup {C[c]['name']} with {ops} Ops: success {p:.0%}" + (" (Battleground: DEFCON -1)" if C[c]["bg"] and not (me == US and st.ev["nuclear_subs"]) else ""))
    if realign:
        b0 = evaluate(w, st, me); gain, tgt = float("-inf"), None
        for c in range(NC):
            if can_realign(st, me, c):
                g = sum(evaluate(w, s, me) for s in realign_outcomes(st, me, c)) / 36.0 - b0
                if g > gain: gain, tgt = g, c
        if tgt is not None:
            v = b0 + gain * base * 0.8
            if v > best[0]: best = (v, "realign", f"Realignment rolls ({base}), best first target {C[tgt]['name']} (+{gain:.2f} per roll)")
    if space and card != 0 and can_space(st, me, card):
        b = st.space[me] + 1; p = SPACE[b][1] / 6.0; ok = st.copy(); space_advance(ok, me, 1)
        v = p * evaluate(w, ok, me) + (1 - p) * evaluate(w, st, me)
        if v > best[0]: best = (v, "space", f"Space Race to box {b} ({SPACE_NAMES[b]}), roll 1-{SPACE[b][1]}: {p:.0%}")
    if best[0] == float("-inf"): best = (evaluate(w, st, me), "none", "no legal use")
    return best

# ---------------------------------------------------------------- DEFCON hard limits (Twilight Strategy; simulator run 3)
def opp_can_bg_coup(st, me, europe_free=False):
    o = opp(me)
    if st.cmc == me: return False
    if o == US and st.ev["nuclear_subs"]: return False
    return any(C[c]["bg"] and st.inf[c][me] > 0 and (can_coup(st, o, c) or (europe_free and C[c]["region"] == EUROPE and can_coup(st, o, c, True)))
               for c in range(NC))
def out_of_play(st, k): return k in st.removed or k in st.discard or k in st.hand
def forbidden_ar(st, me, k, how):
    """how: 'event' or 'ops'. True = never make this play now (DEFCON 2)."""
    if st.defcon > 2 or k == CHINA: return False
    o = opp(me)
    fires = how == "event" or (how == "ops" and owner(k) == o)
    if not fires: return False
    if k in (4, 50, 89): return True
    if k in (26, 62, 67): return opp_can_bg_coup(st, me)
    if k == 96: return opp_can_bg_coup(st, me, True)
    if k == 91: return me == US and st.inf[CUBA][US] > 0 and st.cmc != US
    if k in (20, 45, 49): return how == "event"
    if k == 5:
        if me == US: return not (out_of_play(st, 4) and out_of_play(st, 89))
        return any(h != 5 and (h in (4, 89) or (h in (26, 67, 96) and opp_can_bg_coup(st, me, h == 96))) for h in st.hand)
    return False
def forbidden_headline(st, me, k):
    if st.defcon < 3 or k == CHINA: return False
    if k not in (4, 50, 89, 26, 62, 67, 96, 20, 45, 49, 5, 91): return False
    o = opp(me); ops = lambda c: 0 if CARDS[c]["scoring"] else CARDS[c]["ops"]; mine = ops(k)
    deg = [46, 40] if st.defcon >= 4 else ([4, 89, 50, 46, 40, 47, 26] if o == US else [50, 4, 89, 46, 40, 47, 62])
    def live(u):
        e = CARDS[u]["era"]
        return not out_of_play(st, u) and (e == 0 or (e == 1 and st.turn >= 4) or (e == 2 and st.turn >= 8))
    return [u for u in deg if live(u) and (ops(u) > mine or (ops(u) == mine and o == US))]
def is_suicide(st, me, k):
    if k == CHINA or CARDS[k]["scoring"]: return False
    s2 = st.copy(); s2.defcon = 2
    own_ok = owner(k) != opp(me) and not forbidden_ar(s2, me, k, "event")
    return forbidden_ar(s2, me, k, "ops") and not own_ok and not can_space(st, me, k)

# ---------------------------------------------------------------- unseen cards
def unseen_pool(st):
    eras = 0 if st.turn <= 3 else 1 if st.turn <= 7 else 2
    return [k for k in CARDS if k != CHINA and CARDS[k]["era"] <= eras and k not in st.removed and k not in st.discard
            and k not in st.hand and k not in st.opp_known]
def p_opp_holds(st):
    pool = unseen_pool(st)
    if st.opp_hand_size is None or not pool: return None
    return max(0, st.opp_hand_size - len(st.opp_known)) / len(pool)

# ---------------------------------------------------------------- commands
import json, sys, random
def load(p): return json.load(open(p))
def save(p, g): json.dump(g, open(p, "w"), indent=1, ensure_ascii=False)
def weights(args):
    w = dict(BASE)
    for a in args:
        if a.startswith("--w="): a = a[4:]
        else: continue
        for kv in a.split(","):
            k, v = kv.split("="); w[k] = float(v)
    return w
def cname(k): return f"#{k} {CARDS[k]['name']}"
def fmt_side_score(u, s): return f"US {u} - USSR {s} -> net {u - s:+d} ({'US' if u > s else 'USSR' if s > u else 'even'})"

def cmd_init(path, side, handicap="2"):
    g = {"turn": 1, "ar": 0, "me": side.upper(), "phasing": "USSR", "defcon": 5, "vp": 0, "milops": [0, 0], "space": [0, 0],
         "space_first": {}, "space_attempts": [0, 0], "china": "USSR", "china_up": True,
         "inf": {"Syria": [0, 1], "Iraq": [0, 1], "N. Korea": [0, 3], "E. Germany": [0, 3], "Finland": [0, 1],
                 "Canada": [2, 0], "Iran": [1, 0], "Israel": [1, 0], "Japan": [1, 0], "Australia": [4, 0], "Philippines": [1, 0],
                 "S. Korea": [1, 0], "Panama": [1, 0], "South Africa": [1, 0], "U.K.": [5, 0]},
         "effects": {}, "hand": [], "opp_hand_size": 8, "opp_known": [], "known_to_opp": [], "removed": [], "discard": [],
         "rounds_done": [0, 0], "handicap_us": int(handicap), "log": []}
    save(path, g)
    print(f"Created {path}. Still to place: USSR 6 anywhere in Eastern Europe (USSR first), then US 7 anywhere in Western Europe,"
          f" then the US handicap of +{handicap} (only where the US has Influence, at most 2 above what Control needs).")
    print("Add Influence with: add GAME COUNTRY SIDE N (N may be negative). Put my dealt cards in \"hand\".")

def cmd_add(path, country, side, n):
    g = load(path); c = C[cix(country)]["name"]; v = g["inf"].get(c, [0, 0]); v[sid(side)] = max(0, v[sid(side)] + int(n))
    g["inf"][c] = v
    if v == [0, 0]: del g["inf"][c]
    save(path, g); st = St(g); k = st.ctrl(cix(c))
    print(f"{c}: US {v[0]} / USSR {v[1]} (Stability {C[cix(c)]['stab']}) -> {'US' if k == 0 else 'USSR' if k == 1 else 'nobody'} Controls")

def region_lines(st, r):
    out, lvl, n, b, adj = region_detail(st, r)
    rows = []
    for c in REGION_C[r]:
        u, s = st.inf[c]; k = st.ctrl(c)
        if u or s or C[c]["bg"]:
            tag = ("*" if C[c]["bg"] else " ") + ("US " if k == 0 else "SU " if k == 1 else "   ")
            rows.append(f"    {tag}{C[c]['name']:<18} {u}/{s} (stab {C[c]['stab']})")
    head = f"  {REGIONS[r]}: {fmt_side_score(*out) if max(out) < 1000 else 'EUROPE CONTROL: automatic win for ' + SN[0 if out[0] >= 1000 else 1]}"
    head += f"   [US {lvl[0]}, {n[0]} countries, {b[0]} BG | USSR {lvl[1]}, {n[1]} countries, {b[1]} BG]"
    return [head] + rows

def cmd_report(path, *args):
    g = load(path); st = St(g); me = st.me; o = opp(me); w = weights(args)
    print(f"Turn {st.turn}, Action Round {st.ar} ({'Headline' if st.ar == 0 else 'phasing ' + SN[st.phasing]}), I am {SN[me]}")
    print(f"VP {st.vp:+d} (+ = US){'  ** within 4 of 20 **' if abs(st.vp) >= 16 else ''}   DEFCON {st.defcon}"
          f" (no Coups/Realignments in: {', '.join(r for r, x in [('Europe', st.defcon <= 4), ('Asia', st.defcon <= 3), ('Middle East', st.defcon <= 2)] if x) or 'none'})")
    print(f"Military Ops: US {st.milops[0]}, USSR {st.milops[1]}; required {st.defcon}. Shortfall now: US {max(0, st.defcon - st.milops[0])}, USSR {max(0, st.defcon - st.milops[1])}")
    print(f"Space Race: US box {st.space[0]} ({SPACE_NAMES[st.space[0]]}), USSR box {st.space[1]} ({SPACE_NAMES[st.space[1]]}); abilities: "
          + (", ".join(f"{SN[s]} box {b}" for s in (0, 1) for b in (2, 4, 6, 8) if st.space_ability(s, b)) or "none"))
    print(f"China Card: {SN[st.china]} ({'face up' if st.china_up else 'face down'})")
    print(f"Rounds: US {st.rounds_done[0]}/{rounds_for(st, 0)}, USSR {st.rounds_done[1]}/{rounds_for(st, 1)}")
    act = [f for f in FLAGS if st.ev[f]] + [f"red_scare on {SN[s]}" for s in (0, 1) if st.red_scare[s]] + \
          ([f"LADS by {SN[st.lads]}"] if st.lads is not None else []) + ([f"Chernobyl: {REGIONS[st.chernobyl]}"] if st.chernobyl is not None else []) + \
          ([f"CUBAN MISSILE CRISIS by {SN[st.cmc]}: {SN[opp(st.cmc)]} loses if it Coups"] if st.cmc is not None else [])
    print("Effects in play:", ", ".join(act) or "none")
    print("\nRegions (scoring if played now; * = Battleground; US/SU = Control):")
    for r in range(6):
        for l in region_lines(st, r): print(l)
    u, s = sea_score(st); print(f"  Southeast Asia (card #38{' removed' if 38 in st.removed else ''}): US {u} - USSR {s} -> net {u - s:+d}")
    fp = final_projection(st); ec = europe_controller(st)
    print(f"\nFinal Scoring projection (incl. China Card): net {fp:+d}" + (f"  ** {SN[ec]} Controls Europe: Europe Scoring or Final Scoring wins **" if ec is not None else ""))
    p = p_opp_holds(st); pool = unseen_pool(st)
    print("\nScoring cards:")
    for k in (2, 1, 3, 79, 37, 81, 38):
        net = scoring_net(st, k); mine = net if me == US else -net
        if k in st.hand: where = "IN MY HAND (must be played this turn)"
        elif k in st.removed: where = "removed"
        elif k in st.discard: where = "discard pile"
        elif k in st.opp_known: where = "opponent holds it (known)"
        elif CARDS[k]["era"] > (0 if st.turn <= 3 else 1 if st.turn <= 7 else 2): where = "not yet in the deck"
        else: where = "unseen" + (f": opponent holds it with p = {p:.0%}" if p is not None else "")
        print(f"  {cname(k):<28} now {mine:+d} for me   {where}")
    print("\nMy hand:")
    for k in sorted(st.hand):
        cd = CARDS[k]; side = ["neutral", "US", "USSR"][cd["side"]]
        flags = []
        if owner(k) == o: flags.append("opponent Event: fires if played for Ops")
        if st.defcon == 2:
            for how in ("event", "ops"):
                if forbidden_ar(st, me, k, how): flags.append(f"FORBIDDEN as {how} at DEFCON 2")
        if is_suicide(st, me, k): flags.append("DEFCON suicide card at DEFCON 2: dispose of it while DEFCON >= 3")
        hl = forbidden_headline(st, me, k) if st.ar == 0 else False
        if hl: flags.append("do not Headline: a degrader may resolve first (" + ", ".join(cname(u) for u in hl) + ")")
        print(f"  {cname(k):<34} {cd['ops']} Ops ({card_ops(st, me, k)} now) {side}{' *' if cd['star'] else ''}" + ("  - " + "; ".join(flags) if flags else ""))
    if st.china == me: print(f"  #6 The China Card (4 Ops, +1 all in Asia){' face down: not playable this turn' if not st.china_up else ''}")
    sui = [k for k in st.hand if is_suicide(st, me, k)]
    if sui and st.defcon >= 3:
        safe = len(st.hand) - len(sui) + (1 if st.china == me and st.china_up else 0)
        if rounds_left(st, me) > safe: print(f"  ** {len(sui)} suicide card(s) and only {safe} safe cards for {rounds_left(st, me)} rounds: play one now while DEFCON >= 3 **")
    sc = sum(1 for k in st.hand if CARDS[k]["scoring"])
    if sc and st.ar > 0 and sc >= rounds_left(st, me): print(f"  ** {sc} Scoring card(s) and {rounds_left(st, me)} rounds left: play them now **")
    print(f"\nUnseen pool: {len(pool)} cards; opponent hand {st.opp_hand_size}")
    print(f"Evaluation (core bot weights): me {evaluate(w, st, me):+.2f}")

def cmd_score(path, region="all"):
    st = St(load(path))
    rs = range(6) if region == "all" else [next(i for i, r in enumerate(REGIONS) if r.lower().startswith(region.lower()))]
    for r in rs:
        for l in region_lines(st, r): print(l)
    if region == "all":
        u, s = sea_score(st); print(f"  Southeast Asia: US {u} - USSR {s} -> net {u - s:+d}"); print(f"Final projection net {final_projection(st):+d}")

def cmd_coup(path, side, country, ops, *args):
    st = St(load(path)); s = sid(side); c = cix(country); ops = int(ops); free = "--free" in args
    print(f"{SN[s]} Coup in {C[c]['name']} (Stability {C[c]['stab']}, Influence US {st.inf[c][0]} / USSR {st.inf[c][1]}) with {ops} Ops")
    if not can_coup(st, s, c, free): print("  NOT ALLOWED (no enemy Influence, DEFCON region limit, NATO, US/Japan Pact or The Reformer)"); return
    m = coup_mod(st, s, c)
    for d in range(1, 7):
        x = d + ops + m - 2 * C[c]["stab"]; r = coup_result(st, s, c, ops, free, d)
        print(f"  roll {d}: {'success ' + str(x) if x > 0 else 'fails'} -> US {r.inf[c][0]} / USSR {r.inf[c][1]}" + (" (WIN for " + SN[r.winner] + ")" if r.winner >= 0 else ""))
    p = sum(1 for d in range(1, 7) if d + ops + m - 2 * C[c]["stab"] > 0) / 6
    print(f"  success {p:.0%}; die modifier {m:+d}; Military Ops +{0 if free else ops};"
          + (" DEFCON -1 (Battleground)" if C[c]["bg"] and not (s == US and st.ev["nuclear_subs"]) else " no DEFCON change")
          + (f"  ** DEFCON {st.defcon} -> {st.defcon - 1}: {'YOU LOSE' if st.defcon == 2 else 'check'} **" if C[c]["bg"] and st.defcon <= 3 and not (s == US and st.ev['nuclear_subs']) else ""))
    if st.cmc == opp(s): print("  ** Cuban Missile Crisis: this Coup loses the game **")

def cmd_realign(path, side, country, *args):
    st = St(load(path)); s = sid(side); c = cix(country)
    if not can_realign(st, s, c, "--free" in args): print("NOT ALLOWED"); return
    ma, mb = realign_mod(st, s, c), realign_mod(st, opp(s), c); dist = {}
    for a in range(1, 7):
        for b in range(1, 7):
            d = (a + ma) - (b + mb); dist[d] = dist.get(d, 0) + 1
    win = sum(v for d, v in dist.items() if d > 0) / 36; lose = sum(v for d, v in dist.items() if d < 0) / 36
    exp_rem = sum(min(d, st.inf[c][opp(s)]) * v for d, v in dist.items() if d > 0) / 36
    exp_lost = sum(min(-d, st.inf[c][s]) * v for d, v in dist.items() if d < 0) / 36
    print(f"{SN[s]} Realignment in {C[c]['name']}: modifiers {SN[s]} {ma:+d}, {SN[opp(s)]} {mb:+d}")
    print(f"  I remove Influence {win:.0%} of the time (expected {exp_rem:.2f}); I lose Influence {lose:.0%} (expected {exp_lost:.2f}); tie {1 - win - lose:.0%}")

def cmd_place(path, side, spec, *args):
    """spec: 'Italy,Italy,France' (one entry per marker). --apply writes it."""
    g = load(path); st = St(g); s = sid(side); start = [x[:] for x in st.inf]; cost = 0; ok = True
    for name in [x for x in spec.split(",") if x.strip()]:
        c = cix(name)
        if not can_place(st, s, c, "any", start): print(f"  {C[c]['name']}: NOT ALLOWED (no own Influence there or adjacent at the start of the round{', Chernobyl' if st.chernobyl == C[c]['region'] and s == USSR else ''})"); ok = False; continue
        k = place_cost(st, s, c); cost += k; st.inf[c][s] += 1
        print(f"  {C[c]['name']}: +1 for {k} Ops -> US {st.inf[c][0]} / USSR {st.inf[c][1]}")
    print(f"Total cost {cost} Ops" + ("" if ok else " (some markers not allowed)"))
    if "--apply" in args and ok:
        g["inf"] = {C[c]["name"]: st.inf[c] for c in range(NC) if st.inf[c] != [0, 0]}; save(path, g); print("applied")

def cmd_eval(path, *args):
    g = load(path); st = St(g); side = sid(args[0]) if args and not args[0].startswith("--") else st.me; w = weights(args)
    parts = {}; v = evaluate(w, st, side, parts)
    print(f"Evaluation for {SN[side]}: {v:+.2f}")
    for k, x in sorted(parts.items(), key=lambda t: -abs(t[1])):
        if abs(x) > 1e-9: print(f"  {k:<42} {x:+.2f}")

def apply_changes(st, ch):
    for n, d in ch.get("inf", {}).items():
        c = cix(n); st.inf[c] = [max(0, st.inf[c][0] + d[0]), max(0, st.inf[c][1] + d[1])]
    if "vp" in ch: st.vp += ch["vp"]; st.winner = US if st.vp >= 20 else USSR if st.vp <= -20 else st.winner
    if "defcon" in ch: st.defcon = max(1, min(5, st.defcon + ch["defcon"])); st.winner = opp(st.phasing) if st.defcon <= 1 else st.winner
    for i, d in enumerate(ch.get("milops", [0, 0])): st.milops[i] = min(5, max(0, st.milops[i] + d))
    for i, d in enumerate(ch.get("space", [0, 0])):
        if d: space_advance(st, i, d)
    for k in ch.get("play", []):
        if k in st.hand: st.hand.remove(k)
        if CARDS[k]["star"]: st.removed.add(k)
    for k in ch.get("removed", []): st.removed.add(k)
    for f, v in ch.get("effects", {}).items():
        if f in st.ev: st.ev[f] = bool(v)
    if "china" in ch: st.china = sid(ch["china"]); st.china_up = ch.get("china_up", False)
    return st

def cmd_whatif(path, side, changes, *args):
    g = load(path); st = St(g); s = sid(side); w = weights(args); ch = json.loads(changes)
    b = evaluate(w, st, s); st2 = apply_changes(st.copy(), ch); a = evaluate(w, st2, s)
    print(f"{SN[s]} evaluation: {b:+.2f} -> {a:+.2f}  (change {a - b:+.2f})")
    for r in range(6):
        x, y = region_score(st, r), region_score(st2, r)
        if x != y: print(f"  {REGIONS[r]}: {fmt_side_score(*x)}  ->  {fmt_side_score(*y)}")
    if final_projection(st) != final_projection(st2): print(f"  final projection {final_projection(st):+d} -> {final_projection(st2):+d}")

def cmd_decide(path, *args):
    g = load(path); st = St(g); me = st.me; o = opp(me); w = weights(args); v0 = evaluate(w, st, me); rows = []
    cards = list(st.hand) + ([CHINA] if st.china == me and st.china_up else [])
    for k in cards:
        cd = CARDS[k]
        if cd["scoring"]:
            net = scoring_net(st, k); mine = net if me == US else -net
            t = st.copy(); t.hand.remove(k); t.vp += net
            if k == 2 and europe_controller(st) is not None: rows.append((1e6 if europe_controller(st) == me else -1e6, k, "Europe Scoring: automatic win" if europe_controller(st) == me else "Europe Scoring: LOSES (opponent Controls Europe)")); continue
            rows.append((evaluate(w, t, me) - v0, k, f"score now: {mine:+d} VP for me"))
            continue
        base = card_ops(st, me, k) if k != CHINA else 4 + (1 if me == US and st.ev["containment"] else 0) + (1 if me == USSR and st.ev["brezhnev"] else 0) - (1 if st.red_scare[me] else 0)
        t = st.copy()
        if k in t.hand: t.hand.remove(k)
        v, kind, desc = best_ops(w, t, me, k, base)
        notes = []
        if owner(k) == o: notes.append("opponent Event fires: subtract its value (whatif)")
        if owner(k) != o and not cd["scoring"] and k != CHINA: notes.append("Event not valued here: compare with whatif")
        if st.defcon == 2 and forbidden_ar(st, me, k, "ops"): notes.append("FORBIDDEN for Ops at DEFCON 2")
        if st.defcon == 2 and owner(k) != o and forbidden_ar(st, me, k, "event"): notes.append("Event FORBIDDEN at DEFCON 2")
        if "DEFCON -1" in desc and st.defcon == 2: notes.append("COUP LOSES THE GAME")
        rows.append((v - v0, k, desc + ("  [" + "; ".join(notes) + "]" if notes else "")))
    print(f"{SN[me]} options (Ops only, one move, no opponent reply; value change vs now {v0:+.2f}):")
    for d, k, desc in sorted(rows, key=lambda t: -t[0]): print(f"  {d:+7.2f}  {cname(k):<34} {desc}")

def cmd_limits(path):
    st = St(load(path)); me = st.me
    print(f"DEFCON {st.defcon}. Hard limits for {SN[me]}:")
    for k in sorted(st.hand):
        out = []
        for how in ("event", "ops"):
            if forbidden_ar(st, me, k, how): out.append(f"no {how} now")
        if is_suicide(st, me, k): out.append("suicide card at DEFCON 2")
        if st.ar == 0:
            hl = forbidden_headline(st, me, k)
            if hl: out.append("no Headline (" + ", ".join(cname(u) for u in hl) + " may resolve first)")
        if out: print(f"  {cname(k)}: " + "; ".join(out))

def cmd_roll(n="1"):
    r = [random.randint(1, 6) for _ in range(int(n))]; print("roll:", " ".join(map(str, r)))

def cmd_xcheck(path):
    """Compare with the simulator: lines from `tssim xcheck N seed`."""
    bad = 0; n = 0
    for line in open(path):
        d = json.loads(line); st = St(d["state"]); me = st.me; n += 1; errs = []
        if [st.ctrl(c) for c in range(NC)] != d["ctrl"]: errs.append("ctrl")
        if [region_score(st, r) for r in range(6)] != d["regions"]: errs.append("regions")
        if sea_score(st) != d["sea"]: errs.append("sea")
        for k, v in d["nets"].items():
            if scoring_net(st, int(k)) != v: errs.append(f"net{k}")
        if final_projection(st) != d["final"]: errs.append("final")
        if [rounds_for(st, 0), rounds_for(st, 1)] != d["rounds_for"]: errs.append("rounds_for")
        for s in (0, 1):
            if abs(evaluate(BASE, st, s) - d["eval"][s]) > 1e-4: errs.append(f"eval{s}:{evaluate(BASE, st, s):.4f}/{d['eval'][s]:.4f}")
        for b in d["best_ops"]:
            t = st.copy(); v, kind, desc = best_ops(BASE, t, me, b["card"], b["ops"])
            ch = b["choice"].split(":")[0]
            if abs(v - b["value"]) > 1e-4 or kind != ch: errs.append(f"ops#{b['card']}:{kind} {v:.4f}/{ch} {b['value']:.4f}")
        if errs: bad += 1; print(d["seed"], errs[:6])
    print(f"{n} positions, {bad} with differences")

if __name__ == "__main__":
    a = sys.argv[1:]
    cmds = {"init": cmd_init, "add": cmd_add, "report": cmd_report, "score": cmd_score, "coup": cmd_coup, "realign": cmd_realign,
            "place": cmd_place, "eval": cmd_eval, "whatif": cmd_whatif, "decide": cmd_decide, "limits": cmd_limits, "roll": cmd_roll,
            "xcheck": cmd_xcheck}
    if not a or a[0] not in cmds: print(__doc__); sys.exit(0)
    cmds[a[0]](*a[1:])
