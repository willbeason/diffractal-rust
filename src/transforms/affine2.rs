use crate::transforms::transform::Transform;

pub struct Affine2 {
    pub a: [f64; 4],
    pub offset: [f64; 2],
}

// scale_rotate_offset constructs a 2D affine transform that scales, rotates, and then offsets its inputs.
// The angle is in radians.
pub fn scale_rotate_offset(scale: [f64; 2], angle: f64, offset: [f64; 2]) -> Affine2 {
    let cos = angle.cos();
    let sin = angle.sin();
    Affine2 {
        a: [
            scale[0] * cos,
            scale[0] * sin,
            -scale[1] * sin,
            scale[1] * cos,
        ],
        offset,
    }
}

impl Transform<[f64; 2], [f64; 2]> for Affine2 {
    fn transform(&self, z: [f64; 2]) -> [f64; 2] {
        [
            self.a[0] * z[0] + self.a[1] * z[1] + self.offset[0],
            self.a[2] * z[0] + self.a[3] * z[1] + self.offset[1],
        ]
    }
}
