use crate::bodies::{Body, Position, Vision};
use crate::fishes::{Fish, Plant};
use crate::vectors::Vector2;
use rand::random;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::sys::SDL_Texture;
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use std::time::Duration;

mod bodies;
mod fishes;
mod vectors;

struct ArrowStates {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

impl ArrowStates {
    fn new() -> Self {
        Self {
            up: false,
            down: false,
            right: false,
            left: false,
        }
    }
}

struct Aquarium<'a> {
    plant_start_amount: u32,
    prey_start_amount: u32,
    pred_start_amount: u32,

    offset_window: Vector2,
    offset_zoom: f64,

    textures: Vec<Texture<'a>>,

    plants: Vec<Plant>,
    preys: Vec<Fish>,
    predators: Vec<Fish>,
}

impl<'a> Aquarium<'a> {
    fn create(
        plant_start_amount: u32,
        prey_start_amount: u32,
        pred_start_amount: u32,
    ) -> Result<Self, String> {
        Ok(Self {
            plant_start_amount,
            prey_start_amount,
            pred_start_amount,
            offset_window: Vector2::default(),
            offset_zoom: 0.0,
            textures: vec![],

            plants: vec![],
            preys: vec![],
            predators: vec![],
        })
    }

    fn init(
        &mut self,
        canvas: &mut WindowCanvas,
        tex_creator: &'a TextureCreator<WindowContext>,
    ) -> &mut Self {
        self.textures
            .push(tex_creator.load_texture("assets/seaweed.png").unwrap());
        self.textures
            .push(tex_creator.load_texture("assets/fish.png").unwrap());
        self.textures
            .push(tex_creator.load_texture("assets/shark.png").unwrap());

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let screen_center = Vector2::new(1820.0 / 2.0, 1080.0 / 2.0);
        for _i in 0..self.plant_start_amount {
            let new_pos = screen_center + Vector2::random_in_radius(500.0);
            self.plants.push(Plant::new(new_pos, 25.0));
        }

        for _i in 0..self.prey_start_amount {
            let new_pos = screen_center + Vector2::random_in_radius(300.0);
            self.preys.push(Fish::new(new_pos, 20.0, 170.0, 800.0, 5.0));
        }

        for _i in 0..self.pred_start_amount {
            let new_pos = screen_center + Vector2::random_in_radius(500.0);
            self.predators
                .push(Fish::new(new_pos, 80.0, 90.0, 1000.0, 8.0));
        }

        self
    }

    fn check_proximity<O: Vision, T: Position>(
        origin: &O,
        vec: &Vec<T>,
    ) -> (Option<(Vector2, Vector2)>) {
        let mut closest_tgt = None;
        let mut min_dist = f64::MAX;
        for tgt in vec {
            let tgt_dist_sqr = origin.in_sight(tgt.pos());

            if (tgt_dist_sqr > 0.0) && (tgt_dist_sqr < min_dist) {
                min_dist = tgt_dist_sqr;
                closest_tgt = Some((tgt.pos(), tgt.vel()));
            }
        }

        closest_tgt
    }

    fn process_plants(&mut self, canvas: &mut WindowCanvas, do_grow: bool) {
        let mut i = self.plants.len();
        while i != 0 {
            i -= 1;
            self.plants[i].draw(canvas, &self.textures[0], self.offset_window);

            if do_grow {
                let rootlings = self.plants[i].grow();
                match rootlings {
                    None => {}
                    Some((rootling_1, rootling_2)) => {
                        self.plants.push(rootling_1);
                        self.plants.push(rootling_2);
                    }
                }
            }
        }
    }

    fn process_preys(&mut self, canvas: &mut WindowCanvas) {
        let mut i = self.preys.len();
        while i != 0 {
            i -= 1;
            let closest_plant = Aquarium::check_proximity(&self.preys[i], &self.plants);
            let closest_predator = Aquarium::check_proximity(&self.preys[i], &self.predators);

            match closest_predator {
                Some((preadator_pos, predator_vel)) => {
                    self.preys[i].evade(preadator_pos, predator_vel);
                }
                None => match closest_plant {
                    Some((plant_pos, _)) => {
                        self.preys[i].arrive(plant_pos);
                    }
                    None => {
                        self.preys[i].wander();
                    }
                },
            }

            self.preys[i].draw(canvas, &self.textures[1], self.offset_window);
        }
    }

    fn process_predators(&mut self, canvas: &mut WindowCanvas) {
        let mut i = self.predators.len();
        while i != 0 {
            i -= 1;
            let closest_prey = Aquarium::check_proximity(&self.predators[i], &self.preys);

            match closest_prey {
                Some((prey_pos, prey_vel)) => {
                    self.predators[i].pursuit(prey_pos, prey_vel);
                }
                None => {
                    self.predators[i].wander();
                }
            }

            self.predators[i].draw(canvas, &self.textures[2], self.offset_window);
        }
    }

    fn process_screen_sliding(&mut self, arrows: &ArrowStates) {
        let speed = 6.0;
        if arrows.up {
            self.offset_window.offset(0.0, speed);
        }
        if arrows.down {
            self.offset_window.offset(0.0, -speed);
        }
        if arrows.right {
            self.offset_window.offset(-speed, 0.0);
        }
        if arrows.left {
            self.offset_window.offset(speed, 0.0);
        }
    }
}

pub fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = sdl_init()?;
    let tex_creator = &canvas.texture_creator();

    let mut aquarium = Aquarium::create(0, 100, 1)?;
    aquarium.init(&mut canvas, tex_creator);

    let mut arrows = ArrowStates::new();
    let fps = 60;
    let mut ticks = 0;
    'running: loop {
        ticks += 1;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    arrows.up = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    arrows.down = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    arrows.right = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    arrows.left = true;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    arrows.up = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    arrows.down = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    arrows.right = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    arrows.left = false;
                }

                _ => {}
            }
        }

        fill_bg(&mut canvas, Color::BLACK);

        let time_act = ticks > 2 * fps;
        aquarium.process_plants(&mut canvas, time_act);
        aquarium.process_preys(&mut canvas);
        aquarium.process_predators(&mut canvas);
        aquarium.process_screen_sliding(&arrows);

        if time_act {
            ticks = 0;
        }

        update_screen(&mut canvas, Some(fps));
    }

    Ok(())
}

fn fill_bg(w_canvas: &mut WindowCanvas, color: Color) {
    w_canvas.set_draw_color(color);
    w_canvas.clear();
}

fn update_screen(w_canvas: &mut WindowCanvas, fps: Option<u64>) {
    w_canvas.present();
    match fps {
        None => {
            return;
        }
        Some(_) => {}
    }
    std::thread::sleep(Duration::from_millis(1000 / fps.unwrap()));
}

fn sdl_init() -> Result<(WindowCanvas, EventPump), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Behaviours", 800, 600)
        .position_centered()
        .opengl()
        .fullscreen_desktop()
        .build()
        .map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    Ok((canvas, event_pump))
}