pub mod environment;
pub mod wator;
pub mod sma;
pub mod trace;
pub mod pacman;

use std::convert::TryInto;
use std::ops;

static mut FISH_BREED_COUNT_DOWN: i32 = 0;
static mut SHARK_BREED_COUNT_DOWN: i32 = 0;
static mut SHARK_STARVE_COUNT_DOWN: i32 = 0;
static mut MAX_HEIGTH: i32 = 0;
static mut MAX_WIDTH: i32 = 0;
static mut BORDERLESS: bool = false;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Coord(pub i32, pub i32);

pub type AgentImpl = Box<dyn Agent>;

pub trait CloneBoxed {
    fn clone_boxed(&self) -> Box<dyn CloneBoxed>;
}

impl<T> CloneBoxed for T
where
    T: 'static + Clone + Agent + Fn() + Agent + Sized,
{
    fn clone_boxed(&self) -> Box<dyn CloneBoxed> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Agent> {
    fn clone(&self) -> Self {
        self.as_ref().clone_boxed()
    }
}

pub trait Agent {
    fn decide(&self, neighbors: &Vec<Cell>) -> Decision;
    fn update(&mut self);
    fn get_kind(&self) -> AgentKind;
    fn get_color(&self) -> (f32, f32, f32);
    fn coordinate(&self) -> Coord;
    fn set_coordinate(&mut self, coord: Coord);
    fn breed(&mut self) -> AgentImpl;
    fn reset_starve_count_down(&mut self);
    fn clone_boxed(&self) -> Box<dyn Agent>;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AgentKind {
    Shark,
    Fish,
}

#[derive(Debug)]
pub enum Decision {
    EatAndMove(Coord, Coord),
    EatAndBreed(Coord, Coord),
    Move(Coord, Coord),
    MoveAndBreed(Coord, Coord),
    Starve(Coord),
    Stall(Coord),
}

impl Decision {
    pub fn get_origin(&self) -> &Coord {
        match self {
            Decision::EatAndMove(from, _) => from,
            Decision::EatAndBreed(from, _) => from,
            Decision::Move(from, _) => from,
            Decision::MoveAndBreed(from, _) => from,
            Decision::Starve(from) => from,
            Decision::Stall(from) => from,
        }
    }
}
#[derive(Clone)]
pub enum Cell {
    Empty(Coord),
    Filled(AgentImpl),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Empty(_) => true,
            _ => false,
        }
    }

    pub fn is_fish(&self) -> bool {
        match self {
            Cell::Filled(a) => match a.get_kind() {
                AgentKind::Fish => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_coord_unchecked(&self) -> Coord {
        match self {
            Cell::Filled(a) => a.coordinate(),
            Cell::Empty(coord) => *coord,
        }
    }
}

impl TryInto<AgentImpl> for Cell {
    type Error = String;
    fn try_into(self) -> Result<AgentImpl, Self::Error> {
        match self {
            Cell::Filled(agent) => Ok(agent),
            _ => Err("Can't convert empty cell to agent".into()),
        }
    }
}

impl TryInto<Coord> for Cell {
    type Error = String;
    fn try_into(self) -> Result<Coord, Self::Error> {
        match self {
            Cell::Empty(coord) => Ok(coord),
            _ => Err("Can't convert agent cell to coord".into()),
        }
    }
}

impl TryInto<Coord> for &Cell {
    type Error = String;
    fn try_into(self) -> Result<Coord, Self::Error> {
        match self {
            Cell::Empty(coord) => Ok(*coord),
            _ => Err("Can't convert agent cell to coord".into()),
        }
    }
}

impl Coord {
    pub fn as_idx(&self) -> usize {
        (self.1 * max_width() + self.0) as usize
    }

    pub fn from_idx(idx: i32) -> Coord {
        let x = idx % max_width();
        let y = (idx - x) / max_width();
        Coord(x, y)
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        let res = Coord(self.0 + rhs.0, self.1 + rhs.1);
        if res.0 < 0 || res.0 >= max_width() || res.1 < 0 || res.1 >= max_height() {
            self
        } else {
            res
        }
    }
}

// We use mul for toric world
impl ops::Mul<Coord> for Coord {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Coord {
        let res = Coord(self.0 + rhs.0, self.1 + rhs.1);
        let x;
        let y;
        if res.0 < 0 {
            x = max_width() - 1;
        } else if res.0 >= max_width() {
            x = 0;
        } else {
            x = res.0;
        }

        if res.1 < 0 {
            y = max_height() - 1;
        } else if res.1 >= max_height() {
            y = 0;
        } else {
            y = res.1;
        }

        Coord(x, y)
    }
}

fn get_fish_breed_time() -> i32 {
    unsafe { FISH_BREED_COUNT_DOWN }
}

fn get_shark_breed_time() -> i32 {
    unsafe { SHARK_BREED_COUNT_DOWN }
}

fn get_shark_starve_time() -> i32 {
    unsafe { SHARK_STARVE_COUNT_DOWN }
}

fn max_height() -> i32 {
    unsafe { MAX_HEIGTH }
}

fn max_width() -> i32 {
    unsafe { MAX_WIDTH }
}

fn borderless() -> bool {
    unsafe { BORDERLESS }
}
