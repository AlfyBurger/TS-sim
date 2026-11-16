pub const US: usize = 0;
pub const USSR: usize = 1;
pub fn opp(s: usize) -> usize { 1 - s }
pub fn side_name(s: usize) -> &'static str { if s == US { "US" } else { "USSR" } }

// Regions
pub const EUROPE: u8 = 0;
pub const ASIA: u8 = 1;
pub const MIDEAST: u8 = 2;
pub const AFRICA: u8 = 3;
pub const CENTAM: u8 = 4;
pub const SOUTHAM: u8 = 5;
pub const REGION_NAMES: [&str; 6] = ["Europe", "Asia", "Middle East", "Africa", "Central America", "South America"];

pub struct Country {
    pub name: &'static str,
    pub region: u8,
    pub ee: bool,
    pub we: bool,
    pub sea: bool,
    pub stab: i8,
    pub bg: bool,
    pub adj_us: bool,
    pub adj_ussr: bool,
    pub adj: &'static [usize],
}

pub struct CardInfo {
    pub id: u8,
    pub name: &'static str,
    pub ops: i8,
    pub era: u8,
    pub side: u8, // 0 both, 1 US, 2 USSR
    pub star: bool,
    pub scoring: bool,
}

impl CardInfo {
    /// Side associated with the Event, None for both.
    pub fn owner(&self) -> Option<usize> {
        match self.side { 1 => Some(US), 2 => Some(USSR), _ => None }
    }
}
