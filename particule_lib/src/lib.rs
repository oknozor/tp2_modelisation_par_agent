pub mod environment;
pub mod fish;
pub mod sma;

pub type AgentImpl = Box<dyn Agent>;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Coord(pub i32, pub i32);

pub trait Agent {
    fn decide(&self, neighbors: Vec<Coord>) -> Option<(Coord, Coord)>;
    fn update(&mut self, new_position: &Coord);
    fn get_kind(&self) -> AgentKind;
    fn coordinate(&self) -> Coord;
    fn previous_coordinate(&self) -> Coord;
    fn breed_count_down(&self) -> u8;
    fn set_breed_count_down(&mut self, value: u8);
}

#[derive(Clone, Copy)]
pub enum AgentKind {
    Shark,
    Fish,
}
