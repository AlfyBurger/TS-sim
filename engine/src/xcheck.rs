//! Cross-check data for the game skill's helper script (tsts.py): random positions with the engine's
//! Control, Region scores, Scoring-card values, final projection, evaluation and best Ops use.
use crate::agent::*;
use crate::bot::*;
use crate::data::*;
use crate::engine::*;
use crate::state::*;
use crate::types::*;

struct R(u64);
impl R {
    fn n(&mut self) -> u64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0 }
    fn u(&mut self, k: u64) -> u64 { self.n() % k }
    fn b(&mut self, p: u64) -> bool { self.u(100) < p }
}

const EFFECT_FLAGS: [&str; 17] = ["nato", "de_gaulle", "willy_brandt", "us_japan", "formosan", "reformer", "quagmire", "bear_trap",
    "containment", "brezhnev", "vietnam_revolts", "nuclear_subs", "salt", "iran_contra", "yuri", "nso_turn", "shuttle"];

fn set_flag(e: &mut Effects, k: &str) {
    match k {
        "nato" => e.nato = true, "de_gaulle" => e.de_gaulle = true, "willy_brandt" => e.willy_brandt = true, "us_japan" => e.us_japan = true,
        "formosan" => e.formosan = true, "reformer" => e.reformer = true, "quagmire" => e.quagmire = true, "bear_trap" => e.bear_trap = true,
        "containment" => e.containment = true, "brezhnev" => e.brezhnev = true, "vietnam_revolts" => e.vietnam_revolts = true,
        "nuclear_subs" => e.nuclear_subs = true, "salt" => e.salt = true, "iran_contra" => e.iran_contra = true, "yuri" => e.yuri = true,
        "nso_turn" => e.nso_turn = true, "shuttle" => e.shuttle = true, _ => {}
    }
}

fn sname(s: usize) -> &'static str { side_name(s) }

pub fn random_state(seed: u64) -> (State, Vec<String>) {
    let mut r = R(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
    for _ in 0..4 { r.n(); }
    let mut st = State::new();
    st.turn = 1 + r.u(10) as u8;
    let ars = if st.turn <= 3 { 6 } else { 7 };
    st.ar = 1 + r.u(ars) as u8;
    st.phasing = r.u(2) as u8;
    st.defcon = 2 + r.u(4) as i8;
    st.vp = r.u(31) as i32 - 15;
    st.milops = [r.u(6) as i8, r.u(6) as i8];
    for s in 0..2 { st.space[s] = if r.b(50) { 0 } else { r.u(9) as u8 }; }
    for k in 1..9usize {
        let a = st.space[0] as usize >= k; let b = st.space[1] as usize >= k;
        st.space_first[k] = if a && b { r.u(2) as i8 } else if a { 0 } else if b { 1 } else { -1 };
    }
    st.space_attempts = [r.u(2) as u8, r.u(2) as u8];
    st.china = r.u(2) as u8; st.china_up = r.b(75);
    for c in 0..NC {
        if r.b(55) { st.inf[c][0] = r.u(6) as i8; }
        if r.b(55) { st.inf[c][1] = r.u(6) as i8; }
        if r.b(15) { let s = r.u(2) as usize; st.inf[c][s] = COUNTRIES[c].stab + r.u(3) as i8; }
    }
    let mut flags = vec![];
    for f in EFFECT_FLAGS { if r.b(12) { set_flag(&mut st.ev, f); flags.push(f.to_string()); } }
    for s in 0..2 { if r.b(10) { st.ev.red_scare[s] = true; } }
    if r.b(10) { st.ev.lads = Some(r.u(2) as u8); }
    if r.b(10) { st.ev.chernobyl = Some(r.u(6) as u8); }
    if r.b(5) { st.ev.cmc = Some(r.u(2) as u8); }
    // cards: removed, hand of the phasing side
    let me = st.phasing as usize;
    let mut pool: Vec<u8> = (1..=110u8).filter(|&k| k != CHINA).collect();
    for i in (1..pool.len()).rev() { let j = r.u(i as u64 + 1) as usize; pool.swap(i, j); }
    let nrem = r.u(15) as usize;
    for &k in pool.iter().take(nrem) { st.removed.push(k); }
    let nh = 1 + r.u(9) as usize;
    for &k in pool.iter().skip(nrem).take(nh) { st.hands[me].push(k); }
    st.rounds_done = [r.u(st.ar as u64) as u8, r.u(st.ar as u64) as u8];
    st.snapshot_presence();
    (st, flags)
}

pub fn state_json(st: &State, flags: &[String]) -> String {
    let me = st.phasing as usize;
    let mut inf = vec![];
    for c in 0..NC { if st.inf[c] != [0, 0] { inf.push(format!("\"{}\":[{},{}]", COUNTRIES[c].name, st.inf[c][0], st.inf[c][1])); } }
    let mut eff: Vec<String> = flags.iter().map(|f| format!("\"{f}\":true")).collect();
    let rs: Vec<String> = (0..2).filter(|&s| st.ev.red_scare[s]).map(|s| format!("\"{}\"", sname(s))).collect();
    if !rs.is_empty() { eff.push(format!("\"red_scare\":[{}]", rs.join(","))); }
    if let Some(s) = st.ev.lads { eff.push(format!("\"lads\":\"{}\"", sname(s as usize))); }
    if let Some(rg) = st.ev.chernobyl { eff.push(format!("\"chernobyl\":\"{}\"", REGION_NAMES[rg as usize])); }
    if let Some(s) = st.ev.cmc { eff.push(format!("\"cmc\":\"{}\"", sname(s as usize))); }
    let sf: Vec<String> = (1..9).filter(|&k| st.space_first[k] >= 0).map(|k| format!("\"{}\":\"{}\"", k, sname(st.space_first[k] as usize))).collect();
    format!("{{\"turn\":{},\"ar\":{},\"phasing\":\"{}\",\"me\":\"{}\",\"defcon\":{},\"vp\":{},\"milops\":[{},{}],\"space\":[{},{}],\"space_first\":{{{}}},\"space_attempts\":[{},{}],\"china\":\"{}\",\"china_up\":{},\"inf\":{{{}}},\"effects\":{{{}}},\"hand\":{:?},\"removed\":{:?},\"rounds_done\":[{},{}]}}",
        st.turn, st.ar, sname(me), sname(me), st.defcon, st.vp, st.milops[0], st.milops[1], st.space[0], st.space[1], sf.join(","),
        st.space_attempts[0], st.space_attempts[1], sname(st.china as usize), st.china_up, inf.join(","), eff.join(","),
        st.hands[me].as_slice(), st.removed.as_slice(), st.rounds_done[0], st.rounds_done[1])
}

pub fn run(n: u64, seed0: u64) {
    let b = Brain { w: Weights::core() };
    for i in 0..n {
        let (st, flags) = random_state(seed0 + i);
        let me = st.phasing as usize;
        let ctrl: Vec<i8> = (0..NC).map(|c| st.ctrl(c)).collect();
        let regs: Vec<String> = (0..6u8).map(|r| { let (u, s) = region_score(&st, r, None); format!("[{u},{s}]") }).collect();
        let (su, ss) = sea_score(&st);
        let nets: Vec<String> = [1u8, 2, 3, 37, 38, 79, 81].iter().map(|&k| format!("\"{}\":{}", k, scoring_net(&st, k))).collect();
        let mut ops = vec![];
        for k in st.hands[me].iter() {
            if CARDS[k as usize].scoring { continue; }
            let spec = OpsSpec { base: st.card_ops(me, k), card: k, allow_coup: true, allow_realign: true, allow_place: true, allow_space: true };
            let (v, ch) = b.best_ops(&st, me, &spec);
            let chs = match ch { OpsChoice::Place(_) => "place".to_string(), OpsChoice::Coup(c) => format!("coup:{}", COUNTRIES[c].name),
                OpsChoice::Realign(_) => "realign".to_string(), OpsChoice::Space => "space".to_string(), OpsChoice::None => "none".to_string() };
            ops.push(format!("{{\"card\":{},\"ops\":{},\"value\":{:.6},\"choice\":\"{}\"}}", k, spec.base, v, chs));
        }
        println!("{{\"seed\":{},\"state\":{},\"ctrl\":{:?},\"regions\":[{}],\"sea\":[{},{}],\"nets\":{{{}}},\"final\":{},\"eval\":[{:.6},{:.6}],\"rounds_for\":[{},{}],\"best_ops\":[{}]}}",
            seed0 + i, state_json(&st, &flags), ctrl, regs.join(","), su, ss, nets.join(","), final_projection(&st),
            eval(&b.w, &st, US), eval(&b.w, &st, USSR), rounds_for(&st, US), rounds_for(&st, USSR), ops.join(","));
    }
}
