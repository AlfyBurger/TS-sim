"""Build cards.json from three sources: VASSAL module card types, Twilight Strategy card list, vrazix cards.py. Cross-check."""
import re, json, html, sys
V = '/home/claude/ts/vassal'
s = open(V + '/buildFile.xml', encoding='utf-8').read()
vas = {}
for m in re.finditer(r'entryName="([^"]+)"[^>]*>\+/null/(\w+);(\d+);(true|false);(\d);(\d);(-?\d);(true|false);(true|false)', s):
    name, cls, num, opt, war, ops, side, rem, noth = m.groups(); name = html.unescape(name)
    num = int(num)
    if num in vas: continue
    vas[num] = dict(name=name, optional=opt == 'true', era=['Early', 'Mid', 'Late'][int(war)], ops=int(ops),
                    side=['Both', 'US', 'USSR'][int(side)] if int(side) >= 0 else 'Both', star=rem == 'true', may_not_hold=noth == 'true')
print('vassal cards', len(vas))
tw = {int(r[0]): dict(title=r[1], ops=int(r[2]) if r[2].isdigit() else 0, text=r[3]) for r in json.load(open('/home/claude/ts/src/twistrat/cards_twistrat.json'))}
vsrc = open('/home/claude/ts/src/vrazix/repo/cards.py', encoding='utf-8').read()
vr = []
for m in re.finditer(r"Event\('((?:[^'\\]|\\.)+)', \w+, (\d), '(\w+)', '((?:[^'\\]|\\.)*)'(, True)?\)", vsrc):
    vr.append(dict(title=m.group(1), ops=int(m.group(2)), side={'us': 'US', 'ussr': 'USSR', 'neutral': 'Both'}[m.group(3)], text=m.group(4), star=bool(m.group(5))))
for m in re.finditer(r"ScoringCard\('([^']+)'", vsrc): pass
print('vrazix events', len(vr))
norm = lambda t: re.sub(r'[^a-z0-9]', '', t.lower().replace('*', ''))
vr_by = {norm(e['title']): e for e in vr}
svg = {int(k): v for k, v in json.load(open('/home/claude/ts/work/svgcards.json')).items()}
underlined_vassal = sorted(int(re.search(r'Event(\d+)', f).group(1)) for f in __import__('os').listdir(V + '/images') if re.match(r'Event\d+\.svg', f))
cards, issues = {}, []
for n in range(1, 111):
    a = vas.get(n); b = tw.get(n)
    if not a or not b: issues.append(f'{n}: missing in vassal={not a} twistrat={not b}'); continue
    title = b['title']
    c = vr_by.get(norm(title))
    rec = dict(id=n, title=title.rstrip('*').strip(), ops=a['ops'], era=a['era'], side=a['side'], star=a['star'],
               scoring=a['ops'] == 0 and 'Scoring' in title, optional_in_deluxe=a['optional'],
               underlined=n in underlined_vassal, text=b['text'])
    if a['ops'] != b['ops']: issues.append(f'{n} {title}: ops vassal {a["ops"]} vs twistrat {b["ops"]}')
    if a['star'] != title.endswith('*'): issues.append(f'{n} {title}: star vassal {a["star"]} vs twistrat title')
    if c:
        if c['ops'] != a['ops']: issues.append(f'{n} {title}: ops vrazix {c["ops"]} vs vassal {a["ops"]}')
        if c['side'] != a['side']: issues.append(f'{n} {title}: side vrazix {c["side"]} vs vassal {a["side"]}')
        if c['star'] != a['star']: issues.append(f'{n} {title}: star vrazix {c["star"]} vs vassal {a["star"]}')
        if norm(c['text']) != norm(b['text']): rec['text_vrazix_differs'] = c['text']
    elif not rec['scoring']:
        issues.append(f'{n} {title}: not found in vrazix')
    rec['text_vassal_svg'] = ' | '.join(x for x in svg.get(n, []) if norm(x) != norm(title))
    cards[n] = rec
print('cards', len(cards))
for i in issues: print('ISSUE', i)
from collections import Counter
print('era', Counter(c['era'] for c in cards.values()))
print('side', Counter(c['side'] for c in cards.values()))
print('star', sum(c['star'] for c in cards.values()), 'underlined', underlined_vassal)
json.dump(cards, open(sys.argv[1], 'w'), indent=1, ensure_ascii=False)
