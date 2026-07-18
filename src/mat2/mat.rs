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

    pub fn mul(mat: &mut Mat2, vec: &mut Vec2) -> Vec2 {
            let mut new_vec: Vec<f32> = Vec::new();

            for i in 0..mat.matrix.len() {
                new_vec.push(
                    (mat.matrix[i][0] * vec.x) +
                    (mat.matrix[i][1] * vec.y)
                 );
            }

            Vec2 {
                x:(new_vec[0]), 
                y:(new_vec[1])
            }
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
        let mut vec = Vec2 {x:1.0 , y: 2.0};

        let mut reflect_mat2x2 = Mat2::new([0.0, 0.0], [0.0, 0.0])
            .ref_x();
        let output = Mat2::mul(&mut reflect_mat2x2, &mut vec);
        assert_eq!(expected, output);

        let expected_ref_y = Vec2 { x: -1.0, y: 2.0 };
        let mut ref_y_mat = Mat2::ref_y();
        let output_ref_y = Mat2::mul(&mut ref_y_mat, &mut vec);
        assert_eq!(expected_ref_y, output_ref_y);

        let expected_ref_origin = Vec2 { x: -1.0, y: -2.0 };
        let mut ref_origin_mat = Mat2::ref_origin();
        let output_ref_origin = Mat2::mul(&mut ref_origin_mat, &mut vec);
        assert_eq!(expected_ref_origin, output_ref_origin);

        let expected_swap = Vec2 { x: 2.0, y: 1.0 };
        let mut swap_mat = Mat2::swap();
        let output_swap = Mat2::mul(&mut swap_mat, &mut vec);
        assert_eq!(expected_swap, output_swap);
    }

    #[test]
    fn test_ccw_rotation_apply_rotation() {
        let mut vec = Vec2::new(1.0, 0.0);

        // trig gives floating point results (e.g. cos(90deg) is ~-4e-8,
        // not exactly 0.0), so compare with an epsilon instead of assert_eq!
        let mut rot_90 = Mat2::ccwRotatation(Angle::Degrees(90.0));
        let rotated_90 = Mat2::mul(&mut rot_90, &mut vec);
        assert!((rotated_90.x - 0.0).abs() < 1e-6);
        assert!((rotated_90.y - 1.0).abs() < 1e-6);

        let mut rot_180 = Mat2::ccwRotatation(Angle::Degrees(180.0));
        let rotated_180 = Mat2::mul(&mut rot_180, &mut vec);
        assert!((rotated_180.x - (-1.0)).abs() < 1e-6);
        assert!((rotated_180.y - 0.0).abs() < 1e-6);

        let mut rot_0 = Mat2::ccwRotatation(Angle::Radians(0.0));
        let rotated_0 = Mat2::mul(&mut rot_0, &mut vec);
        assert!((rotated_0.x - 1.0).abs() < 1e-6);
        assert!((rotated_0.y - 0.0).abs() < 1e-6);
    }
}
