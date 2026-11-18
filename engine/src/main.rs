mod agent;
mod bot;
mod data;
mod engine;
mod events;
mod rng;
mod search;
mod state;
mod tests;
mod types;
mod xcheck;

use bot::make_agent;
use data::*;
use engine::*;
use state::*;
use std::io::Write;
use types::*;

/// Every card exactly once: hands, deck, discard, removed, China, Shuttle Diplomacy in effect.
pub fn check_invariants(st: &State) -> Result<(), String> {
    let mut seen = [0u8; 111];
    for l in [&st.hands[0], &st.hands[1], &st.deck, &st.discard, &st.removed] { for k in l.iter() { seen[k as usize] += 1; } }
    if st.ev.shuttle { seen[73] += 1; }
    seen[CHINA as usize] += 1;
    let era_max = if st.turn >= 8 { 2 } else if st.turn >= 4 { 1 } else { 0 };
    for k in 1..=110usize {
        let expect = if CARDS[k].era <= era_max { 1 } else { 0 };
        if seen[k] != expect { return Err(format!("card {} ({}) seen {} times, expected {}", k, CARDS[k].name, seen[k], expect)); }
    }
    for c in 0..NC { if st.inf[c][0] < 0 || st.inf[c][1] < 0 { return Err(format!("negative influence in {}", COUNTRIES[c].name)); } }
    if st.winner < 0 && !(1..=5).contains(&st.defcon) { return Err(format!("DEFCON {}", st.defcon)); }
    for s in 0..2 { if st.milops[s] < 0 || st.milops[s] > 5 { return Err("milops".into()); } if st.space[s] > 8 { return Err("space".into()); } }
    Ok(())
}

pub struct GameResult { pub seed: u64, pub us_bot: usize, pub winner: i8, pub vp: i32, pub end: u8, pub turn: u8, pub stats: Stats }

pub fn play_one(seed: u64, us: &str, ussr: &str, handicap: i8, log: bool) -> (State, Stats, Option<Vec<String>>) {
    let mut a = make_agent(us); let mut b = make_agent(ussr);
    let mut st = State::new();
    let mut cx = Ctx::new(seed, a.as_mut(), b.as_mut());
    if log { cx.log = Some(vec![]); }
    play_game(&mut st, &mut cx, handicap);
    let l = cx.log.take();
    (st, cx.stats, l)
}


/// Score of A (win 1, draw 0.5) from a run_pairs CSV.
fn csv_score(path: &str) -> f64 {
    let t = std::fs::read_to_string(path).unwrap_or_default();
    let (mut s, mut n) = (0.0, 0.0);
    for l in t.lines() {
        let f: Vec<&str> = l.split(',').collect(); if f.len() < 3 { continue; }
        let a_side: i64 = if f[1] == "0" { 0 } else { 1 }; let w: i64 = f[2].parse().unwrap_or(-1);
        s += if w == 2 { 0.5 } else if w == a_side { 1.0 } else { 0.0 }; n += 1.0;
    }
    if n > 0.0 { s / n } else { 0.5 }
}

/// SPSA weight tuning (run 5). Job line: SPSA<label> TAB base-spec TAB comma list of weights TAB seed0 TAB iterations.
/// Each iteration plays 50 deals (both sides) of theta+ against theta-; progress in OUTDIR/<label>_log.txt (resumable);
/// the result goes to tuned.txt next to the job list and to the job's marker file.
fn spsa(label: &str, base: &str, params: &str, seed0: u64, iters: usize, threads: usize, outdir: &str, jobs_path: &str) {
    let names: Vec<&str> = params.split(',').filter(|x| !x.is_empty()).collect();
    let bw = bot::weights_from_spec(base);
    let scale: Vec<f64> = names.iter().map(|k| bw.get(k).abs().max(0.25)).collect();
    let mut theta: Vec<f64> = names.iter().map(|k| bw.get(k)).collect();
    let log = std::path::Path::new(outdir).join(format!("{}_log.txt", label));
    let mut t0 = 0usize;
    if let Ok(prev) = std::fs::read_to_string(&log) {
        if let Some(last) = prev.lines().filter(|l| !l.starts_with('#')).last() {
            let f: Vec<&str> = last.split('\t').collect();
            if f.len() >= 3 + names.len() { t0 = f[0].parse().unwrap_or(0); for i in 0..names.len() { theta[i] = f[3 + i].parse().unwrap_or(theta[i]); } }
        }
    } else { std::fs::write(&log, format!("# iteration\tscore(theta+ vs theta-)\tc\t{}\n", names.join("\t"))).unwrap(); }
    let spec = |th: &Vec<f64>| { let kv: Vec<String> = names.iter().zip(th).map(|(k, v)| format!("{}={:.4}", k, v)).collect();
        if base.contains(':') { format!("{},{}", base, kv.join(",")) } else { format!("{}:{}", base, kv.join(",")) } };
    let (a, c, big_a) = (0.8, 0.15, 10.0);
    let deals = 50usize;
    for t in (t0 + 1)..=iters {
        let ct = c / (t as f64).powf(0.101); let at = a / (t as f64 + big_a).powf(0.602);
        let mut x: u64 = 0x9E3779B97F4A7C15 ^ (seed0 + t as u64).wrapping_mul(0x2545F4914F6CDD1D);
        let delta: Vec<f64> = (0..names.len()).map(|_| { x ^= x << 13; x ^= x >> 7; x ^= x << 17; if x & 1 == 0 { 1.0 } else { -1.0 } }).collect();
        let clampv = |k: &str, v: f64| { let v = v.max(0.0); if k == "nc" { v.min(1.2) } else if k == "sq" { v.min(0.95) } else { v } };
        let plus: Vec<f64> = (0..names.len()).map(|i| clampv(names[i], theta[i] + ct * delta[i] * scale[i])).collect();
        let minus: Vec<f64> = (0..names.len()).map(|i| clampv(names[i], theta[i] - ct * delta[i] * scale[i])).collect();
        let tmp = std::path::Path::new(outdir).join(format!("{}_iter.part", label));
        run_pairs(&spec(&plus), &spec(&minus), deals, seed0 + (t as u64) * 1000, threads, 2, tmp.to_str().unwrap());
        let sc = csv_score(tmp.to_str().unwrap()); let _ = std::fs::remove_file(&tmp);
        for i in 0..names.len() { let g = (sc - 0.5) / (2.0 * ct * delta[i]); theta[i] = clampv(names[i], theta[i] + at * g * scale[i]); }
        let line = format!("{}\t{:.4}\t{:.4}\t{}\n", t, sc, ct, theta.iter().map(|v| format!("{:.4}", v)).collect::<Vec<_>>().join("\t"));
        use std::io::Write as _; std::fs::OpenOptions::new().append(true).open(&log).unwrap().write_all(line.as_bytes()).unwrap();
        println!("SPSA {} iteration {}/{}: score {:.3}; {}", label, t, iters, sc, names.iter().zip(&theta).map(|(k, v)| format!("{}={:.3}", k, v)).collect::<Vec<_>>().join(" "));
    }
    let kv: Vec<String> = names.iter().zip(&theta).map(|(k, v)| format!("{}={:.4}", k, v)).collect();
    let tuned = std::path::Path::new(jobs_path).with_file_name("tuned.txt");
    std::fs::write(&tuned, kv.join(",")).unwrap();
    std::fs::write(std::path::Path::new(outdir).join(format!("{}__{}.csv", label, seed0)), kv.join(",")).unwrap();
}

fn run_pairs(a: &str, b: &str, n: usize, seed0: u64, threads: usize, handicap: i8, out: &str) {
    use std::sync::Mutex;
    let results = Mutex::new(Vec::<(u64, usize, i8, i32, u8, u8)>::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if i >= n { break; }
                let seed = seed0 + i as u64;
                for us_is_a in [true, false] {
                    let (us, ussr) = if us_is_a { (a, b) } else { (b, a) };
                    let (st, _s, _) = play_one(seed, us, ussr, handicap, false);
                    results.lock().unwrap().push((seed, if us_is_a { 0 } else { 1 }, st.winner, st.vp, st.end_reason, st.turn));
                }
            });
        }
    });
    let mut r = results.into_inner().unwrap();
    r.sort();
    let mut f = std::fs::File::create(out).unwrap();
    for x in &r { writeln!(f, "{},{},{},{},{},{}", x.0, x.1, x.2, x.3, x.4, x.5).unwrap(); }
    // A's score: win 1, draw 0.5
    let score = |x: &(u64, usize, i8, i32, u8, u8)| { let a_side = if x.1 == 0 { US as i8 } else { USSR as i8 }; if x.2 == 2 { 0.5 } else if x.2 == a_side { 1.0 } else { 0.0 } };
    let mut per_seed: std::collections::BTreeMap<u64, Vec<f64>> = Default::default();
    let (mut us_s, mut us_n, mut su_s, mut su_n) = (0.0, 0, 0.0, 0);
    for x in &r { let v = score(x); per_seed.entry(x.0).or_default().push(v); if x.1 == 0 { us_s += v; us_n += 1 } else { su_s += v; su_n += 1 } }
    let xs: Vec<f64> = per_seed.values().map(|v| v.iter().sum::<f64>() / v.len() as f64).collect();
    let m = xs.iter().sum::<f64>() / xs.len() as f64;
    let sd = (xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (xs.len() as f64 - 1.0)).sqrt();
    let marg = 1.96 * sd / (xs.len() as f64).sqrt();
    let mut ends = [0usize; 8]; for x in &r { ends[x.4 as usize] += 1; }
    println!("{{\"a\":\"{}\",\"b\":\"{}\",\"n\":{},\"score\":{:.4},\"margin\":{:.4},\"a_as_us\":{:.4},\"a_as_ussr\":{:.4},\"ends\":{:?}}}", a, b, n, m, marg, us_s / us_n as f64, su_s / su_n as f64, ends);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let arg = |i: usize, d: &str| args.get(i).cloned().unwrap_or(d.to_string());
    match cmd {
        "random" => {
            let n: u64 = arg(2, "100").parse().unwrap(); let seed0: u64 = arg(3, "1").parse().unwrap();
            let t0 = std::time::Instant::now();
            let mut ends = [0usize; 8]; let mut us = 0; let mut turns = 0u64;
            for i in 0..n {
                let mut a = make_agent("random"); let mut b = make_agent("random");
                let mut st = State::new();
                let mut cx = Ctx::new(seed0 + i, a.as_mut(), b.as_mut());
                setup(&mut st, &mut cx, 2);
                check_invariants(&st).unwrap_or_else(|e| panic!("seed {} setup: {}", seed0 + i, e));
                for t in 1..=10 { st.turn = t; play_turn(&mut st, &mut cx); if let Err(e) = check_invariants(&st) { panic!("seed {} turn {}: {}", seed0 + i, t, e); } if st.winner >= 0 { break; } }
                if st.winner < 0 { final_scoring(&mut st, &mut cx); }
                ends[st.end_reason as usize] += 1; if st.winner == 0 { us += 1; } turns += st.turn as u64;
            }
            println!("{{\"games\":{},\"us_wins\":{},\"ends\":{:?},\"avg_turn\":{:.2},\"secs\":{:.1}}}", n, us, ends, turns as f64 / n as f64, t0.elapsed().as_secs_f64());
        }
        "match" => {
            let a = arg(2, "core"); let b = arg(3, "random");
            let n: usize = arg(4, "10").parse().unwrap(); let seed: u64 = arg(5, "1000").parse().unwrap();
            let th: usize = arg(6, "2").parse().unwrap(); let out = arg(7, "/tmp/match.csv");
            let t0 = std::time::Instant::now();
            run_pairs(&a, &b, n, seed, th, 2, &out);
            eprintln!("secs {:.1}", t0.elapsed().as_secs_f64());
        }
        "log" => {
            let seed: u64 = arg(2, "1").parse().unwrap(); let us = arg(3, "core"); let ussr = arg(4, "core");
            let (st, _s, l) = play_one(seed, &us, &ussr, 2, true);
            for x in l.unwrap() { println!("{x}"); }
            println!("RESULT winner={} vp={} end={} turn={}", st.winner, st.vp, END_NAMES[st.end_reason as usize], st.turn);
        }
        "xcheck" => {
            // tssim xcheck N seed: random positions with engine values, for the game skill helper cross-check.
            xcheck::run(arg(2, "200").parse().unwrap(), arg(3, "1").parse().unwrap());
        }
        "diag" => {
            let n: u64 = arg(2, "20").parse().unwrap(); let seed0: u64 = arg(3, "5000").parse().unwrap(); let spec = arg(4, "core");
            let mut out = String::from("[");
            for i in 0..n {
                let (st, s, _) = play_one(seed0 + i, &spec, &spec, 2, false);
                out += &format!("{}{{\"seed\":{},\"winner\":{},\"vp\":{},\"end\":\"{}\",\"turn\":{},\"scoring\":{:?},\"china\":{:?},\"space\":{:?},\"opp_events\":{:?},\"coups\":{:?},\"realigns\":{:?},\"milops_pen\":{:?},\"vp_by_turn\":{:?},\"defcon_card\":{},\"ops_by_ar\":{:?},\"exits\":{:?},\"damage\":{:?},\"defcon_loss\":{},\"avoidable\":{:?}}}",
                    if i > 0 { "," } else { "" }, seed0 + i, st.winner, st.vp, END_NAMES[st.end_reason as usize], st.turn, s.scoring_plays, s.china_plays, s.space_attempts, s.opp_events, s.coups, s.realigns, s.milops_penalty, s.vp_by_turn, s.defcon_loss_card, s.ops_by_ar, s.exits, s.damage, s.defcon_loss.map_or("null".to_string(), |d| format!("{:?}", d)), s.avoidable);
            }
            out += "]"; println!("{out}");
        }
        "queue" => {
            // tssim queue JOBS OUTDIR [threads]: run every job line (label TAB A TAB B TAB seed TAB deals) whose CSV is missing.
            // Used on Alfy's PC: runs unattended; results OUTDIR/<label>__<seed>.csv, identical to the cloud's.
            let jobs = arg(2, "jobs.txt"); let outdir = arg(3, "out");
            let threads: usize = arg(4, "0").parse().unwrap_or(0);
            let threads = if threads == 0 { std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4) } else { threads };
            std::fs::create_dir_all(&outdir).unwrap();
            let text = std::fs::read_to_string(&jobs).expect("job list");
            let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
            let t0 = std::time::Instant::now();
            for (i, l) in lines.iter().enumerate() {
                let f: Vec<&str> = l.split('\t').collect();
                if f.len() < 5 { continue; }
                let out = std::path::Path::new(&outdir).join(format!("{}__{}.csv", f[0], f[3]));
                if out.exists() { continue; }
                let tmp = out.with_extension("part");
                let t = std::time::Instant::now();
                run_pairs(f[1], f[2], f[4].parse().unwrap(), f[3].parse().unwrap(), threads, 2, tmp.to_str().unwrap());
                std::fs::rename(&tmp, &out).unwrap();
                println!("[{}/{}] {} seed {}: {:.0}s (total {:.0} min)", i + 1, lines.len(), f[0], f[3], t.elapsed().as_secs_f64(), t0.elapsed().as_secs_f64() / 60.0);
            }
            println!("all jobs done in {:.0} min", t0.elapsed().as_secs_f64() / 60.0);
        }
        "worker" => {
            // tssim worker JOBS OUTDIR [threads]: standing worker for Alfy's PC. Re-reads JOBS before every job, runs the first line
            // whose CSV is missing, and waits for new lines when none is left; stops when OUTDIR/../STOP exists or the window is closed.
            let jobs = arg(2, "jobs.txt"); let outdir = arg(3, "out");
            let threads: usize = arg(4, "0").parse().unwrap_or(0);
            let threads = if threads == 0 { std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4) } else { threads };
            std::fs::create_dir_all(&outdir).unwrap();
            let stop = std::path::Path::new(&jobs).with_file_name("STOP");
            println!("TS simulator worker: {} threads; jobs from {}; results in {}. Close this window to stop.", threads, jobs, outdir);
            let t0 = std::time::Instant::now(); let mut idle = false; let mut done = 0;
            loop {
                if stop.exists() { println!("STOP file found: worker ends."); break; }
                let text = std::fs::read_to_string(&jobs).unwrap_or_default();
                let mut next = None;
                for l in text.lines() {
                    let f: Vec<&str> = l.trim_end_matches('\r').split('\t').collect();
                    if f.len() < 5 { continue; }
                    let out = std::path::Path::new(&outdir).join(format!("{}__{}.csv", f[0], f[3]));
                    if !out.exists() { next = Some((f.iter().map(|x| x.to_string()).collect::<Vec<String>>(), out)); break; }
                }
                match next {
                    Some((f, out)) => {
                        idle = false;
                        let tmp = out.with_extension("part");
                        let t = std::time::Instant::now();
                        if f[0].starts_with("SPSA") { spsa(&f[0], &f[1], &f[2], f[3].parse().unwrap(), f[4].parse().unwrap(), threads, &outdir, &jobs); }
                        else { run_pairs(&f[1], &f[2], f[4].parse().unwrap(), f[3].parse().unwrap(), threads, 2, tmp.to_str().unwrap());
                               std::fs::rename(&tmp, &out).unwrap(); }
                        done += 1;
                        println!("done {} seed {} in {:.0}s ({} blocks, {:.0} min so far)", f[0], f[3], t.elapsed().as_secs_f64(), done, t0.elapsed().as_secs_f64() / 60.0);
                    }
                    None => {
                        if !idle { println!("no jobs left: waiting for new ones (checking every 30 s)..."); idle = true; }
                        std::thread::sleep(std::time::Duration::from_secs(30));
                    }
                }
            }
        }
        "precheck" => {
            // tssim precheck A B N seed: how often B would choose differently from A, in A-vs-A games
            let a = arg(2, "core"); let b = arg(3, "core"); let n: u64 = arg(4, "30").parse().unwrap(); let seed0: u64 = arg(5, "600000").parse().unwrap();
            let counts = std::sync::Arc::new(std::sync::Mutex::new([0u64; 4]));
            for i in 0..n {
                let mut ag0 = bot::PrecheckAgent { base: bot::make_agent(&a), alt: bot::make_agent(&b), counts: counts.clone() };
                let mut ag1 = bot::PrecheckAgent { base: bot::make_agent(&a), alt: bot::make_agent(&b), counts: counts.clone() };
                let mut st = State::new();
                let mut cx = Ctx::new(seed0 + i, &mut ag0, &mut ag1);
                play_game(&mut st, &mut cx, 2);
            }
            let c = *counts.lock().unwrap();
            println!("{{\"a\":\"{}\",\"b\":\"{}\",\"games\":{},\"headlines\":{},\"headlines_changed\":{},\"actions\":{},\"actions_changed\":{},\"share_changed\":{:.4}}}",
                a, b, n, c[0], c[1], c[2], c[3], (c[1] + c[3]) as f64 / (c[0] + c[2]).max(1) as f64);
        }
        _ => eprintln!("usage: tssim random N seed | match A B N seed threads out.csv | log seed US USSR | diag N seed spec"),
    }
}
