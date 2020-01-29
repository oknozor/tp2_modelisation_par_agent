#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub x: f32,
    pub y: f32,
    pub cell_size: f32,
    pub agents: u32,
    pub density: u8,
    pub grid: bool,
    pub fish: bool,
    pub fish_density: u32,
    pub shark_density: u32,
    pub fish_breed_time: u8,
    pub shark_breed_time: u8,
    pub shark_starve_time: u8,
}
