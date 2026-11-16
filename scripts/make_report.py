"""Build the Stage 2 run 1 test report (PDF) from results/run1/*.jsonl and a text file of sections.
Usage: make_report.py results/run1 reports/text_run1.py reports/out.pdf"""
import json, os, sys, runpy
from reportlab.lib.pagesizes import A4
from reportlab.lib.units import cm
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, KeepTogether

rdir, textfile, out = sys.argv[1:4]
T = runpy.run_path(textfile)
ENDS = ["none", "20 VP", "Europe Control", "DEFCON 1", "held Scoring", "Final Scoring", "Wargames", "Cuban Missile Crisis"]

def load(name):
    p = os.path.join(rdir, name)
    return [json.loads(l) for l in open(p)] if os.path.exists(p) else []

ss = getSampleStyleSheet()
body = ParagraphStyle('b', parent=ss['BodyText'], fontSize=9.5, leading=12.5, spaceAfter=5)
small = ParagraphStyle('s', parent=body, fontSize=8, leading=10)
h1 = ParagraphStyle('h1', parent=ss['Heading1'], fontSize=16, spaceAfter=6)
h2 = ParagraphStyle('h2', parent=ss['Heading2'], fontSize=12.5, spaceBefore=10, spaceAfter=4)
cell = ParagraphStyle('c', parent=body, fontSize=8, leading=10, spaceAfter=0)

def P(t, st=body): return Paragraph(t, st)

def table(rows, widths, head=True):
    data = [[P(str(x), cell) for x in r] for r in rows]
    t = Table(data, colWidths=widths, repeatRows=1 if head else 0)
    t.setStyle(TableStyle([
        ('GRID', (0, 0), (-1, -1), 0.3, colors.grey),
        ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#e8ecf2') if head else colors.white),
        ('VALIGN', (0, 0), (-1, -1), 'TOP'),
        ('LEFTPADDING', (0, 0), (-1, -1), 3), ('RIGHTPADDING', (0, 0), (-1, -1), 3)]))
    return t

def result_rows(recs, verdicts):
    rows = [["Test", "Change (vs full bot unless stated)", "Score ± margin", "As US", "As USSR", "Verdict"]]
    for r in recs:
        ref = '' if r['b'] == 'core' else ' (vs ' + r['b'].replace('core:', '') + ')'
        rows.append([r['label'], r['a'].replace('core:', '').replace('core', 'full bot') + ref,
                     f"{r['score']*100:.1f} ± {r['margin']*100:.1f}", f"{r['a_as_us']*100:.1f}", f"{r['a_as_ussr']*100:.1f}",
                     verdicts.get(r['label'], '')])
    return rows

W = A4[0] - 3 * cm
story = [P(T['TITLE'], h1), P(T['SUBTITLE'], small), Spacer(1, 6)]
for sec in T['SECTIONS']:
    kind = sec[0]
    if kind == 'h': story.append(P(sec[1], h2))
    elif kind == 'p': story.append(P(sec[1]))
    elif kind == 'table': story.append(table(sec[1], [w * W for w in sec[2]])); story.append(Spacer(1, 6))
    elif kind == 'results':
        recs = [r for f in sec[1] for r in load(f)]
        recs = [r for r in recs if r['label'] in sec[2]] if sec[2] else recs
        story.append(table(result_rows(recs, T['VERDICTS']), [w * W for w in (0.2, 0.22, 0.13, 0.08, 0.09, 0.28)]))
        story.append(Spacer(1, 6))
    elif kind == 'ends':
        recs = [r for f in sec[1] for r in load(f) if r['label'] in sec[2]]
        rows = [["Test"] + [e for e in ENDS[1:] if e != 'Cuban Missile Crisis'] + ["games"]]
        for r in recs:
            n = sum(r['ends'])
            rows.append([r['label']] + [f"{r['ends'][i]*100/n:.0f}%" for i in range(1, 7)] + [n])
        story.append(table(rows, [W * 0.22] + [W * 0.11] * 6 + [W * 0.12])); story.append(Spacer(1, 6))

def footer(c, d):
    c.saveState(); c.setFont('Helvetica', 7.5)
    c.drawString(1.5 * cm, 1 * cm, T['FOOTER']); c.drawRightString(A4[0] - 1.5 * cm, 1 * cm, f"page {d.page}")
    c.restoreState()

doc = SimpleDocTemplate(out, pagesize=A4, leftMargin=1.5 * cm, rightMargin=1.5 * cm, topMargin=1.5 * cm, bottomMargin=1.6 * cm,
                        title=T['TITLE'], author='Claude for Alfy')
doc.build(story, onFirstPage=footer, onLaterPages=footer)
print('wrote', out)
