TITLE = "Twilight Struggle simulator — Stage 2, test run 5"
SUBTITLE = ("For Alfy · 18 November 2026 · repository AlfyBurger/TS-sim · 2026 Hall of Fame rules, base game, fixed +2 US handicap · "
            "first run on Alfy's PC, started by Windows Task Scheduler at 23:00 and finished unattended at 06:30")
FOOTER = "TS-sim · Stage 2 run 5 test report"
REF_NAME = "run 4 bot"

VERDICTS = {
    "SIMPLE_5off": "<b>Harmful</b>: the five terms are kept",
    "EVT_1.0": "Neutral: not adopted",
    "EVT_0.5": "Neutral: not adopted",
    "TUNED_fresh": "<b>Gain</b>, on deals the tuning never saw: <b>adopted</b>",
    "COMBINED_A": "Below the tuned bot alone (includes the simplification)",
    "COMBINED_B": "Below the tuned bot alone (includes the simplification)",
}

SECTIONS = [
    ('h', "1. Summary"),
    ('p', "Run 5 tested three ideas against the run 4 bot, each over 1,000 deals played from both sides (2,000 games): "
          "(1) <b>simplification</b>: switching off the five evaluation terms that looked neutral in run 4; (2) a <b>timing rule for opponent "
          "Events</b>: play an opponent's card for Operations while its Event is harmless, instead of holding it until it hurts; (3) <b>automatic "
          "tuning</b> of the seven most important weights (SPSA, 100 steps), checked afterwards on fresh deals. Two combinations were also tested."),
    ('p', "<b>Main findings.</b> (1) <b>The tuned weights are a real gain</b>: 52.5 ± 1.9 against the run 4 bot on deals the tuning never used. "
          "They are now the default. (2) <b>Simplification hurts</b>: 45.0 ± 1.9. Each of the five terms was neutral when removed alone in run 4, "
          "but together they matter; they stay. (3) <b>The Event-timing rule is neutral</b> at both strengths (48.6 and 49.0, margins about 2), so it "
          "is not adopted; avoidable damage from opponent Events remains the main open weakness. (4) Both combinations include the simplification, "
          "and score below the tuned bot alone, consistent with (1) and (2)."),

    ('h', "2. Set-up: the PC"),
    ('p', "The whole run (121 jobs: 120 blocks of 50 deals plus the tuning job) ran on Alfy's PC (Intel i7-13700K, 24 threads) with the compiled "
          "Rust simulator, started by Windows Task Scheduler at 23:00 and finished at 06:30, while the Red Sea simulator shared the same machine. "
          "A block of 100 games took about 2 minutes. The PC plays exactly the same games as the cloud and the Mac (identical results on the same deals). "
          "A small wrapper kept the PC awake, wrote a status file every minute and stopped the simulator when every job had its result; Claude "
          "checked the start at 23:10 and collected the results in the morning, with no action needed from Alfy."),

    ('h', "3. Results (1,000 deals × both sides, against the run 4 bot)"),
    ('results', ['results.jsonl'], ['SIMPLE_5off', 'EVT_1.0', 'EVT_0.5', 'TUNED_fresh', 'COMBINED_A', 'COMBINED_B']),
    ('p', "Score: the share of games won by the tested bot (draws count half), with the 95% margin. Above 50 means better than the run 4 bot. "
          "The as-US / as-USSR columns differ because the game itself favours the USSR in self-play at this level, even with the +2 US handicap."),

    ('h', "4. The tuning"),
    ('p', "SPSA nudges all seven weights at once, plays the two nudged bots against each other on 50 deals and moves the weights toward the "
          "winner; 100 such steps were made. The steps are noisy by design (the nudged pairs scored 50.1% on average), so the result was judged "
          "only by the separate test on fresh deals (52.5 ± 1.9)."),
    ('table', [
        ["Weight (what it values)", "Run 4", "Tuned (now default)"],
        ["Region standing (presence, domination, control)", "0.25", "0.32"],
        ["Battlegrounds controlled", "1.00", "1.18"],
        ["Near-Control (one Influence short of a Battleground)", "1.00", "1.07"],
        ["Military Operations requirement", "1.00", "1.10"],
        ["China Card", "1.50", "1.71"],
        ["Held Scoring card", "1.00", "0.92"],
        ["Sequencing (keeping strong cards for later)", "0.50", "0.44"],
    ], (0.55, 0.2, 0.25)),
    ('p', "The direction is consistent: more weight on the map (Regions, Battlegrounds, Military Ops, the China Card), a little less on holding cards back."),

    ('h', "5. What changed, and next steps"),
    ('p', "<b>Adopted as default:</b> the seven tuned weights above (engine <i>Weights::core</i>; the run 4 bot is "
          "<i>core:region_unseen=0.25,bg=1,held=1,milops=1,nc=1,china=1.5,sq=0.5</i>). <b>Not adopted:</b> the simplification and the Event-timing rule. "
          "<b>Added tools:</b> SPSA tuning inside the worker, weight files (<i>@tuned.txt</i>), and the PC wrapper with scheduled start."),
    ('p', "<b>Next:</b> (1) the TS game skill (first version), its helper script and the cross-check with the engine, which is owed before Stage 3; "
          "(2) Stage 3 plans, after Red Sea's v7 result on choosing a plan from the opening hand; (3) avoidable damage from opponent Events stays open; "
          "the simple timing rule did not help, so the next attempt should start from the positions where the cost log shows the largest losses."),

    ('h', "6. How the bot evolved, run by run"),
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
    ('img', 'reports/fig/strength_run5.png', 0.85),
    ('p', "<b>Reading the chart.</b> Each win rate is turned into rating points, as in chess: a 100-point lead means winning about 64% of games, "
          "200 points about 76%. The steps are then added up, starting from the first bot at 0. The bars show the uncertainty, which grows as steps "
          "are added. Because each bot was measured only against the one before it, the totals are approximate; on this scale the run 5 bot would "
          "win about 97–98% of games against the first bot. Run 4 is the big jump; runs 1, 2 and 5 are steady gains, and run 3's value (no DEFCON "
          "losses) does not show on a strength chart."),
]
