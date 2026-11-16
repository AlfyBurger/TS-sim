//! Stage 2 run 2 decision rules, ported from the tested Red Sea bot:
//! opponent reply scan (blunder check), Scoring timing (score now or build first),
//! card sequencing (delta), set-up values for cards that reward preparation, Headline v2.
use crate::agent::*;
use crate::bot::*;
use crate::data::*;
use crate::engine::*;
use crate::events::can_occur;
use crate::rng::{Purpose, Streams};
use crate::state::*;
use crate::types::*;

/// (value to the side choosing, value to me)
pub type Pair = (f64, f64);

/// Best Ops use for `side`, chosen on side's evaluation, also valued on `me`'s.
pub fn ops_pair(b: &Brain, st: &State, side: usize, me: usize, spec: &OpsSpec) -> Pair {
    let mut best: Pair = (f64::NEG_INFINITY, 0.0);
    if spec.allow_place {
        let mut scopes = vec![Scope::Any];
        if spec.card == CHINA { scopes.push(Scope::Asia); }
        for sc in scopes {
            let n = spec.base + scope_bonus(st, side, spec.card, sc);
            let s = b.greedy_place(st, side, n, sc);
            let v = b.e(&s, side);
            if v > best.0 { best = (v, b.e(&s, me)); }
        }
    }
    if spec.allow_coup {
        for c in 0..NC {
            if !can_coup(st, side, c, false) { continue; }
            let sc = if COUNTRIES[c].region == ASIA { if COUNTRIES[c].sea { Scope::Sea } else { Scope::Asia } } else { Scope::Any };
            let ops = spec.base + scope_bonus(st, side, spec.card, sc);
            let (mut a, mut m) = (0.0, 0.0);
            for d in 1..=6 { let s = coup_result(st, side, c, ops, false, d); a += b.e(&s, side); m += b.e(&s, me); }
            if a / 6.0 > best.0 { best = (a / 6.0, m / 6.0); }
        }
    }
    if spec.allow_realign {
        let base_s = b.e(st, side);
        let mut tgt = None; let mut gain = f64::NEG_INFINITY;
        for c in 0..NC { if can_realign(st, side, c, false) { let g = b.realign_ev(st, side, c) - base_s; if g > gain { gain = g; tgt = Some(c); } } }
        if let Some(c) = tgt {
            let v = base_s + gain * spec.base as f64 * 0.8;
            if v > best.0 {
                let base_m = b.e(st, me);
                let gm = b.realign_ev_for(st, side, c, me) - base_m;
                best = (v, base_m + gm * spec.base as f64 * 0.8);
            }
        }
    }
    if best.0 == f64::NEG_INFINITY { best = (b.e(st, side), b.e(st, me)); }
    best
}

fn sim_event(b: &Brain, st: &State, player: usize, card: u8, me: usize, headline: bool, rng: &mut Streams) -> State {
    let mut t = *st;
    let mut g0 = Greedy { b: b.clone() }; let mut g1 = Greedy { b: b.clone() };
    let mut cx = Ctx::new(rng.next(Purpose::Sim, me), &mut g0, &mut g1);
    cx.sim = true;
    play_event(&mut t, &mut cx, player, card, headline);
    t
}

/// The opponent's options after my play, from my point of view: unseen pool (deck + their
/// unknown cards), the cards I know they hold, and the random hand size.
pub struct ReplyScan { pub pool: Vec<Pair>, pub known: Option<Pair>, pub h: usize, pub base_me: f64, pub opp_rounds: i32, pub my_rounds: i32 }

pub fn reply_scan(b: &Brain, s0: &State, me: usize, rng: &mut Streams) -> ReplyScan {
    let o = opp(me);
    let mut s = *s0;
    s.phasing = o as u8;
    s.snapshot_presence();
    let base_me = b.e(&s, me);
    let opp_rounds = rounds_for(&s, o) as i32 - s.rounds_done[o] as i32;
    let my_rounds = rounds_for(&s, me) as i32 - s.rounds_done[me] as i32;
    let empty = ReplyScan { pool: vec![], known: None, h: 0, base_me, opp_rounds, my_rounds };
    if s.winner >= 0 || opp_rounds <= 0 { return empty; }
    let mut cache: [Option<Pair>; 8] = [None; 8];
    let mut val = |k: u8, rng: &mut Streams| -> Pair {
        let ops = s.card_ops(o, k);
        let mut best: Pair = (f64::NEG_INFINITY, base_me);
        if ops > 0 {
            best = if k == CHINA { ops_pair(b, &s, o, me, &ops_spec(&s, o, k, ops)) } else {
                let i = ops.min(7) as usize;
                if cache[i].is_none() { cache[i] = Some(ops_pair(b, &s, o, me, &ops_spec(&s, o, k, ops))); }
                cache[i].unwrap()
            };
        }
        if k != CHINA && CARDS[k as usize].owner() != Some(me) && k != 32 && can_occur(&s, k, o) {
            let t = sim_event(b, &s, o, k, me, false, rng);
            let p = (b.e(&t, o), b.e(&t, me));
            if p.0 > best.0 { best = p; }
        }
        best
    };
    let mut known: Option<Pair> = None;
    let mut pool = vec![];
    let mut h = 0usize;
    for k in s.hands[o].iter() {
        if s.known_by_opp[o].contains(k) { let p = val(k, rng); if known.map_or(true, |q| p.0 > q.0) { known = Some(p); } }
        else { h += 1; }
    }
    if s.china_playable(o) { let p = val(CHINA, rng); if known.map_or(true, |q| p.0 > q.0) { known = Some(p); } }
    if h > 0 {
        let unseen: Vec<u8> = s.deck.iter().chain(s.hands[o].iter().filter(|&k| !s.known_by_opp[o].contains(k))).collect();
        for k in unseen { pool.push(val(k, rng)); }
        pool.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    }
    ReplyScan { pool, known, h, base_me, opp_rounds, my_rounds }
}

impl ReplyScan {
    /// My expected value after the opponent's best reply with `h` random cards (plus known cards).
    pub fn expect(&self, h: usize) -> f64 {
        let u = self.pool.len();
        if h == 0 || u == 0 { return self.known.map_or(self.base_me, |k| k.1); }
        let h = h.min(u);
        let mut p = h as f64 / u as f64;
        let (mut acc, mut mass) = (0.0, 0.0);
        for i in 0..u {
            if let Some(k) = self.known { if self.pool[i].0 <= k.0 { break; } }
            acc += p * self.pool[i].1; mass += p;
            if u - i - 1 == 0 || u - i < h + 1 { break; }
            p *= (u - i - h) as f64 / (u - i - 1) as f64;
        }
        let rest = (1.0 - mass).max(0.0);
        acc + rest * self.known.map_or(self.base_me, |k| k.1)
    }
    /// Expected loss to me from the opponent's best reply now.
    pub fn threat_now(&self) -> f64 { if self.opp_rounds <= 0 { 0.0 } else { self.base_me - self.expect(self.h) } }
    /// Average loss if the same play is made in one of my later rounds this turn (their hand smaller by one per round).
    pub fn threat_later(&self) -> f64 {
        if self.my_rounds <= 0 { return 0.0; }
        let mut tot = 0.0;
        for j in 1..=self.my_rounds {
            if self.opp_rounds - j <= 0 { continue; }
            let h = self.h.saturating_sub(j as usize);
            tot += self.base_me - if h == 0 && self.known.is_none() { self.base_me } else { self.expect(h) };
        }
        tot / self.my_rounds as f64
    }
}

/// Cards whose Event pays more on a prepared board (TS analogue of Red Sea's list).
pub fn is_prep(k: u8) -> bool {
    matches!(k, 7 | 11 | 13 | 15 | 24 | 28 | 36 | 47 | 53 | 54 | 55 | 56 | 61 | 63 | 64 | 65 | 66 | 68 | 72 | 75 | 82 | 87 | 89 | 91 | 99 | 101 | 102 | 104 | 105 | 107 | 110)
}

/// Set-up credit for prep cards still in my hand on position `s`.
pub fn setup_credit(b: &Brain, s: &State, me: usize, rng: &mut Streams) -> f64 {
    let w = b.w.ss;
    if w <= 0.0 { return 0.0; }
    let o = opp(me);
    let base = b.e(s, me);
    let left = rounds_for(s, me) as i32 - s.rounds_done[me] as i32;
    let decay = if left >= 3 { 1.0 } else if left > 0 { left as f64 / 3.0 } else { 0.5 };
    let guard = if s.hands[me].iter().any(|k| CARDS[k as usize].scoring) { 0.5 } else { 1.0 };
    let mut tot = 0.0;
    for k in s.hands[me].iter() {
        if !is_prep(k) || CARDS[k as usize].owner() == Some(o) || !can_occur(s, k, me) { continue; }
        let mut t0 = *s; t0.phasing = me as u8;
        let t = sim_event(b, &t0, me, k, me, false, rng);
        if t.winner == o as i8 { continue; }
        let g = (b.e(&t, me) - base).clamp(0.0, 4.0);
        tot += g;
    }
    w * decay * guard * tot.min(6.0)
}
