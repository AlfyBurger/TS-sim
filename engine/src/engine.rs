//! Rules engine: setup, turn sequence, Operations, scoring. Events are in events.rs.
use crate::agent::*;
use crate::data::*;
use crate::rng::{Purpose, Streams};
use crate::state::*;
use crate::types::*;

pub struct Stats {
    pub scoring_plays: Vec<(u8, u8, u8, i32)>, // (turn, side, card, net VP for the player)
    pub china_plays: Vec<(u8, u8)>,           // (turn, side)
    pub space_attempts: Vec<(u8, u8, u8, bool)>, // (turn, side, card, card was opponent's Event)
    pub opp_events: Vec<(u8, u8, u8)>,        // (turn, side who played, card) opponent Event triggered by Ops play
    pub coups: [u32; 2],
    pub realigns: [u32; 2],
    pub events_played: [u32; 2],
    pub milops_penalty: [i32; 2],
    pub defcon_loss_card: u8,
    pub vp_by_turn: Vec<i32>,
}
impl Stats { pub fn new() -> Self { Stats { scoring_plays: vec![], china_plays: vec![], space_attempts: vec![], opp_events: vec![], coups: [0; 2], realigns: [0; 2], events_played: [0; 2], milops_penalty: [0; 2], defcon_loss_card: 0, vp_by_turn: vec![] } } }

pub struct Ctx<'a> {
    pub rng: Streams,
    pub ag: [&'a mut dyn Agent; 2],
    pub sim: bool,
    pub log: Option<Vec<String>>,
    pub stats: Stats,
    pub dice_used: u32,
    pub current_card: u8,
}

impl<'a> Ctx<'a> {
    pub fn new(seed: u64, a0: &'a mut dyn Agent, a1: &'a mut dyn Agent) -> Self {
        Ctx { rng: Streams::new(seed), ag: [a0, a1], sim: false, log: None, stats: Stats::new(), dice_used: 0, current_card: 0 }
    }
    pub fn die(&mut self, p: Purpose, side: usize) -> i32 { self.dice_used += 1; self.rng.die(p, side) }
    pub fn note(&mut self, f: impl FnOnce() -> String) { if let Some(l) = &mut self.log { l.push(f()); } }
}

pub fn cname(c: usize) -> &'static str { COUNTRIES[c].name }
pub fn card_name(k: u8) -> &'static str { CARDS[k as usize].name }

// ---------------------------------------------------------------- deck
pub fn era_cards(era: u8) -> Vec<u8> { (1..=110u8).filter(|&k| CARDS[k as usize].era == era && k != CHINA).collect() }

pub fn draw(st: &mut State, cx: &mut Ctx) -> Option<u8> {
    if st.deck.is_empty() {
        if st.discard.is_empty() { return None; }
        st.deck = st.discard; st.discard.clear();
        let n = st.deck.len();
        cx.rng.shuffle(Purpose::Shuffle, 0, &mut st.deck.c[..n]);
        cx.note(|| "Reshuffle: discard pile becomes the draw deck".into());
    }
    st.deck.pop()
}

pub fn deal(st: &mut State, cx: &mut Ctx) {
    let size = if st.turn <= 3 { 8 } else { 9 };
    loop {
        let mut dealt = false;
        for s in [USSR, US] {
            if st.hands[s].len() < size { if let Some(k) = draw(st, cx) { st.hands[s].push(k); dealt = true; } }
        }
        if !dealt { break; }
    }
}

pub fn add_era(st: &mut State, cx: &mut Ctx, era: u8) {
    for k in era_cards(era) { st.deck.push(k); }
    let n = st.deck.len();
    cx.rng.shuffle(Purpose::Shuffle, 0, &mut st.deck.c[..n]);
}

// ---------------------------------------------------------------- setup
pub fn setup(st: &mut State, cx: &mut Ctx, handicap_us: i8) {
    *st = State::new();
    st.handicap_us = handicap_us;
    add_era(st, cx, 0);
    let fixed_su = [("Syria", 1), ("Iraq", 1), ("N. Korea", 3), ("E. Germany", 3), ("Finland", 1)];
    let fixed_us = [("Canada", 2), ("Iran", 1), ("Israel", 1), ("Japan", 1), ("Australia", 4), ("Philippines", 1), ("S. Korea", 1), ("Panama", 1), ("South Africa", 1), ("U.K.", 5)];
    for (n, v) in fixed_su { st.inf[cid(n)][USSR] = v; }
    for (n, v) in fixed_us { st.inf[cid(n)][US] = v; }
    deal(st, cx);
    let ee = tables().ee.clone(); let we = tables().we.clone();
    for _ in 0..6 { let t = cx.ag[USSR].place_target(st, USSR, &ee, &mut cx.rng); st.inf[ee[t]][USSR] += 1; }
    for _ in 0..7 { let t = cx.ag[US].place_target(st, US, &we, &mut cx.rng); st.inf[we[t]][US] += 1; }
    // Handicap (rule 13.4 limits): only where the US has Influence; at most 2 above what Control needs.
    for _ in 0..handicap_us {
        let cands: Vec<usize> = (0..NC).filter(|&c| st.inf[c][US] > 0 && st.inf[c][US] + 1 <= st.stab(c) + st.inf[c][USSR] + 2).collect();
        if cands.is_empty() { break; }
        let t = cx.ag[US].place_target(st, US, &cands, &mut cx.rng); st.inf[cands[t]][US] += 1;
    }
    cx.note(|| format!("Setup done (US handicap +{handicap_us})"));
}

// ---------------------------------------------------------------- scoring
pub fn bg_for_scoring(st: &State, c: usize) -> bool {
    COUNTRIES[c].bg || (c == TAIWAN && st.ev.formosan && st.controls(US, c))
}

/// (US points, USSR points) for a Region, ignoring `skip` (Shuttle Diplomacy).
pub fn region_score(st: &State, r: u8, skip: Option<usize>) -> (i32, i32) {
    let vals: [(i32, i32, i32); 6] = [(3, 7, 1000), (3, 7, 9), (3, 5, 7), (1, 4, 6), (1, 3, 5), (2, 5, 6)];
    let cs = &tables().region[r as usize];
    let mut n = [0i32; 2]; let mut b = [0i32; 2]; let mut nb = [0i32; 2]; let mut adj = [0i32; 2]; let mut total_bg = 0;
    for &c in cs {
        if Some(c) == skip { continue; }
        let isbg = bg_for_scoring(st, c);
        if isbg { total_bg += 1; }
        let k = st.ctrl(c);
        if k >= 0 {
            let s = k as usize; n[s] += 1;
            if isbg { b[s] += 1 } else { nb[s] += 1 }
            if st.adj_to_sp(opp(s), c) { adj[s] += 1; }
        }
    }
    let (p, d, ctl) = vals[r as usize];
    let mut out = [0i32; 2];
    for s in 0..2 {
        let o = opp(s);
        if n[s] == 0 { continue; }
        let lvl = if n[s] > n[o] && b[s] == total_bg && total_bg > 0 { ctl } else if n[s] > n[o] && b[s] > b[o] && nb[s] >= 1 { d } else { p };
        out[s] = lvl + b[s] + adj[s];
    }
    (out[US], out[USSR])
}

pub fn europe_controlled_by(st: &State) -> Option<usize> {
    let (u, r) = region_score(st, EUROPE, None);
    if u >= 1000 { Some(US) } else if r >= 1000 { Some(USSR) } else { None }
}

pub fn sea_score(st: &State) -> (i32, i32) {
    let mut out = [0i32; 2];
    for &c in &tables().sea { let k = st.ctrl(c); if k >= 0 { out[k as usize] += if c == THAILAND { 2 } else { 1 }; } }
    (out[US], out[USSR])
}

pub fn scoring_region_of(card: u8) -> Option<u8> {
    match card { 1 => Some(ASIA), 2 => Some(EUROPE), 3 => Some(MIDEAST), 37 => Some(CENTAM), 79 => Some(AFRICA), 81 => Some(SOUTHAM), _ => None }
}

/// Net VP (+ = US) a Scoring card would give now (Shuttle Diplomacy applied as the US would choose).
pub fn scoring_net(st: &State, card: u8) -> i32 {
    if card == 38 { let (u, r) = sea_score(st); return u - r; }
    let r = scoring_region_of(card).unwrap();
    let base = { let (u, s) = region_score(st, r, None); u - s };
    if st.ev.shuttle && (r == MIDEAST || r == ASIA) {
        let mut best = base;
        for &c in &tables().region[r as usize] {
            if bg_for_scoring(st, c) && st.controls(USSR, c) { let (u, s) = region_score(st, r, Some(c)); best = best.max(u - s); }
        }
        return best;
    }
    base
}

pub fn play_scoring(st: &mut State, cx: &mut Ctx, side: usize, card: u8) {
    if card == 2 {
        if let Some(w) = europe_controlled_by(st) { st.winner = w as i8; st.end_reason = END_EUROPE; cx.note(|| format!("Europe Scoring: {} Controls Europe and wins", side_name(w))); return; }
    }
    let net = scoring_net(st, card);
    if let Some(r) = scoring_region_of(card) {
        if st.ev.shuttle && (r == MIDEAST || r == ASIA) { st.ev.shuttle = false; st.discard.push(73); }
    }
    st.vp += net; st.check_vp();
    if !cx.sim { cx.stats.scoring_plays.push((st.turn, side as u8, card, if side == US { net } else { -net })); }
    cx.note(|| format!("{} scores: net {:+} (VP now {:+})", card_name(card), net, st.vp));
}

pub fn final_scoring(st: &mut State, cx: &mut Ctx) {
    if let Some(w) = europe_controlled_by(st) { st.winner = w as i8; st.end_reason = END_EUROPE; return; }
    for r in 0..6u8 { let (u, s) = region_score(st, r, None); st.vp += u - s; }
    st.vp += if st.china == US as u8 { 1 } else { -1 };
    st.winner = if st.vp > 0 { US as i8 } else if st.vp < 0 { USSR as i8 } else { 2 };
    st.end_reason = END_FINAL;
    cx.note(|| format!("Final Scoring: VP {:+}", st.vp));
}

/// Final-scoring projection (net, + = US), without ending the game.
pub fn final_projection(st: &State) -> i32 {
    let mut v = 0; for r in 0..6u8 { let (u, s) = region_score(st, r, None); v += (u - s).clamp(-30, 30); }
    v + if st.china == US as u8 { 1 } else { -1 }
}

// ---------------------------------------------------------------- legality
pub fn defcon_blocks(st: &State, c: usize) -> bool {
    let r = COUNTRIES[c].region;
    (r == EUROPE && st.defcon <= 4) || (r == ASIA && st.defcon <= 3) || (r == MIDEAST && st.defcon <= 2)
}
fn nato_protects(st: &State, side: usize, c: usize) -> bool {
    side == USSR && st.ev.nato && COUNTRIES[c].region == EUROPE && st.controls(US, c)
        && !(c == FRANCE && st.ev.de_gaulle) && !(c == W_GERMANY && st.ev.willy_brandt)
}
pub fn can_coup(st: &State, side: usize, c: usize, free_ignores_defcon: bool) -> bool {
    if st.inf[c][opp(side)] == 0 { return false; }
    if !free_ignores_defcon && defcon_blocks(st, c) { return false; }
    if nato_protects(st, side, c) { return false; }
    if side == USSR && c == JAPAN && st.ev.us_japan { return false; }
    if side == USSR && st.ev.reformer && COUNTRIES[c].region == EUROPE { return false; }
    true
}
pub fn can_realign(st: &State, side: usize, c: usize, free_ignores_defcon: bool) -> bool {
    if st.inf[c][opp(side)] == 0 { return false; }
    if !free_ignores_defcon && defcon_blocks(st, c) { return false; }
    if nato_protects(st, side, c) { return false; }
    if side == USSR && c == JAPAN && st.ev.us_japan { return false; }
    true
}
pub fn in_scope(c: usize, scope: Scope) -> bool {
    match scope { Scope::Any => true, Scope::Asia => COUNTRIES[c].region == ASIA, Scope::Sea => COUNTRIES[c].sea }
}
pub fn can_place(st: &State, side: usize, c: usize, scope: Scope) -> bool {
    if !in_scope(c, scope) { return false; }
    if side == USSR && st.ev.chernobyl == Some(COUNTRIES[c].region) { return false; }
    let m = st.pres_mask[side];
    if m & (1u128 << c) != 0 || st.adj_to_sp(side, c) { return true; }
    COUNTRIES[c].adj.iter().any(|&a| m & (1u128 << a) != 0)
}
pub fn place_cost(st: &State, side: usize, c: usize) -> i32 { if st.controls(opp(side), c) { 2 } else { 1 } }

/// Ops bonus for a declared scope (China Card in Asia, Vietnam Revolts in SE Asia).
pub fn scope_bonus(st: &State, side: usize, card: u8, scope: Scope) -> i32 {
    let mut b = 0;
    if card == CHINA && matches!(scope, Scope::Asia | Scope::Sea) { b += 1; }
    if side == USSR && st.ev.vietnam_revolts && card != 0 && scope == Scope::Sea { b += 1; }
    b
}

// ---------------------------------------------------------------- operations
/// Deterministic Coup result for a given die (for evaluation and lotteries; no RNG, no logging).
pub fn coup_result(st: &State, side: usize, c: usize, ops: i32, free: bool, d: i32) -> State {
    let mut s2 = *st;
    if s2.ev.cmc == Some(opp(side) as u8) { s2.winner = opp(side) as i8; s2.end_reason = END_CMC; return s2; }
    if side == US && s2.ev.yuri { s2.add_vp(USSR, 1); }
    let mut m = 0;
    if s2.ev.salt { m -= 1; }
    let reg = COUNTRIES[c].region;
    if (reg == CENTAM || reg == SOUTHAM) && s2.ev.lads.is_some() { m += if s2.ev.lads == Some(side as u8) { 1 } else { -1 }; }
    let x = d + ops + m - 2 * s2.stab(c) as i32;
    if x > 0 { let o = opp(side); let rem = (x as i8).min(s2.inf[c][o]); s2.inf[c][o] -= rem; s2.inf[c][side] += x as i8 - rem; }
    if !free { s2.add_milops(side, ops as i8); }
    if COUNTRIES[c].bg && !(side == US && s2.ev.nuclear_subs) { s2.degrade(); }
    s2
}
pub fn coup_lottery(st: &State, side: usize, c: usize, ops: i32, free: bool) -> Vec<(f64, State)> {
    (1..=6).map(|d| (1.0 / 6.0, coup_result(st, side, c, ops, free, d))).collect()
}

/// Ops value of an Event's "as if they played an N Ops card", with turn modifiers.
pub fn ops_as_if(st: &State, side: usize, n: i32) -> i32 {
    let mut o = n;
    if side == US && st.ev.containment { o = (o + 1).min(4.max(o)); }
    if side == USSR && st.ev.brezhnev { o = (o + 1).min(4.max(o)); }
    if st.ev.red_scare[side] { o = (o - 1).max(1); }
    o
}

pub fn coup(st: &mut State, cx: &mut Ctx, side: usize, c: usize, ops: i32, free: bool) {
    if st.ev.cmc == Some(opp(side) as u8) {
        st.winner = opp(side) as i8; st.end_reason = END_CMC;
        cx.note(|| format!("{} Coups under the Cuban Missile Crisis and loses", side_name(side)));
        return;
    }
    if side == US && st.ev.yuri { st.add_vp(USSR, 1); }
    let mut m = 0;
    if st.ev.salt { m -= 1; }
    let reg = COUNTRIES[c].region;
    if (reg == CENTAM || reg == SOUTHAM) && st.ev.lads.is_some() { m += if st.ev.lads == Some(side as u8) { 1 } else { -1 }; }
    let d = cx.die(Purpose::Coup, side);
    let x = d + ops + m - 2 * st.stab(c) as i32;
    if x > 0 {
        let o = opp(side);
        let rem = (x as i8).min(st.inf[c][o]);
        st.inf[c][o] -= rem; st.inf[c][side] += x as i8 - rem;
    }
    if !free { st.add_milops(side, ops as i8); }
    if !cx.sim { cx.stats.coups[side] += 1; }
    cx.note(|| format!("{} Coup in {} with {} Ops: roll {} {:+} -> {} (now {}/{})", side_name(side), cname(c), ops, d, m, x, st.inf[c][US], st.inf[c][USSR]));
    if COUNTRIES[c].bg && !(side == US && st.ev.nuclear_subs) { st.degrade(); if st.winner >= 0 && !cx.sim { cx.stats.defcon_loss_card = cx.current_card; } }
}

pub fn realign_mod(st: &State, side: usize, c: usize) -> i32 {
    let mut m = COUNTRIES[c].adj.iter().filter(|&&a| st.controls(side, a)).count() as i32;
    if st.inf[c][side] > st.inf[c][opp(side)] { m += 1; }
    if st.adj_to_sp(side, c) { m += 1; }
    if side == US && st.ev.iran_contra { m -= 1; }
    m
}

pub fn realign(st: &mut State, cx: &mut Ctx, side: usize, c: usize) {
    let o = opp(side);
    let a = cx.die(Purpose::Realign, side) + realign_mod(st, side, c);
    let b = cx.die(Purpose::Realign, o) + realign_mod(st, o, c);
    if a > b { st.inf[c][o] = (st.inf[c][o] - (a - b) as i8).max(0); }
    if b > a { st.inf[c][side] = (st.inf[c][side] - (b - a) as i8).max(0); }
    if !cx.sim { cx.stats.realigns[side] += 1; }
    cx.note(|| format!("{} Realigns {}: {} vs {} (now {}/{})", side_name(side), cname(c), a, b, st.inf[c][US], st.inf[c][USSR]));
}

pub const SPACE: [(i32, i32, i32, i32); 9] = [(0, 0, 0, 0), (2, 3, 2, 1), (2, 4, 0, 0), (2, 3, 2, 0), (2, 4, 0, 0), (3, 3, 3, 1), (3, 4, 0, 0), (3, 3, 4, 2), (4, 2, 2, 0)];

pub fn space_allowed(st: &State, side: usize) -> u8 { if st.space_ability(side, 2) { 2 } else { 1 } }
pub fn can_space(st: &State, side: usize, card: u8) -> bool {
    let b = st.space[side] as usize + 1;
    b <= 8 && st.space_attempts[side] < space_allowed(st, side) && !CARDS[card as usize].scoring && st.card_ops(side, card) >= SPACE[b].0
}
pub fn space_advance(st: &mut State, side: usize, n: usize, last_vp_only: bool) {
    for i in 0..n {
        let b = st.space[side] as usize + 1;
        if b > 8 { break; }
        st.space[side] = b as u8;
        let first = st.space[opp(side)] < b as u8;
        if first { st.space_first[b] = side as i8; }
        if !last_vp_only || i == n - 1 { let v = if first { SPACE[b].2 } else { SPACE[b].3 }; if v > 0 { st.add_vp(side, v); } }
    }
}
pub fn space_attempt(st: &mut State, cx: &mut Ctx, side: usize, card: u8) {
    let b = st.space[side] as usize + 1;
    st.space_attempts[side] += 1;
    let d = cx.die(Purpose::Space, side);
    let ok = d <= SPACE[b].1;
    if ok { space_advance(st, side, 1, false); }
    if !cx.sim {
        let oe = CARDS[card as usize].owner() == Some(opp(side));
        cx.stats.space_attempts.push((st.turn, side as u8, card, oe));
    }
    cx.note(|| format!("{} Space Race with {}: roll {} -> {}", side_name(side), card_name(card), d, if ok { "advance" } else { "fail" }));
}

/// Conduct Operations (card-based or Event-granted).
pub fn conduct_ops(st: &mut State, cx: &mut Ctx, side: usize, spec: OpsSpec) {
    if st.winner >= 0 { return; }
    let choice = cx.ag[side].ops(st, side, &spec, &mut cx.rng);
    match choice {
        OpsChoice::Place(scope) if spec.allow_place => {
            let mut left = spec.base + scope_bonus(st, side, spec.card, scope);
            loop {
                let t: Vec<usize> = (0..NC).filter(|&c| can_place(st, side, c, scope) && place_cost(st, side, c) <= left).collect();
                if t.is_empty() || left <= 0 { break; }
                let i = cx.ag[side].place_target(st, side, &t, &mut cx.rng);
                let c = t[i]; left -= place_cost(st, side, c); st.inf[c][side] += 1;
            }
            cx.note(|| format!("{} places Influence ({} Ops)", side_name(side), spec.base));
        }
        OpsChoice::Coup(c) if spec.allow_coup && can_coup(st, side, c, false) => {
            let sc = if COUNTRIES[c].region == ASIA { if COUNTRIES[c].sea { Scope::Sea } else { Scope::Asia } } else { Scope::Any };
            let ops = spec.base + scope_bonus(st, side, spec.card, sc);
            coup(st, cx, side, c, ops, false);
        }
        OpsChoice::Realign(scope) if spec.allow_realign => {
            let n = spec.base + scope_bonus(st, side, spec.card, scope);
            for _ in 0..n {
                if st.winner >= 0 { break; }
                let t: Vec<usize> = (0..NC).filter(|&c| in_scope(c, scope) && can_realign(st, side, c, false)).collect();
                if t.is_empty() { break; }
                let i = cx.ag[side].realign_target(st, side, &t, &mut cx.rng);
                realign(st, cx, side, t[i]);
            }
        }
        OpsChoice::Space if spec.allow_space && spec.card != 0 && can_space(st, side, spec.card) => { space_attempt(st, cx, side, spec.card); }
        OpsChoice::None => {}
        other => {
            if !cx.sim { panic!("illegal Ops choice {:?} by {} with spec {:?}", other, side_name(side), spec); }
        }
    }
}

pub fn ops_spec(st: &State, side: usize, card: u8, base: i32) -> OpsSpec {
    let _ = st;
    OpsSpec { base, card, allow_coup: true, allow_realign: true, allow_place: true, allow_space: false }
}

// ---------------------------------------------------------------- card plays
pub fn dispose(st: &mut State, card: u8, event_occurred: bool) {
    if card == CHINA { return; }
    if event_occurred && CARDS[card as usize].star { st.removed.push(card); } else { st.discard.push(card); }
}

pub fn is_war(card: u8) -> bool { matches!(card, 11 | 13 | 24 | 36 | 102) }

fn flower_power(st: &mut State, side: usize, card: u8) {
    if side == US && is_war(card) && st.ev.flower_power && !st.ev.evil_empire { st.add_vp(USSR, 2); }
}

/// Resolve a played card's Event for `player`; returns whether it occurred. Handles disposal.
pub fn play_event(st: &mut State, cx: &mut Ctx, player: usize, card: u8, headline: bool) -> bool {
    let info = &CARDS[card as usize];
    if info.scoring { play_scoring(st, cx, player, card); if card == 38 { st.removed.push(38); } else { st.discard.push(card); } return true; }
    let implementer = info.owner().unwrap_or(player);
    let prev = cx.current_card; cx.current_card = card;
    let occurred = crate::events::event(st, cx, card, implementer, player, headline);
    cx.current_card = prev;
    if occurred && !cx.sim { cx.stats.events_played[implementer] += 1; }
    crate::events::after_event(st, card, occurred);
    if card != 49 && card != 73 { dispose(st, card, occurred); }
    occurred
}

/// Execute a full Action Round play.
pub fn execute_play(st: &mut State, cx: &mut Ctx, side: usize, p: Play) {
    let card = p.card;
    if p.how == Use::Pass { return; }
    if card == CHINA { st.china = opp(side) as u8; st.china_up = false; if side == US { st.ev.formosan = false; } if !cx.sim { cx.stats.china_plays.push((st.turn, side as u8)); } }
    else if p.uni_with == 0 && !st.remove_from_hand(side, card) { panic!("{} plays {} not in hand", side_name(side), card_name(card)); }
    cx.note(|| format!("T{} AR{} {} [DEFCON {} VP {:+}]: {} as {:?}{}", st.turn, st.ar, side_name(side), st.defcon, st.vp, card_name(card), p.how, if p.uni_with > 0 { format!(" with {}", card_name(p.uni_with)) } else { String::new() }));
    // UN Intervention: cancel the opponent's Event, use its Ops.
    if p.uni_with != 0 {
        st.remove_from_hand(side, 32); st.remove_from_hand(side, p.uni_with);
        if st.ev.u2 { st.add_vp(USSR, 1); }
        flower_power(st, side, p.uni_with);
        let base = st.card_ops(side, p.uni_with);
        conduct_ops(st, cx, side, ops_spec(st, side, p.uni_with, base));
        st.discard.push(32); st.discard.push(p.uni_with);
        return;
    }
    match p.how {
        Use::Space => { space_attempt(st, cx, side, card); if card != CHINA { st.discard.push(card); } }
        Use::Event => { flower_power(st, side, card); play_event(st, cx, side, card, false); }
        Use::Ops => {
            flower_power(st, side, card);
            let owner = CARDS[card as usize].owner();
            let opp_event = owner == Some(opp(side)) && crate::events::can_occur(st, card, opp(side));
            let base = st.card_ops(side, card);
            if opp_event && !cx.sim { cx.stats.opp_events.push((st.turn, side as u8, card)); }
            if opp_event && p.event_first {
                let occurred = crate::events::event(st, cx, card, opp(side), side, false);
                crate::events::after_event(st, card, occurred);
                conduct_ops(st, cx, side, ops_spec(st, side, card, base));
                if card != 49 && card != 73 { dispose(st, card, occurred); }
            } else {
                conduct_ops(st, cx, side, ops_spec(st, side, card, base));
                if opp_event && st.winner < 0 {
                    let occurred = crate::events::event(st, cx, card, opp(side), side, false);
                    crate::events::after_event(st, card, occurred);
                    if card != 49 && card != 73 { dispose(st, card, occurred); }
                } else if card == 103 && side == USSR {
                    st.add_vp(US, 1); dispose(st, card, false);
                } else { dispose(st, card, false); }
            }
        }
        Use::Pass => {}
    }
}

// ---------------------------------------------------------------- turn sequence
pub fn rounds_for(st: &State, side: usize) -> u8 {
    let base = if st.turn <= 3 { 6 } else { 7 };
    if st.space_ability(side, 8) || (side == US && st.ev.nso_turn) { base + 1 } else { base }
}

fn trap_round(st: &mut State, cx: &mut Ctx, side: usize) -> bool {
    let trapped = (side == US && st.ev.quagmire) || (side == USSR && st.ev.bear_trap);
    if !trapped { return false; }
    let elig: Vec<u8> = st.hands[side].iter().filter(|&k| !CARDS[k as usize].scoring && st.card_ops(side, k) >= 2).collect();
    if !elig.is_empty() {
        let elig = if st.ev.missile_envy_forced == Some(side as u8) && elig.contains(&49) { vec![49] } else { elig };
        let opts: Vec<State> = elig.iter().map(|&k| { let mut s2 = *st; s2.remove_from_hand(side, k); s2.discard.push(k); s2 }).collect();
        let i = cx.ag[side].choose(st, side, &opts, &mut cx.rng);
        let k = elig[i]; st.remove_from_hand(side, k); st.discard.push(k);
        if k == 49 { st.ev.missile_envy_forced = None; }
        let d = cx.die(Purpose::Trap, side);
        if d <= 4 { if side == US { st.ev.quagmire = false } else { st.ev.bear_trap = false } }
        cx.note(|| format!("{} trapped: discards {}, rolls {}", side_name(side), card_name(k), d));
        return true;
    }
    let sc: Vec<u8> = st.hands[side].iter().filter(|&k| CARDS[k as usize].scoring).collect();
    if let Some(&k) = sc.first() {
        st.remove_from_hand(side, k);
        play_event(st, cx, side, k, false);
    }
    true
}

pub fn action_round(st: &mut State, cx: &mut Ctx, side: usize) {
    if st.winner >= 0 { return; }
    st.phasing = side as u8;
    st.ev.defcon_hit_2_this_ar = false;
    st.snapshot_presence();
    let had_cards = !st.hands[side].is_empty() || st.china_playable(side);
    if !had_cards { return; }
    if !trap_round(st, cx, side) {
        let left = rounds_for(st, side) as i32 - st.rounds_done[side] as i32;
        let sc = st.hands[side].iter().filter(|&k| CARDS[k as usize].scoring).count() as i32;
        let p = if st.ev.missile_envy_forced == Some(side as u8) && st.hands[side].contains(49) && sc < left {
            st.ev.missile_envy_forced = None;
            Play { card: 49, how: Use::Ops, event_first: false, uni_with: 0 }
        } else {
            if st.hands[side].is_empty() && !st.china_playable(side) { return; }
            cx.ag[side].action(st, side, &mut cx.rng)
        };
        if side == US && st.ev.wwby_pending {
            st.ev.wwby_pending = false;
            if p.uni_with == 0 { st.add_vp(USSR, 3); }
        }
        if st.winner < 0 { execute_play(st, cx, side, p); }
    }
    // NORAD
    if st.winner < 0 && st.ev.norad && st.ev.defcon_hit_2_this_ar && st.controls(US, CANADA) {
        let t: Vec<usize> = (0..NC).filter(|&c| st.inf[c][US] > 0).collect();
        if !t.is_empty() { let i = cx.ag[US].place_target(st, US, &t, &mut cx.rng); st.inf[t[i]][US] += 1; }
    }
    st.rounds_done[side] += 1;
}

pub fn headline(st: &mut State, cx: &mut Ctx) {
    st.ar = 0;
    st.snapshot_presence();
    let mut h = [0u8; 2];
    let first_shower = if st.space_ability(US, 4) { Some(USSR) } else if st.space_ability(USSR, 4) { Some(US) } else { None };
    match first_shower {
        Some(f) => {
            h[f] = cx.ag[f].headline(st, f, None, &mut cx.rng);
            h[opp(f)] = cx.ag[opp(f)].headline(st, opp(f), Some(h[f]), &mut cx.rng);
        }
        None => {
            h[US] = cx.ag[US].headline(st, US, None, &mut cx.rng);
            h[USSR] = cx.ag[USSR].headline(st, USSR, None, &mut cx.rng);
        }
    }
    for s in 0..2 { if !st.remove_from_hand(s, h[s]) { panic!("headline card not in hand"); } }
    cx.note(|| format!("T{} Headline: US {} / USSR {}", st.turn, card_name(h[US]), card_name(h[USSR])));
    let mut cancelled = [false; 2];
    if h[US] == 103 { cancelled[USSR] = true; st.discard.push(h[USSR]); cx.note(|| "Defectors cancels the USSR Headline".into()); }
    let ops = |k: u8| CARDS[k as usize].ops;
    let order = if ops(h[USSR]) > ops(h[US]) { [USSR, US] } else { [US, USSR] };
    for s in order {
        if cancelled[s] { continue; }
        if st.winner >= 0 { st.discard.push(h[s]); continue; }
        st.phasing = s as u8;
        if h[s] == 103 && s == US { st.discard.push(103); continue; }
        if h[s] == 103 && s == USSR { st.discard.push(103); continue; }
        play_event(st, cx, s, h[s], true);
    }
}

pub fn end_of_turn(st: &mut State, cx: &mut Ctx) {
    // E. Military Operations
    let mut net = 0;
    for s in 0..2 { let short = (st.defcon - st.milops[s]).max(0) as i32; if !cx.sim { cx.stats.milops_penalty[s] += short; } net += if s == US { -short } else { short }; }
    st.vp += net;
    if st.vp.abs() >= 20 {
        let w = if st.vp > 0 { US } else { USSR };
        if st.hands[w].iter().any(|k| CARDS[k as usize].scoring) { st.winner = opp(w) as i8; st.end_reason = END_HELD_SCORING; } else { st.winner = w as i8; st.end_reason = END_VP20; }
        return;
    }
    st.milops = [0; 2];
    // F. Held Scoring cards
    let held = [0, 1].map(|s| st.hands[s].iter().any(|k| CARDS[k as usize].scoring));
    if held[0] || held[1] { st.winner = if held[US] && held[USSR] { US as i8 } else if held[US] { USSR as i8 } else { US as i8 }; st.end_reason = END_HELD_SCORING; return; }
    // Space Race box 6: may discard one held card
    for s in 0..2 {
        if st.space_ability(s, 6) && !st.hands[s].is_empty() {
            let cards: Vec<u8> = st.hands[s].iter().collect();
            let mut opts: Vec<State> = vec![*st];
            for &k in &cards { let mut s2 = *st; s2.remove_from_hand(s, k); s2.discard.push(k); opts.push(s2); }
            let i = cx.ag[s].choose(st, s, &opts, &mut cx.rng);
            if i > 0 { *st = opts[i]; }
        }
    }
    // G. China Card
    st.china_up = true;
    // turn effects end
    let e = &mut st.ev;
    e.containment = false; e.brezhnev = false; e.red_scare = [false; 2]; e.vietnam_revolts = false; e.lads = None;
    e.nuclear_subs = false; e.cmc = None; e.salt = false; e.chernobyl = None; e.yuri = false; e.iran_contra = false;
    e.nso_turn = false; e.u2 = false;
    st.space_attempts = [0; 2];
    st.rounds_done = [0; 2];
    if !cx.sim { cx.stats.vp_by_turn.push(st.vp); }
}

pub fn play_turn(st: &mut State, cx: &mut Ctx) {
    if st.turn > 1 && st.defcon < 5 { st.defcon += 1; }
    if st.turn == 4 { add_era(st, cx, 1); }
    if st.turn == 8 { add_era(st, cx, 2); }
    deal(st, cx);
    headline(st, cx);
    let mut r = 1u8;
    while r <= rounds_for(st, US).max(rounds_for(st, USSR)) {
        st.ar = r;
        for s in [USSR, US] {
            if st.winner >= 0 { return; }
            if r <= rounds_for(st, s) { action_round(st, cx, s); }
        }
        r += 1;
    }
    if st.winner >= 0 { return; }
    end_of_turn(st, cx);
}

pub fn play_game(st: &mut State, cx: &mut Ctx, handicap_us: i8) {
    setup(st, cx, handicap_us);
    for t in 1..=10 {
        st.turn = t;
        play_turn(st, cx);
        if st.winner >= 0 { return; }
    }
    final_scoring(st, cx);
}
