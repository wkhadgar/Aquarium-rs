use crate::bodies::Body;
use crate::vectors::Vector2;

struct Plant {
    body: Body,
}

enum FishBehaviour {
    WANDERING,
    SEEKING,
    ARRIVING,
    FLEEING,
}

struct FishVision {
    range: f64,
    depth: f64,
}

impl FishVision {
    fn new(range: f64, depth: f64) -> Self {
        Self { range, depth }
    }
}

pub struct Fish {
    body: Body,
    behaviour: FishBehaviour,
    vision: FishVision,
    max_force: f64,
    peak_speed: f64,
    default_speed: f64,
    current_speed: f64,
    wander_vector: Vector2,
}

impl Fish {
    pub fn new(pos: Vector2, mass: f64, peak_speed: f64) -> Self {
        Self {
            body: Body::new(100.0, mass, pos),
            behaviour: FishBehaviour::WANDERING,
            vision: FishVision::new(0.0, 100.0),
            max_force: 0.0,
            peak_speed,
            default_speed: 0.0,
            current_speed: 0.0,
            wander_vector: Vector2::new(0.0, 0.0),
        }
    }

    fn is_seeing(&self, target: Body) -> bool {
        let to_target = target.position - self.body.position;

        if self.body.velocity.norm().dot(to_target.norm()) < self.vision.range {
            return false;
        }

        if to_target.length() > self.vision.depth {
            return false;
        }

        true
    }

    fn steer(&mut self, steer_force: Vector2, mut clamp_speed: f64) {
        if (steer_force - self.body.position).length_sqr() < 1.0 {
            return;
        }

        if self.current_speed < clamp_speed {
            self.current_speed += 0.6;
            clamp_speed = self.current_speed;
        } else if self.current_speed > clamp_speed {
            self.current_speed -= 0.6;
            clamp_speed = self.current_speed;
        }

        self.body.velocity +=
            ((steer_force % self.max_force) * (1.0 / self.body.mass)) % clamp_speed;
        self.body.position += self.body.velocity;

        // body->rect.x = (int) body->position.x - (body->rect.w / 2);
        // body->rect.y = (int) body->position.y - (body->rect.h / 2);
        //
        // Vector2_t norm_vel = vector_normalize(body->velocity);
        // body->collision_rect.x = body->position.x + ((body->rect.w / 4) * norm_vel.x) - (body->collision_rect.w / 2);
        // body->collision_rect.y = body->position.y + ((body->rect.h / 4) * norm_vel.y) - (body->collision_rect.w / 2);
    }

    pub fn wander(&mut self) {
        if matches!(self.behaviour, FishBehaviour::WANDERING) {
            self.wander_vector += Vector2::random_in_radius(3.0);
        } else {
            self.behaviour = FishBehaviour::WANDERING;
            self.wander_vector = self.body.velocity.mag(10.0);
        }

        self.steer(
            self.wander_vector.mag(10.0) - self.body.velocity,
            self.default_speed,
        );
    }

    pub fn seek(&mut self, target: Vector2) {
        let desired_velocity = (target - self.body.position) % self.max_force;

        self.behaviour = FishBehaviour::SEEKING;
        self.steer(desired_velocity - self.body.velocity, self.peak_speed);
    }

    pub fn arrive(&mut self, target: Vector2) {
        let to_target = target - self.body.position;
        let clipped_speed = {
            let a = self.peak_speed * to_target.length() / 100.0;
            if a < self.peak_speed {
                a
            } else {
                self.peak_speed
            }
        };
        let desired_velocity = to_target.mag(clipped_speed);

        self.behaviour = FishBehaviour::ARRIVING;
        self.steer(desired_velocity - self.body.velocity, self.peak_speed);
    }

    pub fn flee(&mut self, target: Vector2) {
        let desired_velocity = (self.body.position - target) % self.max_force;

        self.behaviour = FishBehaviour::FLEEING;
        self.steer(desired_velocity - self.body.velocity, self.peak_speed);
    }

    pub fn pursuit(&mut self, target: Self) {
        let scale = (target.body.position - self.body.position).length() * 0.5;
        let desired_pos = target.body.position + target.body.velocity.mag(scale);

        self.seek(desired_pos);
    }

    pub fn evade(&mut self, target: Self) {
        let scale = (target.body.position - self.body.position).length() * 0.5;
        let desired_pos = target.body.position + target.body.velocity.mag(scale);

        self.flee(desired_pos);
    }
}