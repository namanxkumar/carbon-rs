#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
    pub position: Vector3,
    pub intensity: f32,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn to_rotation_matrix(&self) -> [[f32; 3]; 3] {
        // TODO: unsure if this is correct, its just a quick ai gen version

        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;

        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        [
            [1.0 - (yy + zz), xy + wz, xz - wy],
            [xy - wz, 1.0 - (xx + zz), yz + wx],
            [xz + wy, yz - wx, 1.0 - (xx + yy)],
        ]
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

impl Transform {
    pub fn apply(&self, vector: Vector3) -> Vector3 {
        // TODO: Parallelize this operation

        // Apply rotation
        let rotation_matrix = self.rotation.to_rotation_matrix();
        let x = vector.x * rotation_matrix[0][0]
            + vector.y * rotation_matrix[1][0]
            + vector.z * rotation_matrix[2][0];
        let y = vector.x * rotation_matrix[0][1]
            + vector.y * rotation_matrix[1][1]
            + vector.z * rotation_matrix[2][1];
        let z = vector.x * rotation_matrix[0][2]
            + vector.y * rotation_matrix[1][2]
            + vector.z * rotation_matrix[2][2];

        // Apply translation
        Vector3 {
            x: x + self.translation.x,
            y: y + self.translation.y,
            z: z + self.translation.z,
        }
    }
}

pub struct JointCommand {
    pub position: Option<f32>,
    pub velocity: Option<f32>,
    pub effort: Option<f32>,
}

// impl Add for Vector3 {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//         }
//     }
// }
