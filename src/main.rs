use crate::fishes::Fish;
use crate::vectors::Vector2;
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
            offset_window: Vector2::null(),
            offset_zoom: 0.0,
            textures: vec![],
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

        self
    }
}

pub fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = sdl_init()?;
    let tex_creator = &canvas.texture_creator();

    let mut aquarium = Aquarium::create(100, 100, 100)?;
    aquarium.init(&mut canvas, tex_creator);

    let mut test_fish = Fish::new(Vector2::new(200.0, 200.0), 20.0, 20.0);

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
                    aquarium.offset_window += Vector2::new(0.0, 5.0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    aquarium.offset_window += Vector2::new(0.0, -5.0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    aquarium.offset_window += Vector2::new(-5.0, 0.0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    aquarium.offset_window += Vector2::new(5.0, 0.0);
                }
                _ => {}
            }
        }

        fill_bg(&mut canvas, Color::BLACK);

        test_fish.wander();
        test_fish.draw(&mut canvas, &aquarium.textures[1], aquarium.offset_window);

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