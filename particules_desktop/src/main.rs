#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;

use nannou::prelude::*;

use particule_lib::sma::Sma;

mod user_config;

lazy_static! {
    pub static ref CONFIG: user_config::Config = {
        let mut file = File::open("config.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).expect("expected json")
    };
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Grid {
    sma: Sma
}

impl Grid {
    fn new() -> Self {
        let mut sma = Sma::new(CONFIG.x as i32, CONFIG.y as i32);
        sma.gen_agents(2);
        Grid {sma}
    }

    // This is the easy part, just draw the cells fill white if 1, black if 0
    fn display(&self, draw: &app::Draw) {
        let offset = CONFIG.cell_size;

        let width = CONFIG.x * offset;
        let height = CONFIG.y * offset;
        draw.rect()
            .w_h(width, height)
            .rgb(1.0, 1.0, 1.0)
            .stroke(rgb(0.0, 0.0, 0.0));

        self.sma.environment.agents.iter().for_each(|agent| {
            let x = (agent.0).0 as f32;
            let y = (agent.0).1 as f32;
            let x = (x * CONFIG.cell_size) - width / 2.0 + offset / 2.0;
            let y = (y * CONFIG.cell_size) - height / 2.0 + offset / 2.0;
            self.display_agent(&draw, (0.0, 1.0, 0.0), x, y);
        });
    }

    fn display_agent(&self, draw: &app::Draw, color: (f32, f32, f32), x: f32, y: f32) {
        draw.rect()
            .x_y(x, y)
            .w_h(CONFIG.cell_size, CONFIG.cell_size)
            .rgb(color.0, color.1, color.2);
    }
}

struct Model {
    pub grid: Grid,
    pub pause: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_maximized(true)
        .event(window_event)
        .view(view)
        .build()
        .unwrap();

        let grid = Grid::new();
        Model { grid, pause: true }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.pause {
        model.grid.sma.tick();
        model.pause = true;
    }
}

fn view(app: &App, m: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(rgb(1.0, 1.0, 1.0));

    m.grid.display(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn window_event(_: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_) => model.pause = false,
        _ => {}
    }
}
