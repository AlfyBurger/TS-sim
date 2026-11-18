"""Block queue for test runs split between the cloud and Alfy's Mac (run 4, after Red Sea v7).

A test = LABEL, bot A, bot B, first seed, number of deals. It is cut into blocks of BLOCK deals; each block is one
`tssim match A B n seed threads out.csv` (both sides per deal) writing results/<run>/blocks/LABEL__<seed>.csv.
Blocks are independent and results are identical on both machines (checked 17 Nov 2026), so any machine may run any block.

  blocks.py add RUNDIR LABEL A B SEED0 N [BLOCK=50]     append a test's blocks to RUNDIR/jobs.txt
  blocks.py claim RUNDIR front|back K OUT               move K blocks from the queue into OUT (a job list for one machine)
  blocks.py run JOBS BIN THREADS BUDGET OUTDIR           run the jobs whose CSV is missing, within BUDGET seconds
  blocks.py requeue RUNDIR JOBS                          put a job list's unfinished blocks back at the end of the queue
  blocks.py merge RUNDIR LABEL [LABEL...]                summarise finished tests (score, paired margin, sides, ends)
  blocks.py status RUNDIR                                queued blocks and finished CSVs per label
"""
import fcntl, json, math, os, subprocess, sys, time

def _lock(rundir):
    f = open(os.path.join(rundir, 'jobs.lock'), 'a+'); fcntl.flock(f, fcntl.LOCK_EX); return f

def _read(p):
    return [l for l in open(p).read().splitlines() if l.strip()] if os.path.exists(p) else []

def add(rundir, label, a, b, seed0, n, block=50):
    os.makedirs(os.path.join(rundir, 'blocks'), exist_ok=True)
    lk = _lock(rundir); jobs = os.path.join(rundir, 'jobs.txt')
    with open(jobs, 'a') as f:
        for s in range(int(seed0), int(seed0) + int(n), int(block)):
            f.write('\t'.join([label, a, b, str(s), str(min(int(block), int(seed0) + int(n) - s))]) + '\n')
    meta = os.path.join(rundir, 'tests.jsonl')
    with open(meta, 'a') as f: f.write(json.dumps({'label': label, 'a': a, 'b': b, 'seed0': int(seed0), 'n': int(n)}) + '\n')
    lk.close()

def claim(rundir, end, k, out):
    lk = _lock(rundir); jobs = os.path.join(rundir, 'jobs.txt'); lines = _read(jobs); k = int(k)
    take, rest = (lines[:k], lines[k:]) if end == 'front' else (lines[-k:] if k else [], lines[:-k] if k else lines)
    open(jobs, 'w').write(''.join(l + '\n' for l in rest))
    open(out, 'a').write(''.join(l + '\n' for l in take))
    lk.close(); print(len(take), 'claimed,', len(rest), 'left')

def csv_name(label, seed): return f'{label}__{seed}.csv'

def run(jobs, binp, threads, budget, outdir):
    t0 = time.time(); last = 0.0; done = 0; lines = _read(jobs)
    os.makedirs(outdir, exist_ok=True)
    for l in lines:
        label, a, b, seed, n = l.split('\t')
        out = os.path.join(outdir, csv_name(label, seed))
        if os.path.exists(out): done += 1; continue
        if time.time() - t0 + 1.3 * last > float(budget): break
        s = time.time(); tmp = out + '.part'
        subprocess.run([binp, 'match', a, b, n, seed, str(threads), tmp], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if os.path.exists(tmp) and sum(1 for _ in open(tmp)) == 2 * int(n): os.replace(tmp, out); done += 1
        last = time.time() - s
    print(f'{done}/{len(lines)} blocks done, last {last:.0f}s, elapsed {time.time() - t0:.0f}s')

def requeue(rundir, jobs):
    outdir = os.path.join(rundir, 'blocks'); back = []
    for l in _read(jobs):
        label, a, b, seed, n = l.split('\t')
        if not os.path.exists(os.path.join(outdir, csv_name(label, seed))): back.append(l)
    lk = _lock(rundir)
    with open(os.path.join(rundir, 'jobs.txt'), 'a') as f: f.write(''.join(l + '\n' for l in back))
    lk.close(); print(len(back), 'requeued')

def summarise(rundir, label):
    tests = {json.loads(l)['label']: json.loads(l) for l in _read(os.path.join(rundir, 'tests.jsonl'))}
    t = tests[label]; outdir = os.path.join(rundir, 'blocks'); rows = []
    for fn in sorted(os.listdir(outdir)):
        if fn.startswith(label + '__') and fn.endswith('.csv'):
            rows += [r.split(',') for r in _read(os.path.join(outdir, fn))]
    per, side, ends = {}, {0: [], 1: []}, [0] * 8
    for row in rows:
        seed, flag, win, vp, end, turn = row[:6]
        a_side = 0 if flag == '0' else 1; w = int(win)
        v = 0.5 if w == 2 else (1.0 if w == a_side else 0.0)
        per.setdefault(int(seed), []).append(v); side[a_side].append(v); ends[int(end)] += 1
    xs = [sum(v) / len(v) for v in per.values()]; n = len(xs)
    if n < 2: return None
    m = sum(xs) / n; sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1))
    return {'label': label, 'a': t['a'], 'b': t['b'], 'n': n, 'n_planned': t['n'], 'score': m, 'margin': 1.96 * sd / math.sqrt(n),
            'a_as_us': sum(side[0]) / max(1, len(side[0])), 'a_as_ussr': sum(side[1]) / max(1, len(side[1])), 'ends': ends}

def merge(rundir, labels):
    for lab in labels:
        r = summarise(rundir, lab)
        if not r: print(lab, 'no results'); continue
        with open(os.path.join(rundir, 'results.jsonl'), 'a') as f: f.write(json.dumps(r) + '\n')
        print(f"{lab:28} score {r['score']*100:5.1f} ± {r['margin']*100:.1f}  as US {r['a_as_us']*100:5.1f}  as USSR {r['a_as_ussr']*100:5.1f}  ({r['n']}/{r['n_planned']} deals)")

def status(rundir):
    q = _read(os.path.join(rundir, 'jobs.txt')); outdir = os.path.join(rundir, 'blocks')
    done = {}
    for fn in os.listdir(outdir) if os.path.exists(outdir) else []:
        if fn.endswith('.csv'): done[fn.split('__')[0]] = done.get(fn.split('__')[0], 0) + 1
    qd = {}
    for l in q: qd[l.split('\t')[0]] = qd.get(l.split('\t')[0], 0) + 1
    for t in [json.loads(l) for l in _read(os.path.join(rundir, 'tests.jsonl'))]:
        print(f"{t['label']:28} done {done.get(t['label'], 0):3}  queued {qd.get(t['label'], 0):3}")

if __name__ == '__main__':
    cmd, args = sys.argv[1], sys.argv[2:]
    {'add': lambda: add(*args), 'claim': lambda: claim(*args), 'run': lambda: run(*args), 'requeue': lambda: requeue(*args),
     'merge': lambda: merge(args[0], args[1:]), 'status': lambda: status(*args)}[cmd]()
