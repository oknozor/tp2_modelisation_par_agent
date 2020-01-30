#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub x: f32,
    pub y: f32,
    pub cell_size: f32,
    pub borderless: bool,
    pub grid: bool,
    pub fish_number: u32,
    pub shark_number: u32,
    pub fish_breed_time: i32,
    pub shark_breed_time: i32,
    pub shark_starve_time: i32,
}
