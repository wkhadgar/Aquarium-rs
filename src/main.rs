use crate::bodies::{Position, Vision};
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

        for _i in 0..self.plant_start_amount {
            self.plants.push(Plant::new(
                Vector2::new((random::<f64>() * 1820.0), (random::<f64>() * 1080.0)),
                20.0,
            ));
        }

        for _i in 0..self.prey_start_amount {
            self.preys.push(Fish::new(
                Vector2::new((random::<f64>() * 1820.0), (random::<f64>() * 1080.0)),
                20.0,
                20.0,
            ));
        }

        for _i in 0..self.pred_start_amount {
            self.predators.push(Fish::new(
                Vector2::new((random::<f64>() * 1820.0), (random::<f64>() * 1080.0)),
                20.0,
                20.0,
            ));
        }

        self
    }

    fn check_proximity<O: Vision, T: Position>(origin: &O, vec: &Vec<T>) -> Option<Vector2> {
        let mut closest_tgt: Option<Vector2> = None;
        let mut min_dist = f64::MAX;
        for tgt in vec {
            let tgt_dist_sqr = origin.in_sight(tgt.pos());

            if (tgt_dist_sqr > 0.0) && (tgt_dist_sqr < min_dist) {
                min_dist = tgt_dist_sqr;
                closest_tgt = Some(tgt.pos());
            }
        }

        closest_tgt
    }

    fn process_preys(&mut self, canvas: &mut WindowCanvas) {
        for i in 0..(self.preys.len() - 1) {
            let closest_plant = Aquarium::check_proximity(&self.preys[i], &self.plants);

            match closest_plant {
                Some(Vector2) => {
                    self.preys[i].seek(Vector2);
                }
                None => {}
            }

            self.preys[i].draw(canvas, &self.textures[1], self.offset_window);
        }

        for i in 0..(self.plants.len() - 1) {
            self.plants[i].draw(canvas, &self.textures[0], self.offset_window);
        }
    }

    fn process_screen_sliding(&mut self, up: bool, down: bool, right: bool, left: bool) {
        let speed = 4.0;
        if up {
            self.offset_window.offset(0.0, speed);
        }
        if down {
            self.offset_window.offset(0.0, -speed);
        }
        if right {
            self.offset_window.offset(-speed, 0.0);
        }
        if left {
            self.offset_window.offset(speed, 0.0);
        }
    }
}

pub fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = sdl_init()?;
    let tex_creator = &canvas.texture_creator();

    let mut aquarium = Aquarium::create(100, 100, 100)?;
    aquarium.init(&mut canvas, tex_creator);

    let (mut slide_left, mut slide_right, mut slide_up, mut slide_down) =
        (false, false, false, false);
    'running: loop {
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
                    slide_up = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    slide_down = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    slide_right = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    slide_left = true;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    slide_up = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    slide_down = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    slide_right = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    slide_left = false;
                }

                _ => {}
            }
        }

        fill_bg(&mut canvas, Color::BLACK);

        aquarium.process_preys(&mut canvas);
        aquarium.process_screen_sliding(slide_up, slide_down, slide_right, slide_left);
        update_screen(&mut canvas, 60);
    }

    Ok(())
}

fn fill_bg(w_canvas: &mut WindowCanvas, color: Color) {
    w_canvas.set_draw_color(color);
    w_canvas.clear();
}

fn update_screen(w_canvas: &mut WindowCanvas, fps: u64) {
    w_canvas.present();
    std::thread::sleep(Duration::from_millis(1000 / fps));
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