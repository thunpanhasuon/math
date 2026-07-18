#[derive(PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

    pub fn add_vec2(&self, vec2: &mut Vec2) -> Vec2 {
        vec2.x += self.x;
        vec2.y += self.y;

        return Vec2 {
            x: (vec2.x),
            y: (vec2.y)
        }
    }

    pub fn sub_vec2(&self, vec2: &mut Vec2) -> Vec2 {
        vec2.x = self.x - vec2.x;
        vec2.y = self.y - vec2.y;

        return Vec2 {
            x: (vec2.x),
            y: (vec2.y)
        }
    }

    pub fn dot_vec2(&self, vec2: &mut Vec2) -> f32 {
       return (vec2.x * self.x) + (vec2.y * self.y)
    }

    pub fn len_vec2(&self, vec2: &mut Vec2) -> f32 {
        return ((vec2.x).powi(2) + (vec2.y).powi(2)).sqrt()
    }

    pub fn scale_vec2(&self, scale: f32) -> Vec2 {
        return Vec2 { x:(scale * self.x), y:(scale * self.y) }
    }

    pub fn normalize_vec2(&self, vec2: &mut Vec2) -> Vec2 {
        let len = self.len_vec2(vec2);
        if len == 0.0 {
            /* fallback to (0, 0) */
            return Vec2 {x:(0.0), y:(0.0)}
        }

        return Vec2 {
            x:(vec2.x / len),
            y:(vec2.y / len)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vector() {
        let expected = Vec2 {x: 2.0, y: 4.0};
        let vector = Vec2::new(1.0, 2.0);
        let mut pos1 = Vec2 {x: 1.0, y: 2.0};
        let output = vector.add_vec2(&mut pos1);

        assert_eq!(expected, output);
    }
    #[test]
    fn test_sub_vector() {
        let expected = Vec2 {x: 1.0, y: 2.0};
        let vector = Vec2::new(2.0, 4.0);
        let mut pos1 = Vec2 {x: 1.0, y: 2.0};
        let output = vector.sub_vec2(&mut pos1);

        assert_eq!(expected, output);
    }
    #[test]
    fn test_dot_product() {

        let vector = Vec2::new(1.0, 2.0);
        let mut pos1 = Vec2 {x: -2.0, y: 1.0};
        let mut pos2 = Vec2 {x: 2.0, y: 4.0};
        let output_perpendicular = vector.dot_vec2(&mut pos1);
        let output_dot_product = vector.dot_vec2(&mut pos2);

        assert_eq!(0.0, output_perpendicular);
        assert_eq!(10.0, output_dot_product);
    }
    #[test]
    fn test_len() {

        let vector = Vec2::new(3.0, 4.0);
        let mut pos = Vec2 {x: 3.0, y: 4.0};
        let mut add_vec = vector.add_vec2(&mut pos);
        let output = vector.len_vec2(&mut add_vec);

        assert_eq!(10.0, output);
    }
    #[test]
    fn test_scale_vec2() {
        let expected = Vec2 {x: 2.0, y:4.0};
        let vector = Vec2::new(1.0, 2.0);
        let output = vector.scale_vec2(2.0);
        assert_eq!(expected, output);
    }
    #[test]
    fn test_normalize_vec2() {
        let expected = Vec2 {x:1.0, y:0.0};
        let expeted_fallback = Vec2 {x:0.0, y:0.0};

        let vector = Vec2::new(0.0, 0.0);
        let mut pos1 = Vec2 {x: 5.0, y: 0.0};
        let mut pos2 = Vec2 {x: 0.0, y: 0.0};
        let output = vector.normalize_vec2(&mut pos1);
        let output_zero = vector.normalize_vec2(&mut pos2);

        assert_eq!(expected, output);
        assert_eq!(expeted_fallback, output_zero);

    }
}
