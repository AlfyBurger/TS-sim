//! Card Events (2026 HoF rules, FAQ v5, rulings in docs/rulings.md).
//! `s` = the side implementing the Event, `player` = the side who played the card.
use crate::agent::*;
use crate::data::*;
use crate::engine::*;
use crate::rng::Purpose;
use crate::state::*;
use crate::types::*;

pub fn can_occur(st: &State, card: u8, s: usize) -> bool {
    match card {
        7 => !st.ev.iron_lady,
        13 => !st.ev.camp_david,
        21 => st.ev.marshall || st.ev.warsaw,
        32 => false,
        48 => bg_count(st, US) > bg_count(st, USSR),
        55 => !st.ev.tear_down,
        56 => !st.ev.awacs,
        59 => !st.ev.evil_empire,
        61 => !st.ev.north_sea_oil,
        80 => st.space[s] < st.space[opp(s)],
        85 => st.space[US] > st.space[USSR],
        100 => st.defcon == 2,
        101 => st.ev.jp2,
        104 => st.era() < 2,
        108 => tables().region[MIDEAST as usize].iter().any(|&c| st.controls(US, c)),
        _ => true,
    }
}

pub fn after_event(_st: &mut State, _card: u8, _occurred: bool) {}

pub fn bg_count(st: &State, side: usize) -> i32 { (0..NC).filter(|&c| COUNTRIES[c].bg && st.controls(side, c)).count() as i32 }

// ------------------------------------------------------------- helpers
/// `s` adds `n` of its Influence, one at a time, among `cands`, at most `max_per` per country.
fn add_inf(st: &mut State, cx: &mut Ctx, s: usize, n: i32, max_per: i32, cands: &[usize]) {
    let mut added = [0i32; NC];
    for _ in 0..n {
        let t: Vec<usize> = cands.iter().copied().filter(|&c| added[c] < max_per).collect();
        if t.is_empty() { break; }
        let i = cx.ag[s].place_target(st, s, &t, &mut cx.rng);
        st.inf[t[i]][s] += 1; added[t[i]] += 1;
    }
}
/// `s` removes `n` of the opponent's Influence among `cands`, at most `max_per` per country.
fn remove_inf(st: &mut State, cx: &mut Ctx, s: usize, n: i32, max_per: i32, cands: &[usize]) {
    let o = opp(s);
    let mut done = [0i32; NC];
    for _ in 0..n {
        let t: Vec<usize> = cands.iter().copied().filter(|&c| done[c] < max_per && st.inf[c][o] > 0).collect();
        if t.is_empty() { break; }
        let i = cx.ag[s].remove_target(st, s, &t, &mut cx.rng);
        st.inf[t[i]][o] -= 1; done[t[i]] += 1;
    }
}
fn control_for(st: &mut State, s: usize, c: usize) {
    let need = st.stab(c) + st.inf[c][opp(s)];
    if st.inf[c][s] < need { st.inf[c][s] = need; }
}
fn pick_state(st: &mut State, cx: &mut Ctx, s: usize, opts: Vec<State>) {
    if opts.is_empty() { return; }
    let i = cx.ag[s].choose(st, s, &opts, &mut cx.rng);
    *st = opts[i];
}
/// Optional free Coup by `s` among `targets` with `ops` (DEFCON geography ignored).
fn free_coup_choice(st: &mut State, cx: &mut Ctx, s: usize, targets: &[usize], ops: i32) -> Option<usize> {
    let t: Vec<usize> = targets.iter().copied().filter(|&c| can_coup(st, s, c, true)).collect();
    if t.is_empty() { return None; }
    let mut opts: Vec<Vec<(f64, State)>> = vec![vec![(1.0, *st)]];
    for &c in &t { opts.push(coup_lottery(st, s, c, ops, true)); }
    let i = cx.ag[s].choose_lottery(st, s, &opts, &mut cx.rng);
    if i == 0 { return None; }
    let c = t[i - 1];
    let before = st.inf[c][opp(s)];
    coup(st, cx, s, c, ops, true);
    if st.inf[c][opp(s)] < before { Some(c) } else { Some(usize::MAX) }
}
fn war(st: &mut State, cx: &mut Ctx, s: usize, target: usize, extra_mod: i32, win_from: i32, vp: i32, milops: i8) {
    let o = opp(s);
    let mut m = extra_mod;
    for &a in COUNTRIES[target].adj { if st.controls(o, a) { m -= 1; } }
    if st.adj_to_sp(o, target) { m -= 1; }
    let d = cx.die(Purpose::War, s);
    st.add_milops(s, milops);
    if d + m >= win_from {
        st.add_vp(s, vp);
        st.inf[target][s] += st.inf[target][o]; st.inf[target][o] = 0;
    }
    cx.note(|| format!("War on {}: roll {} {:+} -> {}", cname(target), d, m, if d + m >= win_from { "victory" } else { "fails" }));
}
fn war_lottery(st: &State, s: usize, target: usize, win_from: i32, vp: i32) -> Vec<(f64, State)> {
    let o = opp(s);
    let mut m = 0;
    for &a in COUNTRIES[target].adj { if st.controls(o, a) { m -= 1; } }
    if st.adj_to_sp(o, target) { m -= 1; }
    let p = ((7 - (win_from - m)).clamp(0, 6)) as f64 / 6.0;
    let mut win = *st; win.add_vp(s, vp); win.inf[target][s] += win.inf[target][o]; win.inf[target][o] = 0;
    vec![(p, win), (1.0 - p, *st)]
}
fn ops_event(st: &mut State, cx: &mut Ctx, s: usize, n: i32, coup_ok: bool) {
    let base = ops_as_if(st, s, n);
    conduct_ops(st, cx, s, OpsSpec { base, card: 0, allow_coup: coup_ok, allow_realign: true, allow_place: true, allow_space: false });
}
fn reveal_hand(st: &mut State, side: usize) { let h = st.hands[side]; for k in h.iter() { if !st.known_by_opp[side].contains(k) { st.known_by_opp[side].push(k); } } }
fn random_discard(st: &mut State, cx: &mut Ctx, victim: usize) -> Option<u8> {
    let n = st.hands[victim].len();
    if n == 0 { return None; }
    let i = cx.rng.below(Purpose::Random, victim, n);
    let k = st.hands[victim].c[i];
    st.remove_from_hand(victim, k);
    Some(k)
}
fn in_list(names: &[&str]) -> Vec<usize> { names.iter().map(|n| cid(n)).collect() }
fn region_list(r: u8) -> Vec<usize> { tables().region[r as usize].clone() }

// ------------------------------------------------------------- the Events
pub fn event(st: &mut State, cx: &mut Ctx, card: u8, s: usize, player: usize, headline: bool) -> bool {
    if st.winner >= 0 { return false; }
    let o = opp(s);
    if CARDS[card as usize].scoring { play_scoring(st, cx, s, card); return true; }
    if !can_occur(st, card, s) { cx.note(|| format!("{}: Event cannot occur", card_name(card))); return false; }
    cx.note(|| format!("Event {} for {}", card_name(card), side_name(s)));
    match card {
        4 => { st.degrade(); if st.winner < 0 { st.add_vp(US, 5 - st.defcon as i32); } }
        5 => {
            if let Some(k) = random_discard(st, cx, USSR) {
                cx.note(|| format!("Five Year Plan discards {}", card_name(k)));
                if CARDS[k as usize].owner() == Some(US) { play_event(st, cx, US, k, false); } else { st.discard.push(k); }
            }
        }
        7 => { let we = tables().we.clone(); remove_inf(st, cx, USSR, 3, 2, &we); }
        8 => { st.inf[CUBA][US] = 0; control_for(st, USSR, CUBA); }
        9 => { st.inf[VIETNAM][USSR] += 2; st.ev.vietnam_revolts = true; }
        10 => {
            let elig: Vec<u8> = st.hands[US].iter().filter(|&k| !CARDS[k as usize].scoring && st.card_ops(US, k) >= 3).collect();
            let mut opts = vec![{ let mut s2 = *st; s2.inf[W_GERMANY][US] = 0; s2 }];
            for &k in &elig { let mut s2 = *st; s2.remove_from_hand(US, k); s2.discard.push(k); opts.push(s2); }
            pick_state(st, cx, US, opts);
        }
        11 => war(st, cx, USSR, S_KOREA, 0, 4, 2, 2),
        12 => { st.inf[ROMANIA][US] = 0; control_for(st, USSR, ROMANIA); }
        13 => { let m = if st.controls(US, ISRAEL) { -1 } else { 0 }; war(st, cx, USSR, ISRAEL, m, 4, 2, 2) }
        14 => {
            let cands: Vec<usize> = tables().ee.iter().copied().filter(|&c| !st.controls(US, c)).collect();
            add_inf(st, cx, USSR, 4, 1, &cands);
        }
        15 => { st.inf[EGYPT][USSR] += 2; let u = st.inf[EGYPT][US]; st.inf[EGYPT][US] = u - (u + 1) / 2; }
        16 => {
            st.ev.warsaw = true;
            // Option A: remove all US Influence from 4 Eastern European countries; option B: add 5 USSR (max 2 each).
            let mut a = *st;
            let mut ee: Vec<usize> = tables().ee.iter().copied().filter(|&c| a.inf[c][US] > 0).collect();
            ee.sort_by_key(|&c| -(a.inf[c][US] as i32 + if COUNTRIES[c].bg { 3 } else { 0 }));
            for &c in ee.iter().take(4) { a.inf[c][US] = 0; }
            let mut b = *st;
            let ee2 = tables().ee.clone();
            let mut added = [0i32; NC];
            for _ in 0..5 {
                let t: Vec<usize> = ee2.iter().copied().filter(|&c| added[c] < 2).collect();
                let i = cx.ag[USSR].place_target(&b, USSR, &t, &mut cx.rng);
                b.inf[t[i]][USSR] += 1; added[t[i]] += 1;
            }
            pick_state(st, cx, USSR, vec![a, b]);
        }
        17 => { st.inf[FRANCE][US] = (st.inf[FRANCE][US] - 2).max(0); st.inf[FRANCE][USSR] += 1; st.ev.de_gaulle = true; }
        18 => space_advance(st, s, 1, false),
        19 => {
            let t: Vec<usize> = region_list(EUROPE).into_iter().filter(|&c| st.ctrl(c) < 0 && st.inf[c][USSR] > 0).collect();
            let opts: Vec<State> = t.iter().map(|&c| { let mut s2 = *st; s2.inf[c][USSR] = 0; s2 }).collect();
            pick_state(st, cx, US, opts);
        }
        20 => {
            // Sponsor s; opponent participates (sponsor +2 to roll, reroll ties) or boycotts.
            let pw = 26.0 / 32.0;
            let mut win = *st; win.add_vp(s, 2);
            let mut lose = *st; lose.add_vp(o, 2);
            let mut boy = *st; boy.phasing = st.phasing; boy.degrade(); if boy.winner < 0 { boy.add_vp(s, 2); }
            let i = cx.ag[o].choose_lottery(st, o, &[vec![(pw, win), (1.0 - pw, lose)], vec![(1.0, boy)]], &mut cx.rng);
            if i == 0 {
                loop {
                    let a = cx.die(Purpose::EventDie, s) + 2; let b = cx.die(Purpose::EventDie, o);
                    if a > b { st.add_vp(s, 2); break; } if b > a { st.add_vp(o, 2); break; }
                }
            } else {
                st.degrade();
                if st.winner < 0 { ops_event(st, cx, s, 4, true); }
            }
        }
        21 => st.ev.nato = true,
        22 => {
            let t = in_list(&["Yugoslavia", "Romania", "Bulgaria", "Hungary", "Czechoslovakia"]);
            let opts: Vec<State> = t.iter().filter(|&&c| st.inf[c][USSR] > st.inf[c][US]).map(|&c| { let mut s2 = *st; s2.inf[c][US] = s2.inf[c][USSR]; s2 }).collect();
            pick_state(st, cx, US, opts);
        }
        23 => {
            st.ev.marshall = true;
            let cands: Vec<usize> = tables().we.iter().copied().filter(|&c| !st.controls(USSR, c)).collect();
            add_inf(st, cx, US, 7, 1, &cands);
        }
        24 | 102 => {
            let (a, b) = if card == 24 { (INDIA, PAKISTAN) } else { (IRAN, IRAQ) };
            let opts = vec![war_lottery(st, s, a, 4, 2), war_lottery(st, s, b, 4, 2)];
            let i = cx.ag[s].choose_lottery(st, s, &opts, &mut cx.rng);
            war(st, cx, s, if i == 0 { a } else { b }, 0, 4, 2, 2);
        }
        25 => st.ev.containment = true,
        26 => { reveal_hand(st, USSR); ops_event(st, cx, US, 1, true); }
        27 => { control_for(st, US, JAPAN); st.ev.us_japan = true; }
        28 => { let t = in_list(&["France", "U.K.", "Israel"]); remove_inf(st, cx, USSR, 4, 2, &t); }
        29 => { let n = if st.era() == 2 { 2 } else { 1 }; let ee = tables().ee.clone(); eeu(st, cx, n, &ee); }
        30 | 63 => {
            let mut t = region_list(AFRICA); t.extend(tables().sea.iter().copied());
            add_inf(st, cx, if card == 30 { USSR } else { US }, 4, 1, &t);
        }
        31 => st.ev.red_scare[o] = true,
        33 => destalinization(st, cx),
        34 => { st.add_vp(s, st.defcon as i32 - 2); let d = st.defcon + 2; st.set_defcon(d.min(5)); }
        35 => st.ev.formosan = true,
        36 => {
            let t: Vec<usize> = (0..NC).filter(|&c| st.stab(c) <= 2 && !(s == USSR && st.ev.nato && COUNTRIES[c].region == EUROPE && st.controls(US, c))).collect();
            let opts: Vec<Vec<(f64, State)>> = t.iter().map(|&c| { let mut l = war_lottery(st, s, c, 3, 1); for x in l.iter_mut() { x.1.add_milops(s, 3); } l }).collect();
            if !opts.is_empty() {
                let i = cx.ag[s].choose_lottery(st, s, &opts, &mut cx.rng);
                war(st, cx, s, t[i], 0, 3, 1, 3);
            }
        }
        39 => {
            let me = player;
            if st.milops[me] > st.milops[opp(me)] { st.add_vp(me, if st.milops[me] >= st.defcon { 3 } else { 1 }); }
        }
        40 => { st.set_defcon(2); st.ev.cmc = Some(s as u8); }
        41 => st.ev.nuclear_subs = true,
        42 => { st.ev.quagmire = true; st.ev.norad = false; }
        43 => {
            let d = st.defcon + 2; st.set_defcon(d.min(5)); st.ev.salt = true;
            let cands: Vec<u8> = st.discard.iter().filter(|&k| !CARDS[k as usize].scoring).collect();
            let mut opts = vec![*st];
            for &k in &cands { let mut s2 = *st; s2.discard.remove(k); s2.hands[s].push(k); s2.known_by_opp[s].push(k); opts.push(s2); }
            pick_state(st, cx, s, opts);
        }
        44 => st.ev.bear_trap = true,
        45 => {
            let bonus = |side: usize| (0..6u8).filter(|&r| region_level(st, r, side) >= 2).count() as i32;
            let a = cx.die(Purpose::EventDie, US) + bonus(US); let b = cx.die(Purpose::EventDie, USSR) + bonus(USSR);
            if a != b {
                let w = if a > b { US } else { USSR };
                st.add_vp(w, 2);
                if st.winner < 0 {
                    let mut up = *st; let d = up.defcon + 1; up.set_defcon(d.min(5));
                    let mut dn = *st; dn.degrade();
                    pick_state(st, cx, w, vec![*st, up, dn]);
                }
            }
        }
        46 => {
            st.add_milops(s, 5);
            let opts: Vec<State> = (1..=5).map(|d| { let mut s2 = *st; s2.set_defcon(d); s2 }).collect();
            pick_state(st, cx, s, opts);
        }
        47 => {
            let mut t = region_list(CENTAM); t.extend(region_list(SOUTHAM));
            let opts: Vec<State> = t.iter().map(|&c| { let mut s2 = *st; s2.inf[c][s] += 2; s2 }).collect();
            pick_state(st, cx, s, opts);
            let ops = st.card_ops(s, 47);
            free_coup_choice(st, cx, s, &t, ops);
        }
        48 => st.add_vp(US, 2),
        49 => missile_envy(st, cx, s),
        50 => { st.degrade(); if st.winner < 0 { st.ev.wwby_pending = true; } }
        51 => st.ev.brezhnev = true,
        52 => { st.inf[ANGOLA][USSR] += 2; st.inf[SE_AFRICAN_STATES][USSR] += 2; }
        53 => {
            let mut opts = vec![{ let mut s2 = *st; s2.inf[SOUTH_AFRICA][USSR] += 2; s2 }];
            for (a, b) in [(2, 0), (1, 1), (0, 2)] { let mut s2 = *st; s2.inf[SOUTH_AFRICA][USSR] += 1; s2.inf[ANGOLA][USSR] += a; s2.inf[BOTSWANA][USSR] += b; opts.push(s2); }
            pick_state(st, cx, USSR, opts);
        }
        54 => st.inf[CHILE][USSR] += 2,
        55 => { st.add_vp(USSR, 1); st.inf[W_GERMANY][USSR] += 1; st.ev.willy_brandt = true; }
        56 => {
            let t = in_list(&["Sudan", "Iran", "Iraq", "Egypt", "Libya", "Saudi Arabia", "Syria", "Jordan"]);
            for _ in 0..2 {
                let c2: Vec<usize> = t.iter().copied().filter(|&c| st.inf[c][US] > 0).collect();
                let opts: Vec<State> = c2.iter().map(|&c| { let mut s2 = *st; s2.inf[c][US] = 0; s2 }).collect();
                pick_state(st, cx, USSR, opts);
            }
        }
        57 => { let d = st.defcon + 1; st.set_defcon(d.min(5)); ops_event(st, cx, s, 4, true); }
        58 => { if st.china == US as u8 { st.china = USSR as u8; st.china_up = true; } else { st.add_vp(USSR, 1); } }
        59 => st.ev.flower_power = true,
        60 => { st.add_vp(USSR, 1); st.ev.u2 = true; }
        61 => { let n = in_list(&["Egypt", "Iran", "Libya", "Saudi Arabia", "Iraq", "Gulf States", "Venezuela"]).iter().filter(|&&c| st.controls(USSR, c)).count(); st.add_vp(USSR, n as i32); }
        62 => { reveal_hand(st, US); ops_event(st, cx, USSR, 1, true); }
        64 => { for c in in_list(&["Panama", "Costa Rica", "Venezuela"]) { st.inf[c][US] += 1; } }
        65 => { st.add_vp(US, 1); for c in in_list(&["Israel", "Jordan", "Egypt"]) { st.inf[c][US] += 1; } st.ev.camp_david = true; }
        66 => { let t: Vec<usize> = (0..NC).filter(|&c| st.inf[c] == [0, 0]).collect(); add_inf(st, cx, US, 3, 1, &t); }
        67 => grain_sales(st, cx),
        68 => { st.inf[POLAND][USSR] = (st.inf[POLAND][USSR] - 2).max(0); st.inf[POLAND][US] += 1; st.ev.jp2 = true; }
        69 => st.ev.lads = Some(s as u8),
        70 => { let mut t = region_list(CENTAM); t.extend(region_list(SOUTHAM)); add_inf(st, cx, US, 2, 2, &t); }
        71 => { if st.china == USSR as u8 { st.china = US as u8; st.china_up = false; } else { st.add_vp(US, 2); } }
        72 => { st.inf[EGYPT][USSR] = 0; st.inf[EGYPT][US] += 1; }
        73 => st.ev.shuttle = true,
        74 => { let t: Vec<usize> = (0..NC).filter(|&c| COUNTRIES[c].region != EUROPE).collect(); remove_inf(st, cx, US, 4, 2, &t); }
        75 => { let t = region_list(CENTAM); add_inf(st, cx, USSR, 3, 2, &t); }
        76 => { if st.china == USSR as u8 { st.china = US as u8; st.china_up = true; } else { let t = region_list(ASIA); add_inf(st, cx, US, 4, 2, &t); } }
        77 => ask_not(st, cx),
        78 => { let n = (0..NC).filter(|&c| matches!(COUNTRIES[c].region, CENTAM | SOUTHAM) && COUNTRIES[c].bg && st.controls(US, c)).count(); st.add_vp(US, n as i32); }
        80 => space_advance(st, s, 2, true),
        82 => { st.inf[IRAN][US] = 0; st.inf[IRAN][USSR] += 2; st.ev.iranian_hostage = true; }
        83 => { st.inf[ARGENTINA][USSR] += 1; st.inf[UK][USSR] = 0; st.add_vp(US, 1); st.ev.iron_lady = true; }
        84 => st.add_vp(US, st.inf[LIBYA][USSR] as i32 / 2),
        85 => star_wars(st, cx),
        86 => { st.ev.north_sea_oil = true; st.ev.nso_turn = true; }
        87 => { let n = if st.vp < 0 { 6 } else { 4 }; let t = region_list(EUROPE); add_inf(st, cx, USSR, n, 2, &t); st.ev.reformer = true; }
        88 => { st.inf[LEBANON][US] = 0; let t = region_list(MIDEAST); remove_inf(st, cx, USSR, 2, 2, &t); }
        89 => { st.degrade(); if st.winner < 0 { st.add_vp(US, 2); if st.winner < 0 && st.controls(US, S_KOREA) { ops_event(st, cx, US, 4, false); } } }
        90 => { let d = st.defcon + 1; st.set_defcon(d.min(5)); st.add_vp(USSR, 2); if st.winner < 0 && st.ev.reformer { ops_event(st, cx, USSR, 4, false); } }
        91 => { st.inf[NICARAGUA][US] = 0; let ops = st.card_ops(USSR, 91); free_coup_choice(st, cx, USSR, COUNTRIES[NICARAGUA].adj, ops); }
        92 => {
            let n = if s == USSR && st.ev.iranian_hostage { 2 } else { 1 };
            for _ in 0..n { if let Some(k) = random_discard(st, cx, o) { st.discard.push(k); } }
        }
        93 => st.ev.iran_contra = true,
        94 => {
            // US designates the Region where it leads most (protects a lead).
            let mut best = (i32::MIN, 0u8);
            for r in 0..6u8 { let (u, v) = region_score(st, r, None); if u - v > best.0 { best = (u - v, r); } }
            st.ev.chernobyl = Some(best.1);
        }
        95 => {
            let elig: Vec<u8> = st.hands[US].iter().filter(|&k| !CARDS[k as usize].scoring && st.card_ops(US, k) >= 3).collect();
            // USSR's doubling, chosen greedily by the USSR
            let sa = region_list(SOUTHAM);
            let mut dbl = *st;
            for _ in 0..2 {
                let t: Vec<usize> = sa.iter().copied().filter(|&c| dbl.inf[c][USSR] > 0 && dbl.inf[c][USSR] == st.inf[c][USSR]).collect();
                if t.is_empty() { break; }
                let opts: Vec<State> = t.iter().map(|&c| { let mut s2 = dbl; s2.inf[c][USSR] *= 2; s2 }).collect();
                let i = cx.ag[USSR].choose(&dbl, USSR, &opts, &mut cx.rng); dbl = opts[i];
            }
            let mut opts = vec![dbl];
            for &k in &elig { let mut s2 = *st; s2.remove_from_hand(US, k); s2.discard.push(k); opts.push(s2); }
            pick_state(st, cx, US, opts);
        }
        96 => {
            st.ev.willy_brandt = false; st.ev.tear_down = true;
            st.inf[E_GERMANY][US] += 3;
            let ops = st.card_ops(US, 96);
            let t = region_list(EUROPE);
            free_coup_choice(st, cx, US, &t, ops);
        }
        97 => { st.add_vp(US, 1); st.ev.flower_power = false; st.ev.evil_empire = true; }
        98 => {
            reveal_hand(st, US);
            let cards: Vec<u8> = st.hands[US].iter().collect();
            let opts: Vec<State> = cards.iter().map(|&k| { let mut s2 = *st; s2.remove_from_hand(US, k); s2.discard.push(k); s2 }).collect();
            pick_state(st, cx, USSR, opts);
        }
        99 => { st.add_vp(USSR, 1); let we = tables().we.clone(); remove_inf(st, cx, USSR, 3, 1, &we); }
        100 => {
            let mut end = *st;
            end.add_vp(o, 6);
            if end.winner < 0 { end.winner = if end.vp > 0 { US as i8 } else if end.vp < 0 { USSR as i8 } else { 2 }; end.end_reason = END_WARGAMES; }
            pick_state(st, cx, s, vec![*st, end]);
        }
        101 => st.inf[POLAND][US] += 3,
        103 => { if !headline && player == USSR { st.add_vp(US, 1); } }
        104 => {
            let sc: Vec<u8> = st.hands[US].iter().filter(|&k| CARDS[k as usize].scoring).collect();
            for &k in &sc { if !st.known_by_opp[US].contains(k) { st.known_by_opp[US].push(k); } }
            let mut t: Vec<usize> = vec![];
            for &k in &sc { if k == 38 { t.extend(tables().sea.iter().copied()); } else if let Some(r) = scoring_region_of(k) { t.extend(region_list(r)); } }
            if !t.is_empty() { add_inf(st, cx, USSR, 1, 1, &t); }
        }
        105 => {
            if st.controls(US, UK) {
                if !st.ev.nato { let t: Vec<usize> = COUNTRIES[UK].adj.to_vec(); add_inf(st, cx, US, 1, 1, &t); }
                else { let opts: Vec<State> = tables().we.iter().map(|&c| { let mut s2 = *st; s2.inf[c][US] += 2; s2 }).collect(); pick_state(st, cx, US, opts); st.add_vp(US, 2); }
            }
        }
        106 => st.ev.norad = true,
        107 => {
            let mut t = region_list(CENTAM); t.extend(region_list(SOUTHAM)); t.extend(region_list(AFRICA));
            let t: Vec<usize> = t.into_iter().filter(|&c| !COUNTRIES[c].bg).collect();
            let ops = st.card_ops(USSR, 107);
            if let Some(c) = free_coup_choice(st, cx, USSR, &t, ops) {
                if c != usize::MAX && st.winner < 0 { let t2: Vec<usize> = t.into_iter().filter(|&x| x != c).collect(); free_coup_choice(st, cx, USSR, &t2, ops); }
            }
        }
        108 => our_man(st, cx),
        109 => st.ev.yuri = true,
        110 => { st.inf[SAUDI_ARABIA][US] += 2; st.ev.awacs = true; }
        _ => { panic!("no Event for card {card}"); }
    }
    true
}

/// Region level for Summit: 0 none, 1 Presence, 2 Domination, 3 Control.
pub fn region_level(st: &State, r: u8, side: usize) -> i32 {
    let cs = &tables().region[r as usize];
    let (mut n, mut b, mut nb, mut tb) = ([0; 2], [0; 2], [0; 2], 0);
    for &c in cs { let isbg = bg_for_scoring(st, c); if isbg { tb += 1; } let k = st.ctrl(c); if k >= 0 { let s = k as usize; n[s] += 1; if isbg { b[s] += 1 } else { nb[s] += 1 } } }
    let o = opp(side);
    if n[side] == 0 { 0 } else if n[side] > n[o] && b[side] == tb { 3 } else if n[side] > n[o] && b[side] > b[o] && nb[side] >= 1 { 2 } else { 1 }
}

fn eeu(st: &mut State, cx: &mut Ctx, n: i8, ee: &[usize]) {
    let mut done: Vec<usize> = vec![];
    for _ in 0..3 {
        let t: Vec<usize> = ee.iter().copied().filter(|&c| st.inf[c][USSR] > 0 && !done.contains(&c)).collect();
        if t.is_empty() { break; }
        let opts: Vec<State> = t.iter().map(|&c| { let mut s2 = *st; s2.inf[c][USSR] = (s2.inf[c][USSR] - n).max(0); s2 }).collect();
        let i = cx.ag[US].choose(st, US, &opts, &mut cx.rng);
        done.push(t[i]); *st = opts[i];
    }
}

fn destalinization(st: &mut State, cx: &mut Ctx) {
    let mut added = [0i32; NC];
    for _ in 0..4 {
        let src: Vec<usize> = (0..NC).filter(|&c| st.inf[c][USSR] > 0).collect();
        if src.is_empty() { break; }
        let mut opts = vec![*st];
        for &c in &src { let mut s2 = *st; s2.inf[c][USSR] -= 1; opts.push(s2); }
        let i = cx.ag[USSR].choose(st, USSR, &opts, &mut cx.rng);
        if i == 0 { break; }
        let from = src[i - 1];
        let mut tmp = opts[i];
        let dst: Vec<usize> = (0..NC).filter(|&c| c != from && !tmp.controls(US, c) && added[c] < 2).collect();
        if dst.is_empty() { break; }
        let j = cx.ag[USSR].place_target(&tmp, USSR, &dst, &mut cx.rng);
        tmp.inf[dst[j]][USSR] += 1; added[dst[j]] += 1;
        // only keep the move if it is not worse than stopping
        let keep = cx.ag[USSR].choose(st, USSR, &[*st, tmp], &mut cx.rng);
        if keep == 0 { break; }
        *st = tmp;
    }
}

fn missile_envy(st: &mut State, cx: &mut Ctx, s: usize) {
    let o = opp(s);
    if st.hands[o].is_empty() { st.discard.push(49); return; }
    let best = st.hands[o].iter().map(|k| CARDS[k as usize].ops).max().unwrap();
    let tied: Vec<u8> = st.hands[o].iter().filter(|&k| CARDS[k as usize].ops == best).collect();
    let k = if tied.len() == 1 { tied[0] } else {
        let opts: Vec<State> = tied.iter().map(|&k| { let mut s2 = *st; s2.remove_from_hand(o, k); s2 }).collect();
        tied[cx.ag[o].choose(st, o, &opts, &mut cx.rng)]
    };
    st.remove_from_hand(o, k);
    st.hands[o].push(49); st.known_by_opp[o].push(49);
    st.ev.missile_envy_forced = Some(o as u8);
    cx.note(|| format!("Missile Envy takes {}", card_name(k)));
    if CARDS[k as usize].owner() != Some(o) {
        play_event(st, cx, s, k, false);
    } else {
        let base = st.card_ops(s, k);
        conduct_ops(st, cx, s, ops_spec(st, s, k, base));
        st.discard.push(k);
    }
}

fn grain_sales(st: &mut State, cx: &mut Ctx) {
    let n = st.hands[USSR].len();
    if n > 0 {
        let i = cx.rng.below(Purpose::Random, USSR, n);
        let k = st.hands[USSR].c[i];
        // US plays it as an Event if it is a US Event that can occur; otherwise returns it.
        if CARDS[k as usize].owner() == Some(US) && can_occur(st, k, US) {
            st.remove_from_hand(USSR, k);
            play_event(st, cx, US, k, false);
            return;
        }
        if !st.known_by_opp[USSR].contains(k) { st.known_by_opp[USSR].push(k); }
    }
    let base = st.card_ops(US, 67);
    conduct_ops(st, cx, US, ops_spec(st, US, 67, base));
}

fn ask_not(st: &mut State, cx: &mut Ctx) {
    let bad: Vec<u8> = st.hands[US].iter().filter(|&k| {
        let c = &CARDS[k as usize];
        (c.owner() == Some(USSR)) || (c.scoring && scoring_net(st, k) < 0)
    }).collect();
    for &k in &bad { st.remove_from_hand(US, k); st.discard.push(k); }
    for _ in 0..bad.len() { if let Some(k) = draw(st, cx) { st.hands[US].push(k); } }
}

fn star_wars(st: &mut State, cx: &mut Ctx) {
    let cands: Vec<u8> = st.discard.iter().filter(|&k| { let c = &CARDS[k as usize]; !c.scoring && c.owner() != Some(USSR) && k != 32 && k != 103 && can_occur(st, k, US) }).collect();
    if let Some(&k) = cands.iter().max_by_key(|&&k| CARDS[k as usize].ops) {
        st.discard.remove(k);
        play_event(st, cx, US, k, false);
    }
}

fn our_man(st: &mut State, cx: &mut Ctx) {
    let mut drawn: Vec<u8> = vec![];
    for _ in 0..5 { if let Some(k) = st.deck.pop() { drawn.push(k); } }
    for &k in &drawn { if CARDS[k as usize].owner() == Some(USSR) { st.discard.push(k); } else { st.deck.push(k); } }
    let n = st.deck.len();
    cx.rng.shuffle(Purpose::Shuffle, 0, &mut st.deck.c[..n]);
}
