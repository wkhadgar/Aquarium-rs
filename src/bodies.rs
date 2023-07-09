use crate::vectors::Vector2;
use sdl2::rect::Rect;
use sdl2::render::Texture;

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
        let (x, y) = position.get_components();
        let (vx, vy) = vel.get_components();
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

    pub fn draw(&self, texture: Texture) {}
}