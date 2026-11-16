//! Bots: random, and the evaluation bot (Stage 2 procedure).
use crate::agent::*;
use crate::data::*;
use crate::engine::*;
use crate::events::can_occur;
use crate::rng::{Purpose, Streams};
use crate::state::*;
use crate::types::*;

// ------------------------------------------------------------------ weights
#[derive(Clone, Debug)]
pub struct Weights {
    pub vp: f64, pub held: f64, pub region_unseen: f64, pub final_: f64, pub bg: f64, pub europe_mult: f64, pub asia_mult: f64,
    pub access: f64, pub milops: f64, pub squeeze: f64, pub trap: f64, pub china: f64, pub bad: f64, pub space_ab: f64,
    pub inf: f64, pub auto_eu: f64, pub trapped: f64, pub h3: f64, pub h6: f64, pub samples: f64, pub noise: f64,
}
impl Weights {
    pub fn core() -> Self {
        Weights { vp: 1.0, held: 1.0, region_unseen: 0.25, final_: 0.0, bg: 1.0, europe_mult: 1.0, asia_mult: 1.0, access: 3.0,
            milops: 1.0, squeeze: 0.0, trap: 3.0, china: 0.0, bad: 0.3, space_ab: 0.5, inf: 0.05, auto_eu: 15.0, trapped: 3.0, h3: 0.0, h6: 0.0,
            samples: 6.0, noise: 1e-6 }
    }
    pub fn zero() -> Self {
        Weights { vp: 0.0, held: 0.0, region_unseen: 0.0, final_: 0.0, bg: 0.0, europe_mult: 1.0, asia_mult: 1.0, access: 0.0, milops: 0.0,
            squeeze: 0.0, trap: 0.0, china: 0.0, bad: 0.0, space_ab: 0.0, inf: 0.0, auto_eu: 0.0, trapped: 0.0, h3: 0.0, h6: 0.0, samples: 1.0, noise: 1.0 }
    }
    pub fn set(&mut self, k: &str, v: f64) {
        match k {
            "vp" => self.vp = v, "held" => self.held = v, "region_unseen" => self.region_unseen = v, "final" => self.final_ = v,
            "bg" => self.bg = v, "europe_mult" => self.europe_mult = v, "asia_mult" => self.asia_mult = v, "access" => self.access = v,
            "milops" => self.milops = v, "squeeze" => self.squeeze = v, "trap" => self.trap = v, "china" => self.china = v,
            "bad" => self.bad = v, "space_ab" => self.space_ab = v, "inf" => self.inf = v, "auto_eu" => self.auto_eu = v, "trapped" => self.trapped = v,
            "h3" => self.h3 = v, "h6" => self.h6 = v, "samples" => self.samples = v, "noise" => self.noise = v,
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
    if w.final_ != 0.0 { v += w.final_ * sg * final_projection(st) as f64 * (0.3 + 0.7 * st.turn as f64 / 10.0); }
    // battlegrounds
    let h3 = if me == USSR && st.turn <= 3 { 1.0 + w.h3 } else { 1.0 };
    for c in 0..NC {
        if !COUNTRIES[c].bg || ctrl[c] < 0 { continue; }
        let m = match COUNTRIES[c].region { EUROPE => w.europe_mult, ASIA => w.asia_mult, _ => 1.0 };
        if ctrl[c] == me as i8 { v += w.bg * m * h3 } else { v -= w.bg * m * h3 }
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

    fn argmax(&self, vals: &[f64], rng: &mut Streams, me: usize) -> usize {
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
        for i in 0..k {
            let mut s = *base;
            let mut g0 = Greedy { b: self.b.clone() }; let mut g1 = Greedy { b: self.b.clone() };
            let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1);
            cx.sim = true;
            execute_play(&mut s, &mut cx, me, p);
            s.rounds_done[me] += 1;
            let v = self.b.e(&s, me);
            if s.winner == opp(me) as i8 { return v; } // hard filter: a play that can lose on the spot
            tot += v; n += 1;
            if i == 0 && cx.dice_used == 0 { break; }
        }
        tot / n as f64
    }
}

impl Agent for EvalBot {
    fn name(&self) -> String { self.label.clone() }
    fn headline(&mut self, st: &State, me: usize, _shown: Option<u8>, rng: &mut Streams) -> u8 {
        let base = determinize(st, me, rng);
        let cards: Vec<u8> = st.hands[me].iter().filter(|&k| k != 32).collect();
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
        let c = candidates(st, me, &self.b.w);
        if c.len() == 1 { return c[0]; }
        let base = determinize(st, me, rng);
        let vals: Vec<f64> = c.iter().map(|&p| self.sim_value(&base, me, p, rng)).collect();
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
