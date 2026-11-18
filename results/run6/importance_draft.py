"""Draft country-importance table for the distance-to-Control term (run 6). Formula + named adjustments."""
import json, csv
m = json.load(open('data/map.json'))['countries']
REGW = {'Europe': 1.5, 'Asia': 1.4, 'Middle East': 1.2, 'Africa': 1.2, 'South America': 1.2, 'Central America': 1.0}
SEA = {'Burma', 'Laos/Cambodia', 'Vietnam', 'Malaysia', 'Indonesia', 'Philippines', 'Thailand'}
OPEC = {'Egypt', 'Iran', 'Libya', 'Saudi Arabia', 'Iraq', 'Gulf States', 'Venezuela'}
# expert adjustments (Twilight Strategy region articles, 2011-12; my reading), with reasons
ADJ = {
 'Thailand': (0.3, 'decides Southeast Asia Scoring (2 VP) as well as Asia'),
 'Iran': (0.1, 'key opening target; links the Middle East to Asia (Afghanistan, Pakistan)'),
 'Egypt': (0.1, 'only Early War path to Libya; Sadat/Nasser swing it'),
 'Algeria': (0.1, 'hardest African Battleground to flip; linked to France'),
 'Greece': (0.2, '2-stability Mediterranean country: decides Europe Domination with the odd Battleground count'),
 'Turkey': (0.2, '2-stability Mediterranean country; US Influence there can cancel the Cuban Missile Crisis'),
 'Spain/Portugal': (0.2, '2-stability Mediterranean country: decides Europe Domination'),
 'Jordan': (0.15, 'the usual US route into Iraq and Saudi Arabia; blunts Arab-Israeli War'),
 'Lebanon': (0.15, 'cheap US Presence in the Middle East; blunts Arab-Israeli War'),
 'Botswana': (0.25, 'the Realignment hub against Angola and South Africa'),
 'Laos/Cambodia': (0.1, 'blocks access between Southeast Asia and western Asia'),
 'Burma': (0.1, 'blocks access to India; Indo-Pakistani War modifier'),
 'Malaysia': (0.0, 'stepping stone to Thailand'),
 'Colombia': (0.1, 'base for realigning Venezuela'),
 'Uruguay': (0.1, 'Realignment support against Argentina and Brazil'),
 'Peru': (0.1, 'Realignment support against Chile'),
 'Costa Rica': (0.15, 'the US base protecting Panama (stability 3)'),
 'Morocco': (0.1, 'the only African non-Battleground with real stability (3): a safe country for Domination'),
 'Tunisia': (0.0, 'the other route into Libya'),
 'Taiwan': (0.1, 'Formosan Resolution can make it a Battleground'),
 'Venezuela': (0.1, 'most contested South American country (OPEC, hard to support from Colombia)'),
 'W. Germany': (0.1, 'Blockade target; Willy Brandt; Tear Down This Wall'),
 'Italy': (0.0, ''), 'France': (0.1, 'decides Europe Domination; gateway to Algeria'),
}
rows = []
for n, c in m.items():
    reg = c['region']; adj = [a for a in c['adj'] if a in m]; sp = [a for a in c['adj'] if a not in m]
    bgn = [a for a in adj if m[a]['bg']]; xr = [a for a in adj if m[a]['region'] != reg]
    why = []
    if c['bg']:
        v = REGW[reg]; why.append(f'Battleground, {reg} ({REGW[reg]})')
    else:
        v = 0.3 + 0.15 * min(3, len(bgn)); why.append('base 0.3' + (f' + 0.15 x {min(3,len(bgn))} Battleground neighbours' if bgn else ''))
        if xr: v += 0.2; why.append('link to another Region: ' + ', '.join(xr) + ' (+0.2)')
        if n in SEA: v += 0.1; why.append('Southeast Asia Scoring (+0.1)')
    if sp:
        v += 0.2; why.append(('next to the USA: +1 VP for the USSR at each scoring if it Controls it' if sp[0]=='U.S.A.' else 'next to the USSR: +1 VP for the US at each scoring if it Controls it') + ' (+0.2)')
    if n in OPEC and not c['bg']: v += 0.1; why.append('OPEC (+0.1)')
    if n in ADJ and ADJ[n][0]:
        v += ADJ[n][0]; why.append(f'{ADJ[n][1]} (+{ADJ[n][0]})')
    elif n in ADJ and ADJ[n][1]: why.append(ADJ[n][1])
    rows.append(dict(country=n, region=reg, stab=c['stab'], bg='yes' if c['bg'] else '', imp=round(v, 2), why='; '.join(why)))
order = list(REGW)
rows.sort(key=lambda r: (order.index(r['region']), -r['imp'], r['country']))
with open('results/run6/importance_draft.csv', 'w', newline='') as f:
    w = csv.DictWriter(f, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
for r in rows: print(f"{r['region'][:8]:8} {r['country']:20} {r['stab']} {r['bg']:3} {r['imp']:.2f}  {r['why'][:110]}")
