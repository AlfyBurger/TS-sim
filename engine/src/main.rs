mod agent;
mod bot;
mod data;
mod engine;
mod events;
mod rng;
mod state;
mod tests;
mod types;

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
        "diag" => {
            let n: u64 = arg(2, "20").parse().unwrap(); let seed0: u64 = arg(3, "5000").parse().unwrap(); let spec = arg(4, "core");
            let mut out = String::from("[");
            for i in 0..n {
                let (st, s, _) = play_one(seed0 + i, &spec, &spec, 2, false);
                out += &format!("{}{{\"seed\":{},\"winner\":{},\"vp\":{},\"end\":\"{}\",\"turn\":{},\"scoring\":{:?},\"china\":{:?},\"space\":{:?},\"opp_events\":{:?},\"coups\":{:?},\"realigns\":{:?},\"milops_pen\":{:?},\"vp_by_turn\":{:?},\"defcon_card\":{}}}",
                    if i > 0 { "," } else { "" }, seed0 + i, st.winner, st.vp, END_NAMES[st.end_reason as usize], st.turn, s.scoring_plays, s.china_plays, s.space_attempts, s.opp_events, s.coups, s.realigns, s.milops_penalty, s.vp_by_turn, s.defcon_loss_card);
            }
            out += "]"; println!("{out}");
        }
        _ => eprintln!("usage: tssim random N seed | match A B N seed threads out.csv | log seed US USSR | diag N seed spec"),
    }
}
