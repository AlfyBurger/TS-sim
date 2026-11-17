TITLE = "Twilight Struggle simulator — Stage 2, test run 3"
SUBTITLE = ("For Alfy · 17 November 2026 · repository AlfyBurger/TS-sim · 2026 Hall of Fame rules, base game, fixed +2 US handicap · "
            "DEFCON boundaries from Twilight Strategy (twilightstrategy.com) as hard limits")
FOOTER = "TS-sim · Stage 2 run 3 test report"
REF_NAME = "run 2 bot"

VERDICTS = {"TSB_defcon_boundaries": "Neutral in strength; DEFCON losses 19 → 1 per 400 games: <b>adopted</b>"}

SECTIONS = [
    ('h', "1. Summary"),
    ('p', "After run 2 the bot still lost about 5% of its games at DEFCON 1. It treated DEFCON danger as a penalty to weigh against other gains, plus "
          "a filter that only works if every random outcome is simulated. Following Alfy's pointer to twilightstrategy.com, run 3 turned the site's DEFCON "
          "advice into <b>hard limits</b>: plays the bot removes before evaluating anything, and makes only when nothing else is legal (switch <i>tsb</i>, "
          "default on). A rules bug was fixed along the way: the 8th Action Round is optional."),
    ('p', "<b>Result.</b> DEFCON losses in 400 self-play games fell from <b>19 to 1</b>. Head to head against the run 2 bot the limits score "
          "51.0 ± 1.8: no measurable change in strength, which is expected for a problem that hit about 5% of games. They cost nothing elsewhere, so they are adopted."),

    ('h', "2. Source"),
    ('p', "Twilight Strategy (twilightstrategy.com; also a free Leanpub book, CC BY-NC-SA, 2014) was already among the Stage 1 sources, but only its card list "
          "and its general and region articles had been used. All 120 card articles and their 1,344 reader comments were downloaded and searched for DEFCON "
          "warnings and \"never play\" advice. The site's general DEFCON article sorts the dangerous cards into groups; together with the card articles "
          "(e.g. Olympic Games: \"you can't play Olympic Games for the event when DEFCON is at 2\") this gives the rules below. Full text: docs/defcon_boundaries.md."),

    ('h', "3. The limits"),
    ('table', [
        ["Group", "Cards", "Never, on my own round at DEFCON 2"],
        ["Unconditional degraders", "Duck and Cover, \"We Will Bury You\", Soviets Shoot Down KAL-007", "trigger the Event, as my own Event or by playing the opponent's card for Ops"],
        ["Opponent conducts Ops on my round", "CIA Created, \"Lone Gunman\", Grain Sales to Soviets, Tear Down This Wall", "trigger the Event if the opponent could then coup a Battleground of mine (not under my Cuban Missile Crisis, or against US Nuclear Subs)"],
        ["Free coup next to Nicaragua", "Ortega Elected in Nicaragua", "as US, play it for Ops while I have Influence in Cuba"],
        ["Chance of degrading", "Olympic Games, Summit, Missile Envy", "play for the Event (Ops are fine)"],
        ["", "Five Year Plan", "as US: the Event while Duck and Cover or KAL-007 may be in the USSR hand; as USSR: Ops while holding a degrader that could be discarded into play"],
    ], (0.22, 0.32, 0.46)),
    ('p', "<b>Headline.</b> At DEFCON 3, never Headline one of these cards while a card that lowers DEFCON and resolves first may still be in the opponent's "
          "hand: either side's degraders count (a player may Headline the opponent's card just to drop DEFCON), plus How I Learned to Stop Worrying, "
          "Cuban Missile Crisis and Junta. At DEFCON 4–5 only Cuban Missile Crisis and How I Learned to Stop Worrying can reach DEFCON 2 in one Headline."),
    ('p', "<b>Hand planning.</b> If the Action Rounds left this turn exceed the safe cards in hand (plus the China Card), play a card that would be suicidal at "
          "DEFCON 2 now, while DEFCON is still 3 or more."),
    ('p', "<b>Rules fix.</b> The 8th Action Round from Space Station or North Sea Oil is optional (\"may play 8\"); the engine had forced it, which forced "
          "Lone Gunman out in some games. The bot can now decline it."),

    ('h', "4. What the diagnostics showed"),
    ('table', [
        ["Cause of DEFCON 1 loss (400 self-play games each)", "Without limits", "With limits"],
        ["Summit played as an Event", "9", "0"],
        ["Grain Sales to Soviets in the Headline", "5", "0"],
        ["CIA Created (Headline or played for Ops)", "2", "0"],
        ["Ortega Elected, Lone Gunman in the Headline", "2", "0"],
        ["Missile Envy as an Event", "1", "0"],
        ["Lone Gunman forced out in the last Action Round", "0", "1"],
        ["<b>Total</b>", "<b>19</b>", "<b>1</b>"],
    ], (0.6, 0.2, 0.2)),
    ('p', "Two gaps were found and closed while building the limits, both by reading the losing games: a Headline loss where the US Headlined "
          "\"We Will Bury You\" (a USSR card) just to drop DEFCON before the USSR's Grain Sales Headline, which the first version did not count; and "
          "Lone Gunman forced out in Action Rounds 7–8, fixed by the optional 8th round and the hand-planning rule."),

    ('h', "5. Head-to-head test (1,000 deals × both sides, deals 400000–400999)"),
    ('results', ['defcon.jsonl'], None),

    ('h', "6. Not yet built"),
    ('p', "Other ways out of a DEFCON trap named by Twilight Strategy: the China Card, UN Intervention, Quagmire or Bear Trap on yourself, raising DEFCON "
          "(SALT Negotiations, Nuclear Test Ban), Headlining the card at DEFCON 4 or more. The card articles also hold per-card play advice (Event or Ops, "
          "when to Headline, which side benefits) still to be mined for the game skill's card notes and Stage 3."),
]
