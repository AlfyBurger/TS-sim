use crate::rng::Streams;
use crate::state::State;

/// How a card is played in an Action Round.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Use { Event, Ops, Space, Pass }

#[derive(Clone, Copy, Debug)]
pub struct Play {
    pub card: u8,
    pub how: Use,
    pub event_first: bool,   // for an opponent's card played for Ops
    pub uni_with: u8,        // UN Intervention: the opponent card played with it (0 = none)
}

/// Where Ops may go, as declared before use (for China / Vietnam Revolts bonuses).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scope { Any, Asia, Sea }

#[derive(Clone, Debug)]
pub enum OpsChoice { Place(Scope), Coup(usize), Realign(Scope), Space, None }

/// What the ops are allowed to do.
#[derive(Clone, Copy, Debug)]
pub struct OpsSpec {
    pub base: i32,           // Ops value with modifiers (before Asia/SEA bonuses)
    pub card: u8,            // card being used (0 = Event-granted Ops)
    pub allow_coup: bool,
    pub allow_realign: bool,
    pub allow_place: bool,
    pub allow_space: bool,
}

pub trait Agent {
    fn name(&self) -> String;
    /// Choose a Headline card from the hand.
    fn headline(&mut self, st: &State, me: usize, opp_shown: Option<u8>, rng: &mut Streams) -> u8;
    /// Choose the play for an Action Round.
    fn action(&mut self, st: &State, me: usize, rng: &mut Streams) -> Play;
    /// Choose how to use Ops.
    fn ops(&mut self, st: &State, me: usize, spec: &OpsSpec, rng: &mut Streams) -> OpsChoice;
    /// Choose among resulting positions (index into `opts`).
    fn choose(&mut self, st: &State, me: usize, opts: &[State], rng: &mut Streams) -> usize;
    /// Next Realignment target among `targets` (ops = rolls left incl. this one).
    fn realign_target(&mut self, st: &State, me: usize, targets: &[usize], rng: &mut Streams) -> usize;
    /// Next Influence placement among `targets` (cost already legal).
    fn place_target(&mut self, st: &State, me: usize, targets: &[usize], rng: &mut Streams) -> usize;
    /// Next opponent Influence point to remove among `targets`.
    fn remove_target(&mut self, st: &State, me: usize, targets: &[usize], rng: &mut Streams) -> usize;
    /// Choose among lotteries: each option is a list of (probability, resulting position).
    fn choose_lottery(&mut self, st: &State, me: usize, opts: &[Vec<(f64, State)>], rng: &mut Streams) -> usize;
    /// Pick a card among `cards` for a purpose; default: via `choose` over the given result positions.
    fn yes_no(&mut self, st: &State, me: usize, yes: &State, no: &State, rng: &mut Streams) -> bool {
        self.choose(st, me, &[*yes, *no], rng) == 0
    }
}
