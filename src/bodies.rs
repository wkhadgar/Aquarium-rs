use crate::vectors::Vector2;

pub struct Body {
    pub health: f64,
    pub mass: f64,
    pub position: Vector2,
    pub velocity: Vector2,
    pub velocity_norm: Vector2,
    //rect;
    //collision_rect,
    //texture;
}

impl Body {
    pub fn new(health: f64, mass: f64, position: Vector2) -> Self {
        let vel = Vector2::random_in_radius(1.0);
        Self {
            health,
            mass,
            position,
            velocity: vel,
            velocity_norm: vel.norm(),
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

    fn rescale(&mut self) {
        // body->rect.h = 5 * sqrt(body->mass);
        // body->rect.w = 5 * sqrt(body->mass);
        //
        // body->collision_rect.w = body->rect.w / 2;
        // body->collision_rect.h = body->rect.h / 2;
    }

    pub fn grow(&mut self, mass_gained: f64) {
        self.mass += mass_gained;
        self.rescale();
    }

    pub fn shrink(&mut self, mass_loss: f64) {
        self.mass -= mass_loss;
        self.rescale();
    }
}