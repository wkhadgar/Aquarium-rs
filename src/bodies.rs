use crate::vectors::Vector2;
use std::cmp::min;

struct Body {
    health: f32,

    mass: f64,
    max_force: f64,

    peak_speed: f64,
    default_speed: f64,
    current_speed: f64,

    //collision_rect,
    //rect;
    //texture;
    position: Vector2,
    velocity: Vector2,
}

impl Body {
    pub fn new(pos: Vector2, mass: f64, peak_speed: f64) -> Self {
        Self {
            health: 100.0,
            mass,
            max_force: 0.0,
            peak_speed,
            default_speed: 0.0,
            current_speed: 0.0,
            position: pos,
            velocity: Vector2::new(0.0, 0.0),
            // new->rect.x = (int) (x - (mass / 2));
            // new->rect.y = (int) (y - (mass / 2));
            // new->rect.w = (int) mass;
            // new->rect.h = (int) mass;
            //
            // new->collision_rect.x = new->position.x + ((new->rect.w / 4) * (new->velocity.x > 0 ? 1 : -1));
            // new->collision_rect.y = new->position.y + ((new->rect.h / 4) * (new->velocity.y > 0 ? 1 : -1));
            // new->collision_rect.w = new->rect.w / 2;
            // new->collision_rect.h = new->rect.h / 2;
            //
            // new->texture = texture;
        }
    }

    fn steer(&mut self, steer_force: Vector2, mut clamp_speed: f64) {
        if (steer_force - self.position).length_sqr() < 1.0 {
            return;
        }

        if self.current_speed < clamp_speed {
            self.current_speed += 0.6;
            clamp_speed = self.current_speed;
        } else if self.current_speed > clamp_speed {
            self.current_speed -= 0.6;
            clamp_speed = self.current_speed;
        }

        self.velocity += ((steer_force % self.max_force) * (1.0 / self.mass)) % clamp_speed;
        self.position += self.velocity;

        // body->rect.x = (int) body->position.x - (body->rect.w / 2);
        // body->rect.y = (int) body->position.y - (body->rect.h / 2);
        //
        // Vector2_t norm_vel = vector_normalize(body->velocity);
        // body->collision_rect.x = body->position.x + ((body->rect.w / 4) * norm_vel.x) - (body->collision_rect.w / 2);
        // body->collision_rect.y = body->position.y + ((body->rect.h / 4) * norm_vel.y) - (body->collision_rect.w / 2);
    }

    pub fn seek(&mut self, target: Vector2) {
        let desired_velocity = (target - self.position) % self.max_force;

        self.steer((desired_velocity - self.velocity), self.peak_speed);
    }

    pub fn arrive(&mut self, target: Vector2) {
        let to_target = target - self.position;
        let distance = to_target.length();
        let clipped_speed = min(self.peak_speed * distance / 100.0, self.peak_speed);
        let desired_velocity = to_target * (clipped_speed / distance);

        self.steer(desired_velocity - self.velocity, self.peak_speed);
    }

    pub fn wander(&mut self) {
        let wander_point = self.velocity + self.velocity.random_in_radius(10);
    }
}