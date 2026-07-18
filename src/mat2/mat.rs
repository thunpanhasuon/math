use crate::vec2::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Degrees(f32),
    Radians(f32),
}
impl Angle {
    pub fn as_radians(&self) -> f32 {
        match self {
            Angle::Degrees(deg) => deg.to_radians(),
            Angle::Radians(rad) => *rad,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Mat2 {
    matrix: [[f32; 2]; 2],
}
impl Mat2 {

    pub fn new(row1: [f32; 2], row2: [f32; 2]) -> Self {
        Self {
            matrix: [row1, row2],
        }
    }
    pub fn ccwRotatation(angle: Angle) -> Self {
        let rad = angle.as_radians();
        let (sin, cos) = (rad.sin(), rad.cos());

        Self {
            matrix: [
                [cos, -sin],
                [sin, cos]
            ]
        }
    }
    pub fn swap() -> Self {
        Self {
            matrix: [
                [0.0, 1.0],
                [1.0, 0.0]
            ]
        }
    }
    pub fn ref_origin() -> Self {
        Self {
            matrix: [
                [-1.0, 0.0],
                [0.0, -1.0]
            ]
        }
    }
    pub fn ref_y() -> Self {
        Self {
            matrix: [
                [-1.0, 0.0],
                [0.0, 1.0]
            ]
        }
    }

    pub fn ref_x(self) -> Self {
        Self {
            matrix: [
                [1.0, 0.0],
                [0.0, -1.0]
            ]
        }
    }

    pub fn scale(&self, vec2: &Vec2) -> Vec2 {
        let sx = self.matrix[0][0];
        let sy = self.matrix[1][1];

        Vec2::new(sx * vec2.x, sy * vec2.y)
    }

    pub fn applyRotation(&self, vec2: &Vec2) -> Vec2 {

        let mut new_vec: Vec<f32> = Vec::new();

        for i in 0..self.matrix.len() {
            new_vec.push(
                (self.matrix[i][0] * vec2.x) +
                (self.matrix[i][1] * vec2.y)
             );
        }

        return Vec2 {x:(new_vec[0]), y:(new_vec[1])}

    }

    pub fn reflection(&self, vec2: &Vec2) -> Vec2 {

        let mut new_vec: Vec<f32> = Vec::new();

        for i in 0..self.matrix.len() {
            new_vec.push(
                (self.matrix[i][0] * vec2.x) +
                (self.matrix[i][1] * vec2.y)
             );
        }

        return Vec2 {x:(new_vec[0]), y:(new_vec[1])}

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_transformation() {
        let expected = Vec2::new(2.0, 4.0);
        let mat = Mat2::new([2.0, 0.0], [0.0, 2.0]);
        let vec = Vec2::new(1.0, 2.0);

        let output = mat.scale(&vec);

        assert_eq!(expected, output);
    }
    #[test]
    fn test_reflextion_tranformation() {
        let expected = Vec2 { x:1.0, y: -2.0};
        let vec = Vec2 {x:1.0 , y: 2.0};
        let reflect_mat2x2 = Mat2::new([0.0, 0.0], [0.0, 0.0])
            .ref_x();
        let output = reflect_mat2x2.reflection(&vec);
        assert_eq!(expected, output);

        let expected_ref_y = Vec2 { x: -1.0, y: 2.0 };
        let output_ref_y = Mat2::ref_y().reflection(&vec);
        assert_eq!(expected_ref_y, output_ref_y);

        let expected_ref_origin = Vec2 { x: -1.0, y: -2.0 };
        let output_ref_origin = Mat2::ref_origin().reflection(&vec);
        assert_eq!(expected_ref_origin, output_ref_origin);

        let expected_swap = Vec2 { x: 2.0, y: 1.0 };
        let output_swap = Mat2::swap().reflection(&vec);
        assert_eq!(expected_swap, output_swap);
    }

    #[test]
    fn test_ccw_rotation_apply_rotation() {
        let vec = Vec2::new(1.0, 0.0);

        // trig gives floating point results (e.g. cos(90deg) is ~-4e-8,
        // not exactly 0.0), so compare with an epsilon instead of assert_eq!
        let rotated_90 = Mat2::ccwRotatation(Angle::Degrees(90.0)).applyRotation(&vec);
        assert!((rotated_90.x - 0.0).abs() < 1e-6);
        assert!((rotated_90.y - 1.0).abs() < 1e-6);

        let rotated_180 = Mat2::ccwRotatation(Angle::Degrees(180.0)).applyRotation(&vec);
        assert!((rotated_180.x - (-1.0)).abs() < 1e-6);
        assert!((rotated_180.y - 0.0).abs() < 1e-6);

        let rotated_0 = Mat2::ccwRotatation(Angle::Radians(0.0)).applyRotation(&vec);
        assert!((rotated_0.x - 1.0).abs() < 1e-6);
        assert!((rotated_0.y - 0.0).abs() < 1e-6);
    }
}
