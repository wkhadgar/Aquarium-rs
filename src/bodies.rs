use crate::vectors::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub struct Body {
    pub mass: f64,
    pub position: Vector2,
    pub velocity: Vector2,
    pub velocity_norm: Vector2,
    pub rect: Rect,
    pub collision_rect: Rect,
}

impl Body {
    pub fn get_size(mass: f64) -> u32 {
        (4.5 * mass.sqrt()) as u32
    }

    pub fn new(mass: f64, position: Vector2) -> Self {
        let vel = Vector2::random_in_radius(1.0);
        let (x, y): (f64, f64) = position.get_components();
        let (vx, vy): (f64, f64) = vel.get_components();
        let size = Body::get_size(mass);
        Self {
            mass,
            position,
            velocity: vel,
            velocity_norm: vel.norm(),
            rect: Rect::new(x as i32, y as i32, size, size),
            collision_rect: Rect::new(
                x as i32 + (size as i32 / 4) * (if vx < 0.0 { -1 } else { 1 }),
                y as i32 + (size as i32 / 4) * (if vy < 0.0 { -1 } else { 1 }),
                size / 2,
                size / 2,
            ),
        }
    }

    fn rescale(&mut self) {
        let rect_size = Body::get_size(self.mass);
        let center = self.rect.center();
        let (vx, vy) = self.velocity.get_components();
        self.rect.resize(rect_size, rect_size);
        self.rect.center_on(center);
        self.collision_rect.resize(rect_size / 2, rect_size / 2);
        self.collision_rect.offset(
            self.rect.x() + (rect_size as i32 / 4) * (if vx < 0.0 { -1 } else { 1 }),
            self.rect.y() + (rect_size as i32 / 4) * (if vy < 0.0 { -1 } else { 1 }),
        );
    }

    pub fn grow(&mut self, mass_gained: f64) {
        self.mass += mass_gained;
        self.rescale();
    }

    pub fn shrink(&mut self, mass_loss: f64) {
        self.mass -= mass_loss;
        self.rescale();
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, texture: &Texture, debug: bool) {
        let angle = self.velocity.angle();
        canvas
            .copy_ex(
                texture,
                None,
                Some(self.rect),
                angle,
                None,
                false,
                (angle > 90.0) && (angle < 270.0),
            )
            .unwrap();

        if debug {
            let color = canvas.draw_color();
            canvas.set_draw_color(Color::RED);
            canvas.draw_rect(self.rect).unwrap();
            canvas.set_draw_color(Color::YELLOW);
            canvas.draw_rect(self.collision_rect).unwrap();
            canvas.set_draw_color(color);
        }
    }
}

pub trait Vision {
    fn in_sight(&self, target: Vector2) -> f64;
}

pub trait Position {
    fn pos(&self) -> Vector2;

    fn vel(&self) -> Vector2 {
        Vector2::default()
    }
}