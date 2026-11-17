//! Bots: random, and the evaluation bot (Stage 2 procedure).
use crate::agent::*;
use crate::data::*;
use crate::engine::*;
use crate::events::can_occur;
use crate::rng::{Purpose, Streams};
use crate::state::*;
use crate::types::*;
use crate::search::*;

// ------------------------------------------------------------------ weights
#[derive(Clone, Debug)]
pub struct Weights {
    pub vp: f64, pub held: f64, pub region_unseen: f64, pub final_: f64, pub bg: f64, pub europe_mult: f64, pub asia_mult: f64,
    pub access: f64, pub milops: f64, pub squeeze: f64, pub trap: f64, pub china: f64, pub bad: f64, pub space_ab: f64,
    pub inf: f64, pub auto_eu: f64, pub trapped: f64, pub h3: f64, pub h6: f64, pub samples: f64, pub noise: f64,
    pub final_from: f64,
    // decision rules (Stage 2 run 2): reply-scan margin (<0 off), Scoring timing, sequencing delta (<0 off),
    // Headline v2 (0 off, 1 both sides, 2 USSR only, 3 US only), set-up values weight, known card played sooner
    pub la: f64, pub tm: f64, pub sq: f64, pub hl: f64, pub ss: f64, pub km: f64, pub dfix: f64, pub tsb: f64,
    pub nc: f64, pub l2: f64, pub l2n: f64,
}
impl Weights {
    pub fn core() -> Self {
        Weights { vp: 1.0, held: 1.0, region_unseen: 0.25, final_: 0.0, bg: 1.0, europe_mult: 1.0, asia_mult: 1.0, access: 0.0,
            milops: 1.0, squeeze: 0.0, trap: 3.0, china: 1.5, bad: 0.3, space_ab: 0.5, inf: 0.05, auto_eu: 15.0, trapped: 3.0, h3: 0.0, h6: 0.0,
            samples: 6.0, noise: 1e-6, final_from: 0.0, la: -1.0, tm: 1.0, sq: 0.5, hl: 0.0, ss: 0.0, km: 0.0, dfix: 1.0, tsb: 1.0, nc: 1.0, l2: 0.5, l2n: 4.0 }
    }
    pub fn zero() -> Self {
        Weights { vp: 0.0, held: 0.0, region_unseen: 0.0, final_: 0.0, bg: 0.0, europe_mult: 1.0, asia_mult: 1.0, access: 0.0, milops: 0.0,
            squeeze: 0.0, trap: 0.0, china: 0.0, bad: 0.0, space_ab: 0.0, inf: 0.0, auto_eu: 0.0, trapped: 0.0, h3: 0.0, h6: 0.0, samples: 1.0, noise: 1.0, final_from: 0.0, la: -1.0, tm: 0.0, sq: -1.0, hl: 0.0, ss: 0.0, km: 0.0, dfix: 0.0, tsb: 0.0, nc: 0.0, l2: 0.0, l2n: 4.0 }
    }
    pub fn set(&mut self, k: &str, v: f64) {
        match k {
            "vp" => self.vp = v, "held" => self.held = v, "region_unseen" => self.region_unseen = v, "final" => self.final_ = v,
            "bg" => self.bg = v, "europe_mult" => self.europe_mult = v, "asia_mult" => self.asia_mult = v, "access" => self.access = v,
            "milops" => self.milops = v, "squeeze" => self.squeeze = v, "trap" => self.trap = v, "china" => self.china = v,
            "bad" => self.bad = v, "space_ab" => self.space_ab = v, "inf" => self.inf = v, "auto_eu" => self.auto_eu = v, "trapped" => self.trapped = v,
            "h3" => self.h3 = v, "h6" => self.h6 = v, "samples" => self.samples = v, "noise" => self.noise = v, "final_from" => self.final_from = v,
            "la" => self.la = v, "tm" => self.tm = v, "sq" => self.sq = v, "hl" => self.hl = v, "ss" => self.ss = v, "km" => self.km = v, "dfix" => self.dfix = v, "tsb" => self.tsb = v, "nc" => self.nc = v, "l2" => self.l2 = v, "l2n" => self.l2n = v,
            _ => panic!("unknown weight {k}"),
        }
    }
}

pub fn danger(card: u8) -> f64 {
    match card {
        16 | 23 | 42 | 44 | 50 | 56 => 3.0,
        7 | 14 | 17 | 28 | 30 | 33 | 55 | 58 | 59 | 61 | 62 | 82 | 87 | 90 | 95 | 98 | 99 | 107 | 4 | 5 | 21 | 25 | 26 | 27 | 29 | 63 | 65 | 67 | 71 | 73 | 74 | 76 | 77 | 85 | 86 | 89 | 96 => 2.0,
        _ => 1.0,
    }
}
/// Opponent Events that can lose the game for the holder at DEFCON 2 (HoF trap list).
pub fn trap_for(holder: usize, card: u8) -> bool {
    match CARDS[card as usize].owner() {
        Some(US) => holder == USSR && matches!(card, 4 | 5 | 26 | 67 | 85 | 89 | 96),
        Some(USSR) => holder == US && matches!(card, 50 | 62 | 91),
        _ => false,
    }
}
pub fn scoring_card_of_region(r: u8) -> u8 { [2, 1, 3, 79, 37, 81][r as usize] }

fn card_live(st: &State, k: u8) -> bool {
    if st.removed.contains(k) { return false; }
    let era = CARDS[k as usize].era;
    (era == 0) || (era == 1 && st.turn >= 3) || (era == 2 && st.turn >= 7)
}

pub fn rounds_left(st: &State, me: usize) -> i32 { rounds_for(st, me) as i32 - st.rounds_done[me] as i32 }

// ------------------------------------------------------------------ evaluation
pub fn eval(w: &Weights, st: &State, me: usize) -> f64 {
    if st.winner >= 0 { return if st.winner == me as i8 { 1000.0 } else if st.winner == 2 { 0.0 } else { -1000.0 }; }
    let o = opp(me);
    let sg = if me == US { 1.0 } else { -1.0 };
    let mut v = w.vp * sg * st.vp as f64;
    // deadline: Scoring cards that can no longer be played this turn
    let sc_in_hand = st.hands[me].iter().filter(|&k| CARDS[k as usize].scoring).count() as i32;
    if st.ar > 0 && sc_in_hand > rounds_left(st, me) { v -= 900.0; }
    let ctrl: Vec<i8> = (0..NC).map(|c| st.ctrl(c)).collect();
    // regions
    let mut nets = [0.0f64; 6];
    for r in 0..6u8 {
        let (u, s) = region_score(st, r, None);
        if u >= 1000 || s >= 1000 {
            let mine = if me == US { u >= 1000 } else { s >= 1000 };
            v += if mine { w.auto_eu } else { -w.auto_eu };
            nets[r as usize] = if mine { 10.0 } else { -10.0 };
            continue;
        }
        let net = sg * (u - s) as f64; nets[r as usize] = net;
        let k = scoring_card_of_region(r);
        if st.hands[me].contains(k) { v += w.held * net; } else if card_live(st, k) { v += w.region_unseen * net; }
    }
    if !st.removed.contains(38) {
        let (u, s) = sea_score(st); let net = sg * (u - s) as f64;
        if st.hands[me].contains(38) { v += w.held * net; } else if card_live(st, 38) { v += w.region_unseen * net; }
    }
    if w.final_ != 0.0 && st.turn as f64 >= w.final_from { v += w.final_ * sg * final_projection(st) as f64 * (0.3 + 0.7 * st.turn as f64 / 10.0); }
    // battlegrounds
    let h3 = if me == USSR && st.turn <= 3 { 1.0 + w.h3 } else { 1.0 };
    for c in 0..NC {
        if !COUNTRIES[c].bg || ctrl[c] < 0 { continue; }
        let m = match COUNTRIES[c].region { EUROPE => w.europe_mult, ASIA => w.asia_mult, _ => 1.0 };
        if ctrl[c] == me as i8 { v += w.bg * m * h3 } else { v -= w.bg * m * h3 }
    }
    // near-Control (run 4, Red Sea v6 Egypt lesson): a Battleground one Influence short of Control
    if w.nc != 0.0 {
        for c in 0..NC {
            if !COUNTRIES[c].bg { continue; }
            let m = match COUNTRIES[c].region { EUROPE => w.europe_mult, ASIA => w.asia_mult, _ => 1.0 };
            let st_ = st.stab(c) as i32;
            let short = |a: usize| { let (x, y) = (st.inf[c][a] as i32, st.inf[c][opp(a)] as i32); (st_ - x).max(st_ + y - x) };
            if ctrl[c] != me as i8 && st.inf[c][me] > 0 && short(me) == 1 { v += w.nc * w.bg * m; }
            if ctrl[c] != o as i8 && st.inf[c][o] > 0 && short(o) == 1 { v -= w.nc * w.bg * m; }
        }
    }
    // access
    for r in 0..6u8 {
        if r >= AFRICA && st.turn < 3 { continue; }
        if !st.has_influence_in_region(me, r) { v -= w.access; }
        if !st.has_influence_in_region(o, r) { v += 0.5 * w.access; }
    }
    // military ops
    let rf = |s: usize| (st.rounds_done[s] as f64 / rounds_for(st, s).max(1) as f64);
    v -= w.milops * ((st.defcon - st.milops[me]).max(0) as f64) * (0.25 + 0.75 * rf(me));
    v += w.milops * ((st.defcon - st.milops[o]).max(0) as f64) * (0.25 + 0.75 * rf(o));
    // DEFCON squeeze: locked Regions favour whoever leads them
    if w.squeeze != 0.0 {
        for r in [EUROPE, ASIA, MIDEAST] {
            let locked = (r == EUROPE && st.defcon <= 4) || (r == ASIA && st.defcon <= 3) || (r == MIDEAST && st.defcon <= 2);
            if locked { v += w.squeeze * nets[r as usize].signum(); }
        }
    }
    // hand: liabilities and DEFCON traps
    for k in st.hands[me].iter() {
        if CARDS[k as usize].owner() == Some(o) { v -= w.bad * (1.0 + danger(k) / 2.0); }
        if trap_for(me, k) { v -= w.trap * match st.defcon { 2 => 1.0, 3 => 0.5, 4 => 0.2, _ => 0.1 }; }
    }
    // DEFCON traps I may be forced to play (too few other cards for the rounds left)
    if st.ar > 0 {
        let traps = st.hands[me].iter().filter(|&k| trap_for(me, k)).count() as i32;
        let others = st.hands[me].len() as i32 - traps;
        let forced = (rounds_left(st, me) - others).max(0).min(traps);
        if forced > 0 { v -= w.trap * 3.0 * forced as f64 * match st.defcon { 2 => 1.0, 3 => 0.5, _ => 0.15 }; }
    }
    // Quagmire / Bear Trap on me
    if (me == US && st.ev.quagmire) || (me == USSR && st.ev.bear_trap) {
        v -= w.trapped * (1.0 + sc_in_hand as f64);
        // Trapped with a Scoring card: it can only be played once no eligible discard is left.
        let elig = st.hands[me].iter().filter(|&k| !CARDS[k as usize].scoring && st.card_ops(me, k) >= 2).count() as i32;
        if st.ar > 0 && sc_in_hand > 0 && sc_in_hand + elig.min(1) > rounds_left(st, me) { v -= 900.0; }
    }
    if (o == US && st.ev.quagmire) || (o == USSR && st.ev.bear_trap) { v += w.trapped; }
    // China Card
    if st.china == me as u8 { v += w.china * if st.china_up { 1.0 } else { 0.6 }; } else { v -= w.china * if st.china_up { 1.0 } else { 0.6 }; }
    // Space Race abilities
    for b in [2usize, 4, 6, 8] { if st.space_ability(me, b) { v += w.space_ab } if st.space_ability(o, b) { v -= w.space_ab } }
    // Influence
    if w.inf != 0.0 { let mut d = 0i32; for c in 0..NC { d += (st.inf[c][me] - st.inf[c][o]) as i32; } v += w.inf * d as f64; }
    v
}

/// Re-deal the opponent's unknown cards from everything `me` has not seen; reshuffle the deck.
pub fn determinize(st: &State, me: usize, rng: &mut Streams) -> State {
    let mut s = *st;
    let o = opp(me);
    let known = st.known_by_opp[o];
    let mut pool: Vec<u8> = st.deck.iter().collect();
    let mut keep: Vec<u8> = vec![];
    for k in st.hands[o].iter() { if known.contains(k) { keep.push(k) } else { pool.push(k) } }
    rng.shuffle(Purpose::Sim, me, &mut pool);
    let need = st.hands[o].len() - keep.len();
    s.hands[o].clear();
    for &k in &keep { s.hands[o].push(k); }
    for &k in pool.iter().take(need) { s.hands[o].push(k); }
    s.deck.clear();
    for &k in pool.iter().skip(need) { s.deck.push(k); }
    s
}

// ------------------------------------------------------------------ brain (greedy choices)
#[derive(Clone)]
pub struct Brain { pub w: Weights }

impl Brain {
    pub fn e(&self, st: &State, me: usize) -> f64 { eval(&self.w, st, me) }

    pub fn greedy_place(&self, st: &State, me: usize, mut left: i32, scope: Scope) -> State {
        let mut s = *st;
        loop {
            let mut best: Option<(f64, usize)> = None;
            for c in 0..NC {
                if !can_place(&s, me, c, scope) { continue; }
                let cost = place_cost(&s, me, c);
                if cost > left { continue; }
                let mut t = s; t.inf[c][me] += 1;
                let v = self.e(&t, me);
                if best.map_or(true, |(b, _)| v > b) { best = Some((v, c)); }
            }
            match best { None => break, Some((_, c)) => { left -= place_cost(&s, me, c); s.inf[c][me] += 1; } }
            if left <= 0 { break; }
        }
        s
    }

    pub fn realign_ev(&self, st: &State, me: usize, c: usize) -> f64 {
        let o = opp(me);
        let ma = realign_mod(st, me, c); let mb = realign_mod(st, o, c);
        let mut tot = 0.0;
        for a in 1..=6 { for b in 1..=6 {
            let d = (a + ma) - (b + mb);
            let mut s = *st;
            if d > 0 { s.inf[c][o] = (s.inf[c][o] - d as i8).max(0); }
            if d < 0 { s.inf[c][me] = (s.inf[c][me] + d as i8).max(0); }
            tot += self.e(&s, me);
        } }
        tot / 36.0
    }

    /// Realignment by `side` at `c`, valued on `me`'s evaluation.
    pub fn realign_ev_for(&self, st: &State, side: usize, c: usize, me: usize) -> f64 {
        let o = opp(side);
        let ma = realign_mod(st, side, c); let mb = realign_mod(st, o, c);
        let mut tot = 0.0;
        for a in 1..=6 { for b in 1..=6 {
            let d = (a + ma) - (b + mb);
            let mut s = *st;
            if d > 0 { s.inf[c][o] = (s.inf[c][o] - d as i8).max(0); }
            if d < 0 { s.inf[c][side] = (s.inf[c][side] + d as i8).max(0); }
            tot += self.e(&s, me);
        } }
        tot / 36.0
    }

    /// Best use of Ops: (expected value, choice).
    pub fn best_ops(&self, st: &State, me: usize, spec: &OpsSpec) -> (f64, OpsChoice) {
        let mut best = (f64::NEG_INFINITY, OpsChoice::None);
        if spec.allow_place {
            let mut scopes = vec![Scope::Any];
            if spec.card == CHINA { scopes.push(Scope::Asia); }
            if spec.card != 0 && ((me == USSR && st.ev.vietnam_revolts) || spec.card == CHINA) { scopes.push(Scope::Sea); }
            for sc in scopes {
                let n = spec.base + scope_bonus(st, me, spec.card, sc);
                let s = self.greedy_place(st, me, n, sc);
                let v = self.e(&s, me);
                if v > best.0 { best = (v, OpsChoice::Place(sc)); }
            }
        }
        if spec.allow_coup {
            for c in 0..NC {
                if !can_coup(st, me, c, false) { continue; }
                let sc = if COUNTRIES[c].region == ASIA { if COUNTRIES[c].sea { Scope::Sea } else { Scope::Asia } } else { Scope::Any };
                let ops = spec.base + scope_bonus(st, me, spec.card, sc);
                let mut tot = 0.0;
                for d in 1..=6 { tot += self.e(&coup_result(st, me, c, ops, false, d), me); }
                let v = tot / 6.0;
                if v > best.0 { best = (v, OpsChoice::Coup(c)); }
            }
        }
        if spec.allow_realign {
            let base = self.e(st, me);
            let mut gain = f64::NEG_INFINITY;
            for c in 0..NC { if can_realign(st, me, c, false) { let g = self.realign_ev(st, me, c) - base; if g > gain { gain = g; } } }
            if gain > f64::NEG_INFINITY {
                let v = base + gain * spec.base as f64 * 0.8;
                if v > best.0 { best = (v, OpsChoice::Realign(Scope::Any)); }
            }
        }
        if spec.allow_space && spec.card != 0 && can_space(st, me, spec.card) {
            let b = st.space[me] as usize + 1;
            let p = SPACE[b].1 as f64 / 6.0;
            let mut ok = *st; space_advance(&mut ok, me, 1, false);
            let v = p * self.e(&ok, me) + (1.0 - p) * self.e(st, me);
            if v > best.0 { best = (v, OpsChoice::Space); }
        }
        if best.0 == f64::NEG_INFINITY { best.0 = self.e(st, me); }
        best
    }

    pub fn argmax(&self, vals: &[f64], rng: &mut Streams, me: usize) -> usize {
        let mut bi = 0; let mut bv = f64::NEG_INFINITY;
        for (i, &v) in vals.iter().enumerate() {
            let v = v + self.w.noise * (rng.next(Purpose::Bot, me) % 1000) as f64 / 1000.0;
            if v > bv { bv = v; bi = i; }
        }
        bi
    }
}

// ------------------------------------------------------------------ greedy agent (used inside simulations)
pub struct Greedy { pub b: Brain }
impl Agent for Greedy {
    fn name(&self) -> String { "greedy".into() }
    fn headline(&mut self, st: &State, me: usize, _o: Option<u8>, _r: &mut Streams) -> u8 {
        st.hands[me].iter().find(|&k| k != 32).unwrap_or(st.hands[me].c[0])
    }
    fn action(&mut self, st: &State, me: usize, _r: &mut Streams) -> Play {
        let k = st.hands[me].iter().next().unwrap_or(CHINA);
        Play { card: k, how: if CARDS[k as usize].scoring { Use::Event } else { Use::Ops }, event_first: false, uni_with: 0 }
    }
    fn ops(&mut self, st: &State, me: usize, spec: &OpsSpec, _r: &mut Streams) -> OpsChoice { self.b.best_ops(st, me, spec).1 }
    fn choose(&mut self, st: &State, me: usize, opts: &[State], r: &mut Streams) -> usize { let _ = st; let v: Vec<f64> = opts.iter().map(|s| self.b.e(s, me)).collect(); self.b.argmax(&v, r, me) }
    fn choose_lottery(&mut self, _st: &State, me: usize, opts: &[Vec<(f64, State)>], r: &mut Streams) -> usize {
        let v: Vec<f64> = opts.iter().map(|l| l.iter().map(|(p, s)| p * self.b.e(s, me)).sum()).collect(); self.b.argmax(&v, r, me)
    }
    fn realign_target(&mut self, st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { let v: Vec<f64> = t.iter().map(|&c| self.b.realign_ev(st, me, c)).collect(); self.b.argmax(&v, r, me) }
    fn place_target(&mut self, st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { let v: Vec<f64> = t.iter().map(|&c| { let mut s = *st; s.inf[c][me] += 1; self.b.e(&s, me) }).collect(); self.b.argmax(&v, r, me) }
    fn remove_target(&mut self, st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { let o = opp(me); let v: Vec<f64> = t.iter().map(|&c| { let mut s = *st; s.inf[c][o] -= 1; self.b.e(&s, me) }).collect(); self.b.argmax(&v, r, me) }
}

// ------------------------------------------------------------------ candidate plays
pub fn candidates(st: &State, me: usize, w: &Weights) -> Vec<Play> {
    let o = opp(me);
    let mut out = vec![];
    let left = rounds_left(st, me);
    let scoring: Vec<u8> = st.hands[me].iter().filter(|&k| CARDS[k as usize].scoring).collect();
    let must_score = !scoring.is_empty() && scoring.len() as i32 >= left;
    let mut cards: Vec<u8> = st.hands[me].iter().collect();
    if st.china_playable(me) { cards.push(CHINA); }
    for &k in &cards {
        let c = &CARDS[k as usize];
        if c.scoring { out.push(Play { card: k, how: Use::Event, event_first: false, uni_with: 0 }); continue; }
        if must_score { continue; }
        if k != CHINA && k != 32 && c.owner() != Some(o) && can_occur(st, k, me) { out.push(Play { card: k, how: Use::Event, event_first: false, uni_with: 0 }); }
        if c.owner() == Some(o) && can_occur(st, k, o) {
            out.push(Play { card: k, how: Use::Ops, event_first: true, uni_with: 0 });
            out.push(Play { card: k, how: Use::Ops, event_first: false, uni_with: 0 });
        } else { out.push(Play { card: k, how: Use::Ops, event_first: false, uni_with: 0 }); }
        if can_space(st, me, k) && !(w.h6 > 0.0 && c.owner() != Some(o)) { out.push(Play { card: k, how: Use::Space, event_first: false, uni_with: 0 }); }
        if k == 32 {
            for &x in &cards { if x != 32 && x != CHINA && CARDS[x as usize].owner() == Some(o) && can_occur(st, x, o) { out.push(Play { card: 32, how: Use::Ops, event_first: false, uni_with: x }); } }
        }
    }
    if out.is_empty() && st.china_playable(me) && st.hands[me].is_empty() { out.push(Play { card: CHINA, how: Use::Pass, event_first: false, uni_with: 0 }); }
    // the 8th Action Round (Space Station, North Sea Oil) is optional: "may play 8" (HoF rules, card 86)
    let base_rounds = if st.turn <= 3 { 6 } else { 7 };
    if st.rounds_done[me] as i32 >= base_rounds && !st.hands[me].iter().any(|k| CARDS[k as usize].scoring) { out.push(Play { card: st.hands[me].iter().next().unwrap_or(CHINA), how: Use::Pass, event_first: false, uni_with: 0 }); }
    out
}

// ------------------------------------------------------------------ evaluation bot
pub struct EvalBot { pub b: Brain, pub label: String }

impl EvalBot {
    pub fn new(w: Weights, label: &str) -> Self { EvalBot { b: Brain { w }, label: label.into() } }

    /// Simulate `play` from `base` (determinized) and return the expected value.
    fn sim_value(&self, base: &State, me: usize, p: Play, rng: &mut Streams) -> f64 {
        let k = self.b.w.samples.max(1.0) as usize;
        // Pure Ops plays of own/neutral cards: exact expectation without simulation.
        let o = opp(me);
        let opp_event = CARDS[p.card as usize].owner() == Some(o) && can_occur(base, p.card, o);
        if p.how == Use::Ops && !opp_event && p.uni_with == 0 && p.card != 49 {
            let mut s = *base;
            if p.card == CHINA { s.china = o as u8; s.china_up = false; if me == US { s.ev.formosan = false; } } else { s.remove_from_hand(me, p.card); s.discard.push(p.card); }
            if p.card == 103 && me == USSR { s.add_vp(US, 1); }
            if me == US && is_war(p.card) && s.ev.flower_power && !s.ev.evil_empire { s.add_vp(USSR, 2); }
            s.rounds_done[me] += 1;
            let base_ops = base.card_ops(me, p.card);
            return self.b.best_ops(&s, me, &ops_spec(&s, me, p.card, base_ops)).0;
        }
        if p.how == Use::Space {
            let mut s = *base;
            if p.card == CHINA { s.china = o as u8; s.china_up = false; } else { s.remove_from_hand(me, p.card); s.discard.push(p.card); }
            s.rounds_done[me] += 1; s.space_attempts[me] += 1;
            let b = s.space[me] as usize + 1;
            let pr = SPACE[b].1 as f64 / 6.0;
            let mut ok = s; space_advance(&mut ok, me, 1, false);
            return pr * self.b.e(&ok, me) + (1.0 - pr) * self.b.e(&s, me);
        }
        let mut tot = 0.0; let mut n = 0;
        let mut kk = k; let mut i = 0;
        while i < kk {
            let mut s = *base;
            let mut g0 = Greedy { b: self.b.clone() }; let mut g1 = Greedy { b: self.b.clone() };
            let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1);
            cx.sim = true;
            execute_play(&mut s, &mut cx, me, p);
            s.rounds_done[me] += 1;
            let v = self.b.e(&s, me);
            if s.winner == opp(me) as i8 { return v; } // hard filter: a play that can lose on the spot
            tot += v; n += 1;
            if i == 0 && cx.dice_used == 0 && (cx.random_used == 0 || self.b.w.dfix <= 0.0) { break; }
            if i == 0 && cx.random_used > 0 && self.b.w.dfix > 0.0 { kk = 3 * k; }
            i += 1;
        }
        tot / n as f64
    }
}

impl EvalBot {
    /// One sampled resulting position of `p` (for the reply scan and set-up values).
    fn sample_state(&self, base: &State, me: usize, p: Play, rng: &mut Streams) -> State {
        let mut s = *base;
        let mut g0 = Greedy { b: self.b.clone() }; let mut g1 = Greedy { b: self.b.clone() };
        let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1);
        cx.sim = true;
        execute_play(&mut s, &mut cx, me, p);
        s.rounds_done[me] += 1;
        s
    }

    /// Headline v2: each card valued against the opponent's likely Headline (best-of-hand odds over
    /// the cards I have not seen, resolution order, Defectors), minus its Action Round value lost.
    fn headline_v2(&self, st: &State, me: usize, shown: Option<u8>, rng: &mut Streams) -> u8 {
        let o = opp(me);
        let base = determinize(st, me, rng);
        let mut mine: Vec<u8> = st.hands[me].iter().filter(|&k| k != 32).collect();
        if self.b.w.tsb > 0.0 { let safe: Vec<u8> = mine.iter().copied().filter(|&k| !defcon_forbidden_headline(st, me, k)).collect(); if !safe.is_empty() { mine = safe; } }
        if mine.len() <= 1 { return mine.first().copied().unwrap_or(st.hands[me].c[0]); }
        // Action Round value of each card (best play now), and the leftover card
        let mut b1 = base; b1.ar = 1; b1.phasing = me as u8; b1.snapshot_presence();
        let v0 = self.b.e(&b1, me);
        let cands = candidates(&b1, me, &self.b.w);
        let mut arval = vec![f64::NEG_INFINITY; mine.len()];
        for p in &cands {
            if let Some(i) = mine.iter().position(|&k| k == p.card) {
                let v = self.sim_value(&b1, me, *p, rng) - v0;
                if v > arval[i] { arval[i] = v; }
            }
        }
        let n_ar = rounds_for(st, me) as usize;
        let leftover = if mine.len() > n_ar + 1 {
            (0..mine.len()).filter(|&i| !CARDS[mine[i] as usize].scoring).min_by(|&a, &b| arval[a].partial_cmp(&arval[b]).unwrap())
        } else { None };
        // the opponent's likely Headlines
        let mut opts: Vec<(u8, f64)> = vec![];
        let (pool_cards, h, known_cards): (Vec<u8>, usize, Vec<u8>) = if let Some(k) = shown { (vec![], 0, vec![k]) } else {
            let known: Vec<u8> = base.hands[o].iter().filter(|&k| st.known_by_opp[o].contains(k)).collect();
            let unseen: Vec<u8> = base.deck.iter().chain(base.hands[o].iter().filter(|&k| !st.known_by_opp[o].contains(k))).collect();
            (unseen, base.hands[o].len() - known.len(), known)
        };
        let hval = |k: u8, rng: &mut Streams| -> f64 {
            if k == 32 || k == CHINA || CARDS[k as usize].owner() == Some(me) { return f64::NEG_INFINITY; }
            let mut t = base; t.phasing = o as u8; t.ar = 0;
            let mut g0 = Greedy { b: self.b.clone() }; let mut g1 = Greedy { b: self.b.clone() };
            let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1); cx.sim = true;
            play_event(&mut t, &mut cx, o, k, true);
            self.b.e(&t, o)
        };
        let mut pool: Vec<(f64, u8)> = pool_cards.iter().map(|&k| (hval(k, rng), k)).filter(|x| x.0 > f64::NEG_INFINITY).collect();
        pool.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let kbest = known_cards.iter().map(|&k| (hval(k, rng), k)).filter(|x| x.0 > f64::NEG_INFINITY).max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let u = pool.len();
        if h > 0 && u > 0 {
            let hh = h.min(u); let mut p = hh as f64 / u as f64; let mut mass = 0.0;
            for i in 0..u {
                if let Some(kb) = kbest { if pool[i].0 <= kb.0 { break; } }
                opts.push((pool[i].1, p)); mass += p;
                if mass > 0.95 || opts.len() >= 8 || u - i <= hh { break; }
                p *= (u - i - hh) as f64 / (u - i - 1) as f64;
            }
            if let Some(kb) = kbest { opts.push((kb.1, (1.0 - mass).max(0.0))); }
        } else if let Some(kb) = kbest { opts.push((kb.1, 1.0)); }
        let tot: f64 = opts.iter().map(|x| x.1).sum();
        if tot <= 0.0 { opts = vec![(0, 1.0)]; } else { for x in opts.iter_mut() { x.1 /= tot; } }
        // value each of my Headlines
        let any_non_sc = mine.iter().any(|&k| !CARDS[k as usize].scoring);
        let mut best = (f64::NEG_INFINITY, mine[0]);
        for (i, &k) in mine.iter().enumerate() {
            if CARDS[k as usize].scoring && any_non_sc { continue; }
            let mut ev = 0.0;
            for &(ok, p) in &opts {
                let mut t = base; t.remove_from_hand(me, k); t.ar = 0;
                if ok != 0 { t.remove_from_hand(o, ok); }
                let mut g0 = Greedy { b: self.b.clone() }; let mut g1 = Greedy { b: self.b.clone() };
                let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1); cx.sim = true;
                let mut h2 = [0u8; 2]; h2[me] = k; h2[o] = ok;
                let mut cancelled = [false; 2];
                if h2[US] == 103 && h2[USSR] != 0 { cancelled[USSR] = true; }
                let ops = |c: u8| if c == 0 { -1 } else { CARDS[c as usize].ops as i32 };
                let order = if ops(h2[USSR]) > ops(h2[US]) { [USSR, US] } else { [US, USSR] };
                for sd in order {
                    if h2[sd] == 0 || cancelled[sd] || t.winner >= 0 || h2[sd] == 103 { continue; }
                    t.phasing = sd as u8;
                    play_event(&mut t, &mut cx, sd, h2[sd], true);
                }
                ev += p * self.b.e(&t, me);
            }
            let cost = match leftover { Some(l) if l != i && arval[i] > f64::NEG_INFINITY && arval[l] > f64::NEG_INFINITY => (arval[i] - arval[l]).max(0.0), _ => 0.0 };
            let v = ev - cost;
            if std::env::var("TSSIM_DEBUG2").is_ok() { eprintln!("HL T{} {} {:28} ev {:.2} cost {:.2}", st.turn, side_name(me), CARDS[k as usize].name, ev - v0, cost); }
            if v > best.0 { best = (v, k); }
        }
        best.1
    }

    /// Best one-ply play for `side` on `st` (one sample per play), DEFCON limits applied: (play, value for side).
    fn best1(&self, b1: &EvalBot, st: &State, side: usize, rng: &mut Streams) -> Option<(Play, f64)> {
        let mut s = *st; s.phasing = side as u8; s.snapshot_presence();
        let mut c = candidates(&s, side, &b1.b.w);
        if b1.b.w.tsb > 0.0 { let safe: Vec<Play> = c.iter().copied().filter(|p| !defcon_forbidden_ar(&s, side, p)).collect(); if !safe.is_empty() { c = safe; } }
        let mut best: Option<(Play, f64)> = None;
        for p in c { let v = b1.sim_value(&s, side, p, rng); if best.map_or(true, |x| v > x.1) { best = Some((p, v)); } }
        best
    }

    /// Selective two-move look-ahead (run 4, Red Sea v6): my move, the opponent's best reply on a re-dealt hand,
    /// then my best next move; averaged over l2n samples. Returns the value relative to v0.
    fn two_move(&self, base: &State, me: usize, p: Play, v0: f64, rng: &mut Streams) -> f64 {
        let o = opp(me);
        let mut w1 = self.b.w.clone(); w1.samples = 1.0; w1.noise = 0.0; let b1 = EvalBot::new(w1, "l2");
        let n = self.b.w.l2n.max(1.0) as usize;
        let mut tot = 0.0;
        for _ in 0..n {
            let s1 = self.sample_state(base, me, p, rng);
            if s1.winner >= 0 { tot += self.b.e(&s1, me) - v0; continue; }
            let mut s2 = s1;
            let opp_left = rounds_for(&s1, o) as i32 - s1.rounds_done[o] as i32;
            if opp_left > 0 && (!s1.hands[o].is_empty() || s1.china_playable(o)) {
                let so = determinize(&s1, me, rng);
                if let Some((po, _)) = self.best1(&b1, &so, o, rng) {
                    let mut t = so; t.phasing = o as u8; t.snapshot_presence();
                    let mut g0 = Greedy { b: b1.b.clone() }; let mut g1 = Greedy { b: b1.b.clone() };
                    let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1); cx.sim = true;
                    execute_play(&mut t, &mut cx, o, po);
                    t.rounds_done[o] += 1;
                    s2 = t;
                }
            }
            if s2.winner >= 0 { tot += self.b.e(&s2, me) - v0; continue; }
            let my_left = rounds_for(&s2, me) as i32 - s2.rounds_done[me] as i32;
            let v = if my_left > 0 && (!s2.hands[me].is_empty() || s2.china_playable(me)) {
                self.best1(&b1, &s2, me, rng).map_or(self.b.e(&s2, me), |x| x.1)
            } else { self.b.e(&s2, me) };
            tot += v - v0;
        }
        tot / n as f64
    }

    /// Run 2 decision rules on top of the one-move values (see search.rs).
    pub fn decide(&self, st: &State, base: &State, me: usize, c: &[Play], vals: &[f64], rng: &mut Streams) -> usize {
        let w = &self.b.w;
        let g = self.b.argmax(vals, rng, me);
        // shortlist: best play of each of the top 3 cards, plus Scoring cards when timing is on
        let mut order: Vec<usize> = (0..c.len()).collect();
        order.sort_by(|&a, &b| vals[b].partial_cmp(&vals[a]).unwrap());
        let mut set: Vec<usize> = vec![]; let mut cards: Vec<u8> = vec![];
        for &i in &order { if vals[i] < -500.0 { continue; } if !cards.contains(&c[i].card) { cards.push(c[i].card); set.push(i); if set.len() == 3 { break; } } }
        if !set.contains(&g) { set.insert(0, g); }
        let is_sc = |i: usize| CARDS[c[i].card as usize].scoring;
        if w.tm > 0.0 { for i in 0..c.len() { if is_sc(i) && !set.contains(&i) && vals[i] > -500.0 { set.push(i); } } }
        if set.len() < 2 && w.ss <= 0.0 { return g; }
        let v0 = self.b.e(base, me);
        let best_other = (0..c.len()).filter(|&i| !is_sc(i)).map(|i| vals[i]).fold(f64::NEG_INFINITY, f64::max);
        let scan_on = w.la >= 0.0 || w.tm > 0.0 || w.sq >= 0.0;
        let o = opp(me);
        let mut r = vec![0.0; c.len()]; let mut seq = vec![f64::NEG_INFINITY; c.len()];
        for &i in &set {
            let s = self.sample_state(base, me, c[i], rng);
            let credit = if w.tm > 0.0 && is_sc(i) && best_other > f64::NEG_INFINITY { (best_other - v0).max(0.0) } else { 0.0 };
            let su = setup_credit(&self.b, &s, me, rng);
            let (tn, tl) = if scan_on { let sc = reply_scan(&self.b, &s, me, rng); (sc.threat_now(), sc.threat_later()) } else { (0.0, 0.0) };
            r[i] = vals[i] + su + credit - tn;
            let d = if w.km > 0.0 && c[i].card != CHINA && st.card_known(me, c[i].card) && rounds_for(st, o) as i32 - st.rounds_done[o] as i32 > 0 { 0.0 } else { w.sq };
            seq[i] = (1.0 - d) * (vals[i] + su + credit - v0) - tn + d * tl;
            if std::env::var("TSSIM_DEBUG2").is_ok() { eprintln!("T{} AR{} {} {:28} {:?} val {:+.2} setup {:.2} credit {:.2} threat now {:.2} later {:.2} -> r {:.2} seq {:.2}", st.turn, st.ar, side_name(me), CARDS[c[i].card as usize].name, c[i].how, vals[i]-v0, su, credit, tn, tl, r[i]-v0, seq[i]); }
        }
        if w.sq >= 0.0 && w.l2 > 0.0 {
            for &i in &set { let t = self.two_move(base, me, c[i], v0, rng); seq[i] = (1.0 - w.l2) * seq[i] + w.l2 * t; }
        }
        if w.sq >= 0.0 { return *set.iter().max_by(|&&a, &&b| seq[a].partial_cmp(&seq[b]).unwrap()).unwrap(); }
        let mut pick = g;
        if w.la >= 0.0 || w.ss > 0.0 {
            let alt = *set.iter().max_by(|&&a, &&b| r[a].partial_cmp(&r[b]).unwrap()).unwrap();
            let m = if w.la >= 0.0 { w.la } else { 0.0 };
            if r[alt] > r[pick] + m { pick = alt; }
        }
        if w.tm > 0.0 && !is_sc(pick) {
            if let Some(&sc) = set.iter().filter(|&&i| is_sc(i)).max_by(|&&a, &&b| r[a].partial_cmp(&r[b]).unwrap()) { if r[sc] > r[pick] { pick = sc; } }
        }
        pick
    }
}

impl Agent for EvalBot {
    fn name(&self) -> String { self.label.clone() }
    fn headline(&mut self, st: &State, me: usize, shown: Option<u8>, rng: &mut Streams) -> u8 {
        let hl = self.b.w.hl as i32;
        if hl == 1 || (hl == 2 && me == USSR) || (hl == 3 && me == US) { return self.headline_v2(st, me, shown, rng); }
        let base = determinize(st, me, rng);
        let mut cards: Vec<u8> = st.hands[me].iter().filter(|&k| k != 32).collect();
        if self.b.w.tsb > 0.0 { let safe: Vec<u8> = cards.iter().copied().filter(|&k| !defcon_forbidden_headline(st, me, k)).collect(); if !safe.is_empty() { cards = safe; } }
        // DEFCON fix: the opponent's Headline may lower DEFCON first, then a boycott or Summit result ends the game
        if self.b.w.dfix > 0.0 && st.defcon <= 3 { let safe: Vec<u8> = cards.iter().copied().filter(|&k| k != 20 && k != 45).collect(); if !safe.is_empty() { cards = safe; } }
        if cards.is_empty() { return st.hands[me].c[0]; }
        let now = self.b.e(st, me);
        let mut vals = vec![];
        for &k in &cards {
            if k == 103 { vals.push(if me == US { now + 1.5 } else { now - 1.0 }); continue; }
            let samples = self.b.w.samples.max(1.0) as usize;
            let mut tot = 0.0; let mut n = 0;
            for i in 0..samples {
                let mut s = base; s.remove_from_hand(me, k); s.phasing = me as u8;
                let mut g0 = Greedy { b: self.b.clone() }; let mut g1 = Greedy { b: self.b.clone() };
                let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1); cx.sim = true;
                play_event(&mut s, &mut cx, me, k, true);
                tot += self.b.e(&s, me); n += 1;
                if i == 0 && cx.dice_used == 0 { break; }
            }
            vals.push(tot / n as f64);
        }
        cards[self.b.argmax(&vals, rng, me)]
    }
    fn action(&mut self, st: &State, me: usize, rng: &mut Streams) -> Play {
        let mut c = candidates(st, me, &self.b.w);
        if self.b.w.tsb > 0.0 {
            let safe: Vec<Play> = c.iter().copied().filter(|p| !defcon_forbidden_ar(st, me, p)).collect();
            if !safe.is_empty() { c = safe; }
            if let Some(d) = forced_disposal(st, me, &c) { c = d; }
        }
        if c.len() == 1 { return c[0]; }
        let base = determinize(st, me, rng);
        let vals: Vec<f64> = c.iter().map(|&p| self.sim_value(&base, me, p, rng)).collect();
        let w = &self.b.w;
        if w.la >= 0.0 || w.tm > 0.0 || w.sq >= 0.0 || w.ss > 0.0 {
            return c[self.decide(st, &base, me, &c, &vals, rng)];
        }
        if std::env::var("TSSIM_DEBUG").is_ok() {
            eprintln!("T{} AR{} {} hand {:?}", st.turn, st.ar, side_name(me), st.hands[me].iter().map(|k| CARDS[k as usize].name).collect::<Vec<_>>());
            for (p, v) in c.iter().zip(&vals) { eprintln!("   {:8.2} {} {:?} ef={} uni={}", v, CARDS[p.card as usize].name, p.how, p.event_first, p.uni_with); }
        }
        c[self.b.argmax(&vals, rng, me)]
    }
    fn ops(&mut self, st: &State, me: usize, spec: &OpsSpec, _r: &mut Streams) -> OpsChoice { self.b.best_ops(st, me, spec).1 }
    fn choose(&mut self, st: &State, me: usize, opts: &[State], r: &mut Streams) -> usize { let _ = st; let v: Vec<f64> = opts.iter().map(|s| self.b.e(s, me)).collect(); self.b.argmax(&v, r, me) }
    fn choose_lottery(&mut self, _st: &State, me: usize, opts: &[Vec<(f64, State)>], r: &mut Streams) -> usize {
        let v: Vec<f64> = opts.iter().map(|l| l.iter().map(|(p, s)| p * self.b.e(s, me)).sum()).collect(); self.b.argmax(&v, r, me)
    }
    fn realign_target(&mut self, st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { let v: Vec<f64> = t.iter().map(|&c| self.b.realign_ev(st, me, c)).collect(); self.b.argmax(&v, r, me) }
    fn place_target(&mut self, st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { let v: Vec<f64> = t.iter().map(|&c| { let mut s = *st; s.inf[c][me] += 1; self.b.e(&s, me) }).collect(); self.b.argmax(&v, r, me) }
    fn remove_target(&mut self, st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { let o = opp(me); let v: Vec<f64> = t.iter().map(|&c| { let mut s = *st; s.inf[c][o] -= 1; self.b.e(&s, me) }).collect(); self.b.argmax(&v, r, me) }
}

// ------------------------------------------------------------------ random agent
pub struct RandomBot;
impl Agent for RandomBot {
    fn name(&self) -> String { "random".into() }
    fn headline(&mut self, st: &State, me: usize, _o: Option<u8>, r: &mut Streams) -> u8 {
        let c: Vec<u8> = st.hands[me].iter().filter(|&k| k != 32).collect();
        if c.is_empty() { st.hands[me].c[0] } else { c[r.below(Purpose::Bot, me, c.len())] }
    }
    fn action(&mut self, st: &State, me: usize, r: &mut Streams) -> Play {
        // Uniform among plays that do not lose on the spot (checked on one random continuation).
        let mut c = candidates(st, me, &Weights::core());
        while c.len() > 1 {
            let i = r.below(Purpose::Bot, me, c.len());
            let mut s = *st; let mut a0 = RandomBot; let mut a1 = RandomBot;
            let mut cx = Ctx::new(r.next(Purpose::Sim, me), &mut a0, &mut a1); cx.sim = true;
            execute_play(&mut s, &mut cx, me, c[i]);
            if s.winner == opp(me) as i8 { c.swap_remove(i); } else { return c[i]; }
        }
        c[0]
    }
    fn ops(&mut self, st: &State, me: usize, spec: &OpsSpec, r: &mut Streams) -> OpsChoice {
        let mut o = vec![];
        if spec.allow_place { o.push(OpsChoice::Place(Scope::Any)); }
        if spec.allow_coup { for c in 0..NC { if can_coup(st, me, c, false) && !(COUNTRIES[c].bg && st.defcon <= 2) && st.ev.cmc != Some(opp(me) as u8) { o.push(OpsChoice::Coup(c)); } } }
        if spec.allow_realign && (0..NC).any(|c| can_realign(st, me, c, false)) { o.push(OpsChoice::Realign(Scope::Any)); }
        if o.is_empty() { return OpsChoice::None; }
        o.swap_remove(r.below(Purpose::Bot, me, o.len()))
    }
    fn choose(&mut self, _st: &State, me: usize, opts: &[State], r: &mut Streams) -> usize { r.below(Purpose::Bot, me, opts.len()) }
    fn choose_lottery(&mut self, _st: &State, me: usize, opts: &[Vec<(f64, State)>], r: &mut Streams) -> usize { r.below(Purpose::Bot, me, opts.len()) }
    fn realign_target(&mut self, _st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { r.below(Purpose::Bot, me, t.len()) }
    fn place_target(&mut self, _st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { r.below(Purpose::Bot, me, t.len()) }
    fn remove_target(&mut self, _st: &State, me: usize, t: &[usize], r: &mut Streams) -> usize { r.below(Purpose::Bot, me, t.len()) }
}

/// Build an agent from a spec: "random", "zero", "core", or "core:key=val,key=val".
pub fn make_agent(spec: &str) -> Box<dyn Agent + Send> {
    if spec == "random" { return Box::new(RandomBot); }
    let (head, rest) = spec.split_once(':').unwrap_or((spec, ""));
    let mut w = match head { "zero" => Weights::zero(), "core" => Weights::core(), _ => panic!("unknown bot {head}") };
    for kv in rest.split(',').filter(|x| !x.is_empty()) { let (k, v) = kv.split_once('=').expect("key=val"); w.set(k, v.parse().expect("number")); }
    Box::new(EvalBot::new(w, spec))
}

// ------------------------------------------------------------------ decision-change pre-check (run 4)
/// Plays as `base`; at every Headline and Action Round also asks `alt` on a copy of the random streams
/// and counts how often the two would choose differently.
pub struct PrecheckAgent { pub base: Box<dyn Agent + Send>, pub alt: Box<dyn Agent + Send>, pub counts: std::sync::Arc<std::sync::Mutex<[u64; 4]>> }
impl Agent for PrecheckAgent {
    fn name(&self) -> String { format!("precheck({})", self.base.name()) }
    fn headline(&mut self, st: &State, me: usize, shown: Option<u8>, rng: &mut Streams) -> u8 {
        let mut r2 = rng.clone();
        let a = self.base.headline(st, me, shown, rng); let b = self.alt.headline(st, me, shown, &mut r2);
        let mut c = self.counts.lock().unwrap(); c[0] += 1; if a != b { c[1] += 1; }
        a
    }
    fn action(&mut self, st: &State, me: usize, rng: &mut Streams) -> Play {
        let mut r2 = rng.clone();
        let a = self.base.action(st, me, rng); let b = self.alt.action(st, me, &mut r2);
        let same = a.card == b.card && a.how == b.how && a.uni_with == b.uni_with;
        let mut c = self.counts.lock().unwrap(); c[2] += 1; if !same { c[3] += 1; }
        a
    }
    fn ops(&mut self, st: &State, me: usize, spec: &OpsSpec, rng: &mut Streams) -> OpsChoice { self.base.ops(st, me, spec, rng) }
    fn choose(&mut self, st: &State, me: usize, opts: &[State], rng: &mut Streams) -> usize { self.base.choose(st, me, opts, rng) }
    fn choose_lottery(&mut self, st: &State, me: usize, opts: &[Vec<(f64, State)>], rng: &mut Streams) -> usize { self.base.choose_lottery(st, me, opts, rng) }
    fn realign_target(&mut self, st: &State, me: usize, t: &[usize], rng: &mut Streams) -> usize { self.base.realign_target(st, me, t, rng) }
    fn place_target(&mut self, st: &State, me: usize, t: &[usize], rng: &mut Streams) -> usize { self.base.place_target(st, me, t, rng) }
    fn remove_target(&mut self, st: &State, me: usize, t: &[usize], rng: &mut Streams) -> usize { self.base.remove_target(st, me, t, rng) }
}
