"""Chained strength chart: each run's final bot vs the previous run's bot, converted to rating points (Elo scale)."""
import math, sys, matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
steps = [("First bot", None), ("Run 1", (0.568, 0.031)), ("Run 2", (0.599, 0.020)), ("Run 3", (0.510, 0.0185)),
         ("Run 4", (0.948, 0.009)), ("Run 5", (0.525, 0.019))]
elo = lambda p: 400 * math.log10(p / (1 - p))
xs, ys, es, gains = [], [], [], []; tot, var = 0.0, 0.0
for i, (name, s) in enumerate(steps):
    if s:
        p, m = s; g = elo(p); e = (elo(min(p + m, 0.999)) - elo(max(p - m, 0.001))) / 2
        tot += g; var += e * e; gains.append((name, g, e))
    xs.append(name); ys.append(tot); es.append(math.sqrt(var))
for n, g, e in gains: print(f"{n}: +{g:.0f} ± {e:.0f}")
print("cumulative", [round(y) for y in ys], "±", [round(e) for e in es])
fig, ax = plt.subplots(figsize=(6.4, 3.0), dpi=200)
ax.errorbar(range(len(xs)), ys, yerr=es, color='#2a5d9f', marker='o', ms=5, capsize=3, lw=1.8)
for i, y in enumerate(ys): ax.annotate(f"{y:.0f}", (i, y), textcoords='offset points', xytext=(-24, 3), fontsize=7.5, color='#333')
ax.set_xticks(range(len(xs))); ax.set_xticklabels(xs, fontsize=8.5)
ax.set_ylabel("Strength (rating points)", fontsize=8.5); ax.tick_params(axis='y', labelsize=8)
ax.set_title("Bot strength after each test run (first bot = 0)", fontsize=9.5)
ax.grid(axis='y', color='#ddd', lw=0.6); ax.spines[['top', 'right']].set_visible(False)
fig.tight_layout(); fig.savefig(sys.argv[1]); print('wrote', sys.argv[1])
