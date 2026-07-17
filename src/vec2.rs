#[derive(PartialEq, Debug)]
pub struct Vec2 {
    pub x: i32, 
    pub y: i32, 
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
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

    pub fn dot_vec2(&self, vec2: &mut Vec2) -> i32 {
       return (vec2.x * self.x) + (vec2.y * self.y)
    }

    pub fn len_vec2(&self, vec2: &mut Vec2) -> i32 {
        return ((vec2.x).pow(2) + (vec2.y).pow(2)).isqrt()
    }

    pub fn scale_vec2(&self, scale: i32) -> Vec2 {
        return Vec2 { x:(scale * self.x), y:(scale * self.y) }
    }

    pub fn normalize_vec2(&self, vec2: &mut Vec2) -> Vec2 {
        let len = self.len_vec2(vec2);
        if len == 0 {
            /* fallback to (0, 0) */
            return Vec2 {x:(0), y:(0)}
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
        let expected = Vec2 {x: 2, y: 4};
        let vector = Vec2::new(1, 2);
        let mut pos1 = Vec2 {x: 1, y: 2};
        let output = vector.add_vec2(&mut pos1);

        assert_eq!(expected, output);
    }
    #[test]
    fn test_sub_vector() {
        let expected = Vec2 {x: 1, y: 2};
        let vector = Vec2::new(2, 4);
        let mut pos1 = Vec2 {x: 1, y: 2};
        let output = vector.sub_vec2(&mut pos1);

        assert_eq!(expected, output);
    }
    #[test]
    fn test_dot_product() {

        let vector = Vec2::new(1, 2);
        let mut pos1 = Vec2 {x: -2, y: 1};
        let mut pos2 = Vec2 {x: 2, y: 4};
        let output_perpendicular = vector.dot_vec2(&mut pos1);
        let output_dot_product = vector.dot_vec2(&mut pos2);

        assert_eq!(0, output_perpendicular);
        assert_eq!(10, output_dot_product);
    }
    #[test]
    fn test_len() {

        let vector = Vec2::new(3, 4);
        let mut pos = Vec2 {x: 3, y: 4};
        let mut add_vec = vector.add_vec2(&mut pos);
        let output = vector.len_vec2(&mut add_vec);

        assert_eq!(10, output);
    }
    #[test]
    fn test_scale_vec2() {
        let expected = Vec2 {x: 2, y:4};
        let vector = Vec2::new(1, 2);
        let output = vector.scale_vec2(2);
        assert_eq!(expected, output);
    } 
    #[test]
    fn test_normalize_vec2() {
        let expected = Vec2 {x:1, y:0};
        let expeted_fallback = Vec2 {x:0, y:0};

        let vector = Vec2::new(0, 0);
        let mut pos1 = Vec2 {x: 5, y: 0};
        let mut pos2 = Vec2 {x: 0, y: 0};
        let output = vector.normalize_vec2(&mut pos1); 
        let output_zero = vector.normalize_vec2(&mut pos2);

        assert_eq!(expected, output);
        assert_eq!(expeted_fallback, output_zero);

    }
}
