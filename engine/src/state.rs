use crate::data::*;
use crate::types::*;
use std::sync::OnceLock;

/// Fixed-capacity card list (no heap: a State copies with memcpy).
#[derive(Clone, Copy)]
pub struct CardList { pub n: u8, pub c: [u8; 112] }
impl CardList {
    pub const fn new() -> Self { CardList { n: 0, c: [0; 112] } }
    pub fn len(&self) -> usize { self.n as usize }
    pub fn is_empty(&self) -> bool { self.n == 0 }
    pub fn as_slice(&self) -> &[u8] { &self.c[..self.n as usize] }
    pub fn push(&mut self, x: u8) { self.c[self.n as usize] = x; self.n += 1; }
    pub fn pop(&mut self) -> Option<u8> { if self.n == 0 { None } else { self.n -= 1; Some(self.c[self.n as usize]) } }
    pub fn contains(&self, x: u8) -> bool { self.as_slice().contains(&x) }
    pub fn remove(&mut self, x: u8) -> bool {
        if let Some(i) = self.as_slice().iter().position(|&y| y == x) {
            for j in i..self.n as usize - 1 { self.c[j] = self.c[j + 1]; }
            self.n -= 1; true
        } else { false }
    }
    pub fn clear(&mut self) { self.n = 0; }
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ { self.as_slice().iter().copied() }
    pub fn as_mut_slice(&mut self) -> &mut [u8] { &mut self.c[..self.n as usize] }
}

pub const CHINA: u8 = 6;

/// Lasting and turn effects.
#[derive(Clone, Copy, Default)]
pub struct Effects {
    // lasting (underlined / starred with markers)
    pub marshall: bool, pub warsaw: bool, pub nato: bool, pub de_gaulle: bool, pub willy_brandt: bool,
    pub us_japan: bool, pub formosan: bool, pub camp_david: bool, pub jp2: bool, pub flower_power: bool,
    pub evil_empire: bool, pub iron_lady: bool, pub north_sea_oil: bool, pub reformer: bool, pub tear_down: bool,
    pub awacs: bool, pub norad: bool, pub iranian_hostage: bool, pub shuttle: bool,
    pub quagmire: bool, pub bear_trap: bool,
    // turn effects
    pub containment: bool, pub brezhnev: bool, pub red_scare: [bool; 2], pub vietnam_revolts: bool,
    pub lads: Option<u8>, pub nuclear_subs: bool, pub cmc: Option<u8>, pub salt: bool, pub chernobyl: Option<u8>,
    pub yuri: bool, pub iran_contra: bool, pub nso_turn: bool, pub u2: bool,
    pub wwby_pending: bool, pub missile_envy_forced: Option<u8>,
    pub defcon_hit_2_this_ar: bool,
}

#[derive(Clone, Copy)]
pub struct State {
    pub inf: [[i8; 2]; NC],
    pub turn: u8,
    pub ar: u8,              // 0 = headline
    pub phasing: u8,
    pub defcon: i8,
    pub vp: i32,             // + = US
    pub milops: [i8; 2],
    pub space: [u8; 2],
    pub space_first: [i8; 9], // side that first reached box k (-1 none)
    pub space_attempts: [u8; 2],
    pub hands: [CardList; 2],
    pub deck: CardList,
    pub discard: CardList,
    pub removed: CardList,
    pub china: u8,           // holder
    pub china_up: bool,
    pub winner: i8,          // -1 none, 0 US, 1 USSR, 2 draw
    pub end_reason: u8,
    pub ev: Effects,
    pub rounds_done: [u8; 2],
    pub known_by_opp: [CardList; 2], // cards in side s's hand that the opponent has seen
    pub handicap_us: i8,
    pub pres_mask: [u128; 2], // Influence present at the start of the Action Round
}

pub const END_NONE: u8 = 0; pub const END_VP20: u8 = 1; pub const END_EUROPE: u8 = 2; pub const END_DEFCON: u8 = 3;
pub const END_HELD_SCORING: u8 = 4; pub const END_FINAL: u8 = 5; pub const END_WARGAMES: u8 = 6; pub const END_CMC: u8 = 7;
pub const END_NAMES: [&str; 8] = ["none", "20 VP", "Europe Control", "DEFCON 1", "held Scoring card", "Final Scoring", "Wargames", "Cuban Missile Crisis"];

pub struct Tables { pub region: [Vec<usize>; 6], pub ee: Vec<usize>, pub we: Vec<usize>, pub sea: Vec<usize> }
pub fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        let mut region: [Vec<usize>; 6] = Default::default();
        let (mut ee, mut we, mut sea) = (vec![], vec![], vec![]);
        for (i, c) in COUNTRIES.iter().enumerate() {
            region[c.region as usize].push(i);
            if c.ee { ee.push(i) } if c.we { we.push(i) } if c.sea { sea.push(i) }
        }
        Tables { region, ee, we, sea }
    })
}
pub fn cid(name: &str) -> usize { COUNTRIES.iter().position(|c| c.name == name).unwrap_or_else(|| panic!("no country {name}")) }

impl State {
    pub fn new() -> Self {
        State {
            inf: [[0; 2]; NC], turn: 1, ar: 0, phasing: USSR as u8, defcon: 5, vp: 0, milops: [0; 2], space: [0; 2],
            space_first: [-1; 9], space_attempts: [0; 2], hands: [CardList::new(); 2], deck: CardList::new(),
            discard: CardList::new(), removed: CardList::new(), china: USSR as u8, china_up: true, winner: -1,
            end_reason: 0, ev: Effects::default(), rounds_done: [0; 2], known_by_opp: [CardList::new(); 2], handicap_us: 0, pres_mask: [0; 2],
        }
    }
    #[inline] pub fn stab(&self, c: usize) -> i8 { COUNTRIES[c].stab }
    #[inline] pub fn ctrl(&self, c: usize) -> i8 {
        let [u, r] = self.inf[c]; let s = self.stab(c);
        if u >= s && u - r >= s { US as i8 } else if r >= s && r - u >= s { USSR as i8 } else { -1 }
    }
    #[inline] pub fn controls(&self, side: usize, c: usize) -> bool { self.ctrl(c) == side as i8 }
    pub fn is_bg(&self, c: usize) -> bool { COUNTRIES[c].bg }
    pub fn adj_to_sp(&self, side: usize, c: usize) -> bool { if side == US { COUNTRIES[c].adj_us } else { COUNTRIES[c].adj_ussr } }
    pub fn era(&self) -> u8 { if self.turn <= 3 { 0 } else if self.turn <= 7 { 1 } else { 2 } }
    pub fn add_vp(&mut self, side: usize, n: i32) {
        if self.winner >= 0 { return; }
        self.vp += if side == US { n } else { -n };
        self.check_vp();
    }
    pub fn check_vp(&mut self) {
        if self.winner >= 0 { return; }
        if self.vp >= 20 { self.winner = US as i8; self.end_reason = END_VP20; }
        if self.vp <= -20 { self.winner = USSR as i8; self.end_reason = END_VP20; }
    }
    /// Degrade DEFCON by one; the phasing player loses at 1.
    pub fn degrade(&mut self) { self.set_defcon(self.defcon - 1); }
    pub fn set_defcon(&mut self, d: i8) {
        let d = d.clamp(1, 5);
        if d == 2 && self.defcon != 2 { self.ev.defcon_hit_2_this_ar = true; }
        self.defcon = d;
        if d <= 1 && self.winner < 0 { self.winner = opp(self.phasing as usize) as i8; self.end_reason = END_DEFCON; }
    }
    pub fn add_milops(&mut self, side: usize, n: i8) { self.milops[side] = (self.milops[side] + n).min(5); }
    pub fn has_influence_in_region(&self, side: usize, r: u8) -> bool { tables().region[r as usize].iter().any(|&c| self.inf[c][side] > 0) }
    /// Card's Ops value for `side` with turn modifiers (Containment, Brezhnev, Red Scare); China base 4.
    pub fn card_ops(&self, side: usize, card: u8) -> i32 {
        let mut o = CARDS[card as usize].ops as i32;
        if CARDS[card as usize].scoring { return 0; }
        if side == US && self.ev.containment { o = (o + 1).min(4.max(o)); }
        if side == USSR && self.ev.brezhnev { o = (o + 1).min(4.max(o)); }
        if self.ev.red_scare[side] { o = (o - 1).max(1); }
        o
    }
    pub fn holds_china(&self, side: usize) -> bool { self.china == side as u8 }
    pub fn china_playable(&self, side: usize) -> bool { self.china == side as u8 && self.china_up }
    /// Space Race ability holder: first to reach `k` while the opponent has not.
    pub fn space_ability(&self, side: usize, k: usize) -> bool {
        self.space_first[k] == side as i8 && (self.space[opp(side)] as usize) < k
    }
    pub fn card_known(&self, side: usize, card: u8) -> bool { self.known_by_opp[side].contains(card) }
    pub fn remove_from_hand(&mut self, side: usize, card: u8) -> bool {
        self.known_by_opp[side].remove(card);
        self.hands[side].remove(card)
    }
}

impl State {
    pub fn snapshot_presence(&mut self) {
        for s in 0..2 { let mut m = 0u128; for c in 0..NC { if self.inf[c][s] > 0 { m |= 1u128 << c; } } self.pres_mask[s] = m; }
    }
}

impl Default for State { fn default() -> Self { State::new() } }
