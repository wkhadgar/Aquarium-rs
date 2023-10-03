use crate::bodies::{Position, Vision};
use crate::fishes::{Fish, Plant};
use crate::vectors::Vector2;
use fltk::button::Button;
use fltk::enums::FrameType;
use fltk::frame::Frame;
use fltk::group::{Group, Pack};
use fltk::input::Input;
use fltk::{app, prelude::*, window::Window};
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

mod bodies;
mod fishes;
mod vectors;

struct ScreenControl {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

impl ScreenControl {
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
    offset_window: Vector2,
    offset_zoom: f64,

    textures: Vec<Texture<'a>>,

    plants: Vec<Plant>,
    preys: Vec<Fish>,
    predators: Vec<Fish>,
}

impl<'a> Aquarium<'a> {
    fn create() -> Result<Self, String> {
        Ok(Self {
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
        parameters: MutexGuard<SimParam>,
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
        for _i in 0..parameters.pl_pop as i32 {
            let new_pos = screen_center + Vector2::random_in_radius(parameters.pl_spread);
            self.plants
                .push(Plant::new(new_pos, parameters.pl_mass / 10.0));
        }

        for _i in 0..parameters.pr_pop as i32 {
            let new_pos = screen_center + Vector2::random_in_radius(parameters.pr_spread);
            self.preys.push(Fish::new(
                new_pos,
                parameters.pr_mass / 10.0,
                parameters.pr_vis_a,
                parameters.pr_vis_d,
                parameters.pr_p_speed,
            ));
        }

        for _i in 0..parameters.pd_pop as i32 {
            let new_pos = screen_center + Vector2::random_in_radius(parameters.pd_spread);
            self.predators.push(Fish::new(
                new_pos,
                parameters.pd_mass / 10.0,
                parameters.pd_vis_a,
                parameters.pd_vis_d,
                parameters.pd_p_speed,
            ));
        }

        self
    }

    fn check_proximity<O: Vision, T: Position>(
        origin: &O,
        vec: &Vec<T>,
    ) -> Option<(Vector2, Vector2)> {
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
            let closest_prey = Aquarium::check_proximity(&self.plants[i], &self.preys);

            match closest_prey {
                None => {
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
                Some((prey_pos, _)) => {
                    self.plants[i].health -= 1;
                    if self.plants[i].health < 3 {
                        self.plants.swap_remove(i);
                        continue;
                    }
                }
            }
            self.plants[i].draw(canvas, &self.textures[0], self.offset_window);
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
                    self.preys[i].flee(preadator_pos);
                    if (preadator_pos - self.preys[i].pos()).length() < 10.0 {
                        self.preys.swap_remove(i);
                        continue;
                    }
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

    fn process_screen_sliding(&mut self, arrows: &ScreenControl) {
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

struct SimParam {
    pl_mass: f64,
    pr_mass: f64,
    pd_mass: f64,
    pl_pop: f64,
    pr_pop: f64,
    pd_pop: f64,
    pl_spread: f64,
    pr_spread: f64,
    pd_spread: f64,
    pr_vis_a: f64,
    pd_vis_a: f64,
    pr_vis_d: f64,
    pd_vis_d: f64,
    pr_p_speed: f64,
    pd_p_speed: f64,
    ready: bool,
}

impl SimParam {
    fn default() -> Self {
        SimParam {
            pl_mass: 0.0,
            pr_mass: 0.0,
            pd_mass: 0.0,
            pl_pop: 0.0,
            pr_pop: 0.0,
            pd_pop: 0.0,
            pl_spread: 0.0,
            pr_spread: 0.0,
            pd_spread: 0.0,
            pr_vis_a: 0.0,
            pd_vis_a: 0.0,
            pr_vis_d: 0.0,
            pd_vis_d: 0.0,
            pr_p_speed: 0.0,
            pd_p_speed: 0.0,
            ready: false,
        }
    }
}

fn new_input_field<T: Into<Option<&'static str>>>(
    x: i32,
    y: i32,
    characters: u8,
    title: T,
    default: i32,
    group: &mut Group,
) -> Input {
    let mut field = Input::new(x, y, (characters * 10) as i32, 30, title);
    field.set_value(&*format!("{default}"));
    group.add(&field);

    field
}
fn param_set(mutex: Arc<Mutex<SimParam>>) {
    app::App::default();
    let mut win = Window::new(100, 100, 500, 320, "Aquarium PPP-Sim");

    let mut plant_group = Group::new(10, 20, 480, 40, "Parâmetros para as plantas:");
    plant_group.set_frame(FrameType::DownBox);
    let plant_mass = new_input_field(110, 25, 4, "Massa (g):", 250, &mut plant_group);
    let plant_pop = new_input_field(240, 25, 6, "População:", 10, &mut plant_group);
    let plant_spread = new_input_field(420, 25, 4, "Raio inicial (px):", 500, &mut plant_group);

    let mut prey_group = Group::new(10, 85, 480, 80, "Parâmetros para as presas:");
    prey_group.set_frame(FrameType::DownBox);
    let prey_mass = new_input_field(110, 90, 4, "Massa (g):", 200, &mut prey_group);
    let prey_pop = new_input_field(240, 90, 6, "População:", 5, &mut prey_group);
    let prey_spread = new_input_field(420, 90, 4, "Raio inicial (px):", 300, &mut prey_group);
    let prey_vis_angle = new_input_field(110, 130, 4, "Visão (°):", 179, &mut prey_group);
    let prey_vis_dist = new_input_field(240, 130, 6, "Visão (px):", 800, &mut prey_group);
    let prey_peak_speed = new_input_field(420, 130, 4, "Velocidade máx:", 5, &mut prey_group);

    let mut pred_group = Group::new(10, 190, 480, 80, "Parâmetros para os predadores:");
    pred_group.set_frame(FrameType::DownBox);
    let pred_mass = new_input_field(110, 195, 4, "Massa (g):", 800, &mut pred_group);
    let pred_pop = new_input_field(240, 195, 6, "População:", 2, &mut pred_group);
    let pred_spread = new_input_field(420, 195, 4, "Raio inicial (px):", 500, &mut pred_group);
    let pred_vis_angle = new_input_field(110, 235, 4, "Visão (°):", 120, &mut pred_group);
    let pred_vis_dist = new_input_field(240, 235, 6, "Visão (px):", 1000, &mut pred_group);
    let pred_peak_speed = new_input_field(420, 235, 4, "Velocidade máx:", 6, &mut pred_group);

    win.add(&plant_group);
    win.add(&prey_group);
    win.add(&pred_group);

    let mut save_button = Button::new(200, 280, 100, 30, "Simular!");
    win.add(&save_button);

    save_button.set_callback(move |_| {
        let mut guard = mutex.lock().unwrap();

        *guard = SimParam {
            pl_mass: plant_mass.value().parse().unwrap_or(1.0),
            pr_mass: prey_mass.value().parse().unwrap_or(1.0),
            pd_mass: pred_mass.value().parse().unwrap_or(1.0),
            pl_pop: plant_pop.value().parse().unwrap_or(0.0),
            pr_pop: prey_pop.value().parse().unwrap_or(0.0),
            pd_pop: pred_pop.value().parse().unwrap_or(0.0),
            pl_spread: plant_spread.value().parse().unwrap_or(10.0),
            pr_spread: prey_spread.value().parse().unwrap_or(10.0),
            pd_spread: pred_spread.value().parse().unwrap_or(10.0),
            pr_vis_a: prey_vis_angle.value().parse().unwrap_or(0.0),
            pd_vis_a: pred_vis_angle.value().parse().unwrap_or(0.0),
            pr_vis_d: prey_vis_dist.value().parse().unwrap_or(0.0),
            pd_vis_d: pred_vis_dist.value().parse().unwrap_or(0.0),
            pr_p_speed: prey_peak_speed.value().parse().unwrap_or(0.0),
            pd_p_speed: pred_peak_speed.value().parse().unwrap_or(0.0),
            ready: true,
        };

        println!("Simulação iniciando...");
        app::quit();
    });

    win.end();
    win.show();

    app::run().unwrap();
}

pub fn main() -> Result<(), String> {
    let mutex: Arc<Mutex<SimParam>> = Arc::new(Mutex::new(SimParam::default()));
    param_set(mutex.clone());
    let guard = mutex.lock().unwrap();
    if !guard.ready {
        return Err(String::from("Simulação cancelada."));
    }

    let mut pl_p = guard.pl_pop;
    let mut pr_p = guard.pr_pop;
    let mut pd_p = guard.pd_pop;

    let (mut canvas, mut event_pump) = sdl_init()?;
    let tex_creator = &canvas.texture_creator();
    let mut aquarium = Aquarium::create()?;

    aquarium.init(&mut canvas, tex_creator, guard);

    let mut arrows = ScreenControl::new();
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

        let time_act = ticks > 1 * fps;
        aquarium.process_plants(&mut canvas, time_act);
        aquarium.process_preys(&mut canvas);
        aquarium.process_predators(&mut canvas);
        aquarium.process_screen_sliding(&arrows);

        if time_act {
            ticks = 0;
        }

        update_screen(&mut canvas, Some(fps));

        if pl_p as usize != aquarium.plants.len()
            || pr_p as usize != aquarium.preys.len()
            || pd_p as usize != aquarium.predators.len()
        {
            pl_p = aquarium.plants.len() as f64;
            pr_p = aquarium.preys.len() as f64;
            pd_p = aquarium.predators.len() as f64;

            println!(
                "{}, {}, {}",
                aquarium.plants.len(),
                aquarium.predators.len(),
                aquarium.preys.len()
            );
        } else {}
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