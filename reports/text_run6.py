import json, os
_R = 'results/run6'
_res = {json.loads(l)['label']: json.loads(l) for l in open(os.path.join(_R, 'results.jsonl'))}
_reg = json.load(open(os.path.join(_R, 'region_core.json')))

TITLE = "Twilight Struggle simulator — Stage 2, test run 6"
SUBTITLE = ("For Alfy · 19 November 2026 · repository AlfyBurger/TS-sim · 2026 Hall of Fame rules, base game, fixed +2 US handicap · "
            "run on Alfy's PC from 18 Nov 12:09 to 19 Nov 07:46 (Amman time)")
FOOTER = "TS-sim · Stage 2 run 6 test report"
REF_NAME = "run 5 bot"
VERDICTS = {}


def _s(lab): r = _res[lab]; return f"{r['score']*100:.1f} ± {r['margin']*100:.1f}"
def _u(lab): return f"{_res[lab]['a_as_us']*100:.1f}"
def _x(lab): return f"{_res[lab]['a_as_ussr']*100:.1f}"


def _rows(items):
    rows = [["Test", "Change (against the run 5 bot)", "Score ± margin", "As US", "As USSR", "Verdict"]]
    for lab, what, verdict in items: rows.append([lab, what, _s(lab), _u(lab), _x(lab), verdict])
    return rows


_W = (0.13, 0.33, 0.13, 0.08, 0.09, 0.24)

_first = _rows([
    ("NE1", "distance, strength 1, on top of near-Control", "Far worse"),
    ("NE2", "same, strength 2", "Far worse"),
    ("NE3", "same, strength 3", "Far worse"),
    ("NE1R", "strength 1, replacing near-Control", "Far worse"),
    ("NE2R", "strength 2, replacing near-Control", "Far worse"),
    ("NE3R", "strength 3, replacing near-Control", "Far worse"),
    ("FLAT2", "flat importance, strength 2, on top", "Far worse"),
    ("FLAT2R", "flat importance, strength 2, replacing", "Far worse"),
])
_second = _rows([
    ("V2_05", "corrected form, strength 0.5, on top of near-Control", "Better (screening)"),
    ("V2_1", "corrected form, strength 1, on top of near-Control", "<b>Best in screening</b>"),
    ("V2_05R", "strength 0.5, replacing near-Control", "Far worse"),
    ("V2_1R", "strength 1, replacing near-Control", "Far worse"),
    ("V2_2R", "strength 2, replacing near-Control", "Far worse"),
    ("V2F_1R", "flat importance, strength 1, replacing", "Far worse"),
    ("CONFIRM_V2", "V2_1 on 1,000 fresh deals", "<b>Gain: adopted</b>"),
])
_other = _rows([
    ("HL_USSR", "Headline v2 for the USSR", "Neutral: not adopted"),
    ("SS_02", "set-up values for preparation cards, weight 0.2", "Neutral: not adopted"),
    ("SS_04", "same, weight 0.4", "Neutral: not adopted"),
])

_ERAS = ['Early (1-3)', 'Mid (4-7)', 'Late (8-10)']
_ORDER = ['Europe', 'Asia', 'Southeast Asia', 'Middle East', 'Africa', 'Central America', 'South America']


def _cell(st):
    if not st or not st.get('n'): return "–"
    m = round(st['mean'], 1) + 0.0
    return f"{m:+.1f}<br/>({st['us']*100:.0f} / {st['ussr']*100:.0f})"


_region = [["Region", "Early War (turns 1–3)", "Mid War (4–7)", "Late War (8–10)", "Final Scoring"]]
for r in _ORDER:
    _region.append([r] + [_cell(_reg['scoring'].get(f'{r}|{e}')) for e in _ERAS] + [_cell(_reg['final'].get(r))])

SECTIONS = [
    ('h', "1. Summary"),
    ('p', "Run 6 tested one main idea and two smaller ones against the run 5 bot. The main idea, <b>distance to Control</b>, comes from the Red Sea "
          "work: besides the countries each side already Controls, the bot also values how close each side is to Controlling every other country, "
          "using your importance table. The smaller ideas were a Headline rule for the USSR and <b>set-up values</b> for preparation cards."),
    ('p', f"<b>Main findings.</b> (1) <b>Distance to Control is a real gain, and is now the default</b>: {_s('CONFIRM_V2')} against the run 5 bot on "
          "1,000 fresh deals (about +65 rating points), the largest step since run 4. It works only in the corrected form (a country already Controlled "
          "counts in full), only <b>on top of</b> the existing near-Control credit, and only with your importance table: all other versions were far worse. "
          f"(2) The <b>first form failed badly</b> (0–35%), for a reason explained in section 3. (3) <b>Headline v2 for the USSR</b> ({_s('HL_USSR')}) and the "
          f"<b>set-up values</b> ({_s('SS_02')} and {_s('SS_04')}) made no difference and are not adopted. (4) The <b>Region record</b> you asked for "
          "is in section 6: in self-play the USSR does better in Europe (Early and Mid War) and the Middle East; the US in Asia from Mid War on, South America and Africa."),

    ('h', "2. How the run went"),
    ('p', "240 blocks of 50 deals, each deal played from both sides, on your PC with the compiled simulator. Two interruptions cost time but no results:"),
    ('p', "• <b>Idle 16:06–18:39.</b> When the second part was added, the new program was started while the old one was still finishing. The old "
          "wrapper, on exit, leaves a STOP file; the new one read it and stopped at once, so the PC sat idle for 2.5 hours until the next check."),
    ('p', "• <b>Stopped at about 20:30.</b> The run had been started from Claude's own shell on the PC. When the chat was reopened after a long pause, "
          "the Claude app reconnected and its shell's processes were closed, taking the run with them. It was restarted at 20:31 through Windows Task "
          "Scheduler, which keeps it independent of Claude, and a watchdog task was added to restart it automatically if it ever died again. It was not needed."),
    ('p', "From about 23:00 the Red Sea v8 run shared the PC, so the last blocks took about twice as long; the run finished at 07:46 on 19 November."),

    ('h', "3. Distance to Control: the first form, and why it failed"),
    ('p', "The term adds, for every country, its importance (attack and defend values from your table) times a factor that shrinks with the number "
          "of Operations points a side still needs to Control it: 0.5 per missing point (0.4 for attacks in Europe, so that \"cheap chances count, "
          "long pushes little\"). The count includes the extra cost of breaking the other side's Control, and one more point when the side has no "
          "Influence in or next to the country."),
    ('table', _first, _W),
    ('p', "<b>Why it failed.</b> The first form was copied from Red Sea, where a Controlled country earns nothing in this term because Red Sea's "
          "evaluation already rewards Control everywhere else. In Twilight Struggle the bot only rewards Control of Battlegrounds. So in this form, "
          "taking Control of a non-Battleground <b>lost</b> the bot its credit for being close, and the opponent's Control of a country looked like a "
          "<b>good</b> thing (their distance credit vanished). The stronger the term, the worse the bot played: it preferred to stay one point short. "
          "The fix (\"corrected form\") gives a Controlled country its full credit, as if the distance were zero."),

    ('h', "4. The corrected form"),
    ('table', _second, _W),
    ('p', "Screening used 500 deals per version; the best one (V2_1) was then confirmed on 1,000 deals it had never seen, as the rule set before "
          f"the run requires: {_s('CONFIRM_V2')}, whose lower end (57.3) is above 50. Strengths 0.5 and 1 scored the same in screening, so the "
          "exact strength matters little. The term must come <b>on top of</b> the near-Control credit (a Battleground one Influence short of Control): "
          "replacing it lost three quarters of the games. And the importance table matters: the version with every country equally important "
          f"(V2F_1R) won only {_s('V2F_1R').split(' ')[0]}%."),
    ('ends', ['results.jsonl'], ['CONFIRM_NE', 'CONFIRM_V2']),
    ('p', "How games end (all games of each test): CONFIRM_NE is the run 5 bot against itself, CONFIRM_V2 the new bot against the run 5 bot. "
          "Most games end at 20 VP or at Final Scoring; Wargames ends about one game in ten."),

    ('h', "5. The two smaller ideas"),
    ('table', _other, _W),
    ('p', "<b>Headline v2 for the USSR</b> was adopted in Red Sea; here it is neutral. <b>Set-up values</b> give a little credit, in the choice "
          "of move, for making an own preparation card's Event worth more when it is played later (Red Sea v5 found +8–14% more value from those "
          "Events). Here the realised value of preparation Events did not change: 17.3 evaluation points per game with weight 0.2 against 17.0 for the "
          "run 5 bot (± 0.55), and 17.2 against 17.2 with weight 0.4. Neither is adopted."),

    ('h', "6. Region record: which side does better where"),
    ('p', "You asked for this to split the importance table later into a US and a USSR column. Source: the run 5 bot playing itself on 1,000 deals "
          f"(test CONFIRM_NE; the USSR won {_res['CONFIRM_NE']['a_as_ussr']*100:.1f}% of these games despite the +2 US handicap). Each cell: the average "
          "net score of that Region's Scoring card (+ = US), then the share of scorings won by the US / by the USSR (the rest are ties). Final Scoring: "
          f"the {_reg['final_games']} games that reached it."),
    ('table', _region, (0.19, 0.2, 0.2, 0.2, 0.21)),
    ('p', "<b>What it shows.</b> The USSR is stronger in <b>Europe</b> in Early and Mid War (it wins over half the scorings, the US only 13–17%), "
          "but Europe is even in Late War. The USSR leads in the <b>Middle East</b> in every era. Asia is even early, then the US pulls ahead (+1.1 in "
          "Mid War, +2.0 in Late War, +1.9 at Final Scoring). <b>South America</b> leans US throughout (+1.0) and <b>Africa</b> slightly US; Central "
          "America and Southeast Asia are close to even, slightly USSR."),
    ('p', "<b>Caution.</b> This describes where each side ends up when the same bot plays both; it does not yet say which countries each side "
          "should value more. A two-column table built from it is a hypothesis to test, like the importance table itself."),

    ('h', "7. What changed, and next steps"),
    ('p', "<b>Adopted as default:</b> distance to Control in the corrected form, strength 1, with your importance table, on top of near-Control "
          "(engine <i>Weights::core</i>: ne=1, nv=2; the run 5 bot is <i>core:ne=0</i>). The game skill's helper (<i>tsts.py</i>) now includes the term and "
          "the table, and agrees with the engine on all 300 test positions. <b>Not adopted:</b> Headline v2 for the USSR, set-up values."),
    ('p', "<b>Next:</b> (1) the two-column (US / USSR) importance table, using section 6; (2) possibly retune the main weights now that a new term "
          "is in, since the old weights were tuned without it; (3) hand plan and shaping wait for the new Red Sea tests; (4) the move advisor "
          "(<i>tssim advise</i>) before the first real game; (5) avoidable damage from opponent Events stays open."),

    ('h', "8. How the bot evolved, run by run"),
    ('p', "The bot chooses each move by imagining the board that move would lead to and giving that board a score. The score adds up the things "
          "that win Twilight Struggle: victory points, who controls the key countries (Battlegrounds), how each Region would score, the Scoring cards "
          "in hand, the Military Operations requirement, and so on. Each test run changed how the bot judges or chooses, and then measured the change "
          "by playing thousands of games against the previous version, from both sides, on identical deals."),
    ('p', "<b>Run 1: checking the judgement.</b> Each part of the score was switched off in turn to see whether it helped. One part was hurting: "
          "an estimate of what the end-of-game scoring would give if the game ended now. It distracted the bot from the Scoring cards that actually "
          "decide most games. Without it the bot won <b>56.8%</b> of games against the first version."),
    ('p', "<b>Run 2: playing cards in the right order.</b> The first bots spent their best cards first. Run 2 taught the bot to keep strong cards "
          "for later in the turn, to play Scoring cards when the board favours it, to hold the China Card, and to stop a few DEFCON accidents. "
          "It won <b>59.9%</b> against the run 1 bot."),
    ('p', "<b>Run 3: never losing to DEFCON.</b> Using the card advice on Twilight Strategy (for example: never play Olympic Games for the Event at "
          "DEFCON 2), hard limits were added so the bot cannot make a move that ends the game in nuclear war. Losses at DEFCON 1 fell from 19 to 1 in 400 "
          "games. Overall strength barely moved (<b>51.0%</b>), because this only happened in about 5% of games, but it removed the most embarrassing "
          "way to lose."),
    ('p', "<b>Run 4: seeing what is about to happen.</b> Two ideas from the Red Sea work. First, the bot now values a Battleground where it is one "
          "Influence short of Control almost as much as one it controls, because either side can flip it with a single placement. Second, for its best "
          "few options it looks two moves ahead: its move, the opponent's best answer, and its own reply. This was by far the largest step: the new bot "
          "won <b>94.8%</b> against the run 3 bot."),
    ('p', "<b>Run 5: fine-tuning.</b> An automatic search adjusted the seven most important weights. The tuned bot wins <b>52.5%</b> against the "
          "run 4 bot: a small but real gain. Two other ideas (a simpler score, and a rule for when to play the opponent's cards) did not help and were dropped."),
    ('p', "<b>Run 6: knowing how far each country is.</b> The bot now looks at every country on the map, not only Battlegrounds: how many Operations "
          "points each side still needs to Control it, weighted by your table of how much each country matters. A country two points away counts "
          "for something; one five points away counts for little. The first version, copied from Red Sea, backfired because it forgot that Control "
          f"itself must count; corrected, the new bot wins <b>{_res['CONFIRM_V2']['score']*100:.1f}%</b> against the run 5 bot."),
    ('img', 'reports/fig/strength_run6.png', 0.85),
    ('p', "<b>Reading the chart.</b> Each win rate is turned into rating points, as in chess: a 100-point lead means winning about 64% of games, "
          "200 points about 76%. The steps are then added up, starting from the first bot at 0. The bars show the uncertainty, which grows as steps "
          "are added. Because each bot was measured only against the one before it, the totals are approximate; on this scale the run 6 bot would "
          "win about 98% of games against the first bot. Run 4 is the big jump; runs 1, 2 and 6 are the next largest gains; run 3's value (no DEFCON "
          "losses) does not show on a strength chart."),
]
