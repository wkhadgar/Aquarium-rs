use crate::vectors::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use sdl2::sys::{SDL_Renderer, SDL_Texture, Window};

pub struct Body {
    pub health: f64,
    pub mass: f64,
    pub position: Vector2,
    pub velocity: Vector2,
    pub velocity_norm: Vector2,
    pub rect: Rect,
    pub collision_rect: Rect,
}

impl Body {
    pub fn new(health: f64, mass: f64, position: Vector2) -> Self {
        let vel = Vector2::random_in_radius(1.0);
        let (x, y): (f64, f64) = position.get_components();
        let (vx, vy): (f64, f64) = vel.get_components();
        Self {
            health,
            mass,
            position,
            velocity: vel,
            velocity_norm: vel.norm(),
            rect: Rect::new(x as i32, y as i32, mass as u32, mass as u32),
            collision_rect: Rect::new(
                (x + (mass / 4.0) * (if vx < 0.0 { -1.0 } else { 1.0 })) as i32,
                (y + (mass / 4.0) * (if vy < 0.0 { -1.0 } else { 1.0 })) as i32,
                (mass / 2.0) as u32,
                (mass / 2.0) as u32,
            ),
        }
    }

    fn rescale(&mut self) {
        let rect_scaling = 5.0 * self.mass.sqrt();
        self.rect.resize(rect_scaling as u32, rect_scaling as u32);
        self.rect
            .resize(rect_scaling as u32 / 2, rect_scaling as u32 / 2);
    }

    pub fn grow(&mut self, mass_gained: f64, health_gained: f64) {
        self.mass += mass_gained;
        self.health += health_gained;
        self.rescale();
    }

    pub fn shrink(&mut self, mass_loss: f64, health_loss: f64) {
        self.mass -= mass_loss;
        self.health -= health_loss;
        self.rescale();
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, texture: &Texture, debug: bool, offset: Vector2) {
        let angle = self.velocity.angle();
        let mut display_rect = self.rect;
        let (offset_x, offset_y) = offset.get_components();
        display_rect.x += offset_x as i32;
        display_rect.y += offset_y as i32;

        canvas
            .copy_ex(
                texture,
                None,
                Some(display_rect),
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