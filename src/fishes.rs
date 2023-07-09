use crate::bodies::Body;
use crate::vectors::Vector2;
use sdl2::render;
use sdl2::render::{Canvas, WindowCanvas};

enum FishBehaviour {
    STILL,
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

struct Flock {
    separation_vec: Vector2,
    separation_w: f64,

    cohesion_vec: Vector2,
    cohesion_w: f64,

    alignment_vec: Vector2,
    alignment_w: f64,
}

impl Flock {
    fn clear(&mut self) {
        let zero_vec = Vector2::new(0.0, 0.0);
        self.separation_vec = zero_vec;
        self.cohesion_vec = zero_vec;
        self.alignment_vec = zero_vec;
    }

    fn add(
        &mut self,
        neighbor_distance: Vector2,
        neighbor_position: Vector2,
        neighbor_velocity: Vector2,
    ) {
        self.separation_vec += neighbor_distance.norm() * (1.0 / neighbor_distance.length_sqr());

        self.cohesion_vec += neighbor_position;

        self.alignment_vec += neighbor_velocity;
    }
}

struct FishDesireVectors {
    wander_vector: Vector2,
    flocking: Flock,
}

pub struct Fish {
    body: Body,
    behaviour: FishBehaviour,
    vision: FishVision,
    desires: FishDesireVectors,
    max_force: f64,
    peak_speed: f64,
    default_speed: f64,
    current_speed: f64,
}

impl Fish {
    pub fn new(pos: Vector2, mass: f64, peak_speed: f64) -> Self {
        Self {
            body: Body::new(100.0, mass, pos),
            behaviour: FishBehaviour::STILL,
            vision: FishVision::new(0.0, 100.0),
            max_force: 0.0,
            peak_speed,
            default_speed: 0.0,
            current_speed: 0.0,
            desires: FishDesireVectors {
                wander_vector: Vector2::new(0.0, 0.0),
                flocking: Flock {
                    separation_vec: Vector2::new(0.0, 0.0),
                    separation_w: 1.0,
                    cohesion_vec: Vector2::new(0.0, 0.0),
                    cohesion_w: 1.0,
                    alignment_vec: Vector2::new(0.0, 0.0),
                    alignment_w: 1.0,
                },
            },
        }
    }

    pub fn is_seeing(&self, target: Body) -> bool {
        let to_target = target.position - self.body.position;

        if self.body.velocity_norm.dot(to_target.norm()) < self.vision.range {
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
        self.body.velocity_norm = self.body.velocity.norm();

        // body->rect.x = (int) body->position.x - (body->rect.w / 2);
        // body->rect.y = (int) body->position.y - (body->rect.h / 2);
        //
        // Vector2_t norm_vel = vector_normalize(body->velocity);
        // body->collision_rect.x = body->position.x + ((body->rect.w / 4) * norm_vel.x) - (body->collision_rect.w / 2);
        // body->collision_rect.y = body->position.y + ((body->rect.h / 4) * norm_vel.y) - (body->collision_rect.w / 2);
    }

    pub fn wander(&mut self) {
        if matches!(self.behaviour, FishBehaviour::WANDERING) {
            self.desires.wander_vector += Vector2::random_in_radius(3.0);
        } else {
            self.behaviour = FishBehaviour::WANDERING;
            self.desires.wander_vector = self.body.velocity.mag(10.0);
        }

        self.steer(
            self.desires.wander_vector.mag(10.0) - self.body.velocity,
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
        let desired_pos = target.body.position + (target.body.velocity_norm * scale);

        self.flee(desired_pos);
    }

    pub fn ponder_flock(&mut self, neighbor: Body) {
        self.desires.flocking.add(
            (self.body.position - neighbor.position),
            neighbor.position,
            neighbor.velocity_norm,
        )
    }

    pub fn compute_flock(&mut self) {
        let separation =
            self.desires.flocking.separation_vec.norm() * self.desires.flocking.separation_w;
        let cohesion = self.desires.flocking.cohesion_vec.norm() * self.desires.flocking.cohesion_w;
        let alignment =
            self.desires.flocking.alignment_vec.norm() * self.desires.flocking.alignment_w;

        let flock_vec = separation + cohesion + alignment;

        self.steer(flock_vec, self.peak_speed); //verificar se é melhor subtrair a velocidade atual.
        self.desires.flocking.clear();
    }

    pub fn draw(&self, canvas: WindowCanvas, texture: &render::Texture) {
        self.body.draw(canvas, texture, false);
    }
}

struct Plant {
    body: Body,
    spreading_radius: f64,
}

impl Plant {
    pub fn new(pos: Vector2, mass: f64) -> Self {
        Self {
            body: Body::new(mass * 1.5, mass, pos),
            spreading_radius: mass / 3.0,
        }
    }

    pub fn spread(&self) -> Self {
        Self::new(
            self.body.position + Vector2::random_in_radius(self.spreading_radius),
            self.body.mass / 5.0,
        )
    }
}