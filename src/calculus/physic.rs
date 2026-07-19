use crate::vec2::Vec2;

pub struct Physic {
    position: Vec2,
    velocity: Vec2,
}

const GRAVITY: f32 = 9.8;

impl Physic {
    pub fn new(inital_pos: Vec2, inital_velocity: Vec2) -> Self {
        Self {
            position: Vec2 {x:(inital_pos.x), y:(inital_pos.y)},
            velocity: Vec2 {x:(inital_velocity.x), y:(inital_velocity.y)},
        }
    }
    pub fn apply_arc(&self, dt: f32) -> f32 {
        return self.velocity.y * dt + 0.5 * GRAVITY * (dt.powi(2));
    }
    pub fn apply_gravity(&mut self, dt: f32) -> Vec2 {
        // gravity only pulls on the vertical (y) velocity
        self.velocity.y += GRAVITY * dt;

        Vec2 {
            x: (self.velocity.x),
            y: (self.velocity.y)
        }
    }
}

// Axis-aligned bounding box: a rectangle described by its min (bottom-left)
// and max (top-right) corners. Used for cheap, simple collision checks.
#[derive(PartialEq, Debug)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    // Two AABBs overlap unless one is entirely to the side of the other
    // on either axis.
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x &&
        self.max.x >= other.min.x &&
        self.min.y <= other.max.y &&
        self.max.y >= other.min.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_gravity() {
        let mut physic = Physic::new(Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0));
        let output = physic.apply_gravity(1.0);

        assert_eq!(Vec2 { x: 5.0, y: 9.8 }, output);
    }

    #[test]
    fn test_aabb_intersects() {
        let a = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let overlapping = Aabb::new(Vec2::new(1.0, 1.0), Vec2::new(3.0, 3.0));
        let separate = Aabb::new(Vec2::new(5.0, 5.0), Vec2::new(6.0, 6.0));

        assert!(a.intersects(&overlapping));
        assert!(!a.intersects(&separate));
    }
}
