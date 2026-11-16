"""Build map.json from the VASSAL module (buildFile.xml + Influence.java) and cross-check with vrazix board.py."""
import re, json, sys, ast
import xml.etree.ElementTree as ET
V = '/home/claude/ts/vassal'
java = open(V + '/src/main/java/ca/mkiefte/Influence.java').read()
consts = dict(re.findall(r'static final String ([A-Z_]+) = "([^"]+)";', java))
consts['CHINA'] = 'China'
chains_src = java[java.index('CHAINS = {'):java.index('};', java.index('CHAINS = {'))]
chains = [[consts[t.strip()] for t in ch.split(',') if t.strip()] for ch in re.findall(r'\{([A-Z_,\s]+)\}', chains_src)]
adj = {}
for ch in chains:
    for a, b in zip(ch, ch[1:]):
        if 'China' in (a, b): continue
        adj.setdefault(a, set()).add(b); adj.setdefault(b, set()).add(a)
def setof(name):
    m = re.search(name + r' = new HashSet<String>\(Arrays.asList\(new String\[\] \{(.*?)\}', java, re.S)
    return {consts[t.strip()] for t in m.group(1).split(',') if t.strip()}
EE, WE, SEA = setof('EASTERN_EUROPE'), setof('WESTERN_EUROPE'), setof('SOUTH_EAST_ASIA')
rows = {}
t = ET.parse(V + '/buildFile.xml').getroot()
for st in t.iter('VASSAL.build.module.map.SetupStack'):
    nm = st.get('name', '')
    if '/' not in nm: continue
    name = nm.rsplit('/', 1)[0]
    for ps in st.iter('VASSAL.build.widget.PieceSlot'):
        m = re.search(r'influence;(\d+);([^;\t]+);(true|false)', ps.text or '')
        if m and name != 'Eqypt':
            rows[name] = dict(stab=int(m.group(1)), region=m.group(2), bg=m.group(3) == 'true', xy=[int(st.get('x')), int(st.get('y'))])
out = {}
for n, r in rows.items():
    sub = []
    if n in EE: sub.append('Eastern Europe')
    if n in WE: sub.append('Western Europe')
    if n in SEA: sub.append('Southeast Asia')
    out[n] = dict(region=r['region'], sub=sub, stab=r['stab'], bg=r['bg'], adj=sorted(adj.get(n, [])), xy=r['xy'])
missing = [n for n in out if not out[n]['adj']]
extra = [n for n in adj if n not in out and n not in ('U.S.A.', 'U.S.S.R.')]
print('countries', len(out), 'no-adjacency', missing, 'adj-only names', extra)
for sp in ('U.S.A.', 'U.S.S.R.'): print(sp, sorted(adj[sp]))
json.dump(dict(countries=out, superpowers={sp: sorted(adj[sp]) for sp in ('U.S.A.', 'U.S.S.R.')}), open(sys.argv[1], 'w'), indent=1)
# ---- cross-check with vrazix board.py (second, independent source)
src = open('/home/claude/ts/src/vrazix/repo/board.py').read()
alias = {'UK': 'U.K.', 'West Germany': 'W. Germany', 'East Germany': 'E. Germany', 'Afganistan': 'Afghanistan', 'Phillipines': 'Philippines',
         'South Korea': 'S. Korea', 'North Korea': 'N. Korea', 'Dominican Republic': 'Dominican Rep', 'Columbia': 'Colombia', 'MAP_US_SP': 'U.S.A.', 'MAP_USSR_SP': 'U.S.S.R.'}
A = lambda x: alias.get(x, x)
vr = {}
for m in re.finditer(r"Country\('([^']+)', '(\w+)', (\(?[^\[]*?\)?), \[([^\]]*)\], (\d), (\d), (\d)(, True)?\)", src):
    name, reg, subs, nb, stab, us, su, bg = m.groups()
    nbs = [A(x.strip().strip("'")) for x in nb.split(',') if x.strip()]
    vr[A(name)] = dict(stab=int(stab), bg=bool(bg), adj=sorted(nbs), us=int(us), su=int(su), subs=subs)
print('vrazix countries', len(vr))
diff = 0
for n in sorted(set(out) | set(vr)):
    if n not in out or n not in vr: print('ONLY IN ONE:', n, n in out, n in vr); diff += 1; continue
    a, b = out[n], vr[n]
    for k in ('stab', 'bg', 'adj'):
        if a[k] != b[k]: print(f'DIFF {n} {k}: vassal={a[k]} vrazix={b[k]}'); diff += 1
print('differences', diff)
