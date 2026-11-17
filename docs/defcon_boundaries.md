# DEFCON boundaries (hard limits)

Source: Twilight Strategy (twilightstrategy.com), the general article on DEFCON (12 Dec 2011) and the per-card articles (Olympic Games, Summit, Missile Envy, Five Year Plan, CIA Created, "Lone Gunman", Grain Sales to Soviets, Tear Down This Wall, Ortega Elected in Nicaragua, Duck and Cover, "We Will Bury You", Soviets Shoot Down KAL-007, Cuban Missile Crisis, How I Learned to Stop Worrying, Nuclear Subs, Quagmire, Bear Trap, The China Card). Summarised in our own words; rule references are to the 2026 HoF rulebook.

Basis: DEFCON 1 on your own round (Action Round or your Headline) loses the game for you (8.1). At DEFCON 2 the cards below become "DEFCON suicide cards".

These are **hard limits**: the bot removes these plays before evaluating anything, and makes one only if nothing else is legal. Switch: `tsb` (on by default from run 3).

## Action Rounds at DEFCON 2

| Group | Cards | Never, on my own round |
|---|---|---|
| Unconditional degraders | #4 Duck and Cover (US), #50 "We Will Bury You" (USSR), #89 Soviets Shoot Down KAL-007 (US) | trigger the Event: as my own Event, or by playing the opponent's card for Ops |
| Opponent conducts Ops on my round | #26 CIA Created (US), #62 "Lone Gunman" (USSR), #67 Grain Sales to Soviets (US), #96 Tear Down This Wall (US; its coup ignores Europe's DEFCON limit) | trigger the Event if the opponent could then coup a Battleground where I have Influence. Not a danger under my Cuban Missile Crisis, or when the US holds Nuclear Subs and the opponent is the US |
| Free coup next to Nicaragua | #91 Ortega Elected in Nicaragua (USSR) | as US, play it for Ops while I have Influence in Cuba |
| Chance of degrading | #20 Olympic Games (boycott), #45 Summit (winner may degrade), #49 Missile Envy (can hand me a degrader) | play for the Event (Ops are fine) |
| | #5 Five Year Plan (US) | as US: Event while Duck and Cover or KAL-007 may be in the USSR hand. As USSR: play it for Ops while my hand holds Duck and Cover, KAL-007, or CIA Created / Grain Sales / Tear Down This Wall with a Battleground coup open to the US |

## Headline

A Headline can lose even at DEFCON 3: if the other Headline resolves first (higher Ops, US first on ties) and lowers DEFCON to 2, my DEFCON-sensitive Headline then takes it to 1 on my turn.
- At DEFCON 3: never Headline any card in the table above while a degrader that would resolve first may still be in the opponent's hand. Degraders: #4, #50, #89 (either side's: a player may Headline the opponent's card just to drop DEFCON, e.g. the US Headlining "We Will Bury You"), #46 How I Learned to Stop Worrying, #40 Cuban Missile Crisis, #47 Junta, and the opponent's own coup-giver (#26 for the US, #62 for the USSR).
- At DEFCON 4 or 5: the same, but only Cuban Missile Crisis and How I Learned to Stop Worrying can bring DEFCON to 2 in one Headline.

## Hand planning

- Get rid of suicide cards while DEFCON is still 3 or more. If the Action Rounds left this turn exceed my safe cards (plus the China Card), one suicide card will be forced out later, so play it now.
- The 8th Action Round (Space Station, North Sea Oil) is optional ("may play 8"); the engine forced it until run 3. Not playing it lets a suicide card be held to the next turn.
- Other ways out named by Twilight Strategy, not yet built: send it to space (2+ Ops cards), the China Card, UN Intervention, Quagmire/Bear Trap on yourself, raising DEFCON (SALT Negotiations, Nuclear Test Ban), Headlining it at DEFCON 4+.

## Effect (run 3)

DEFCON losses in 400 self-play games: 19 without the limits, 1 with them (Lone Gunman forced out late in a lost position).
