use glam::f32 as glam_primitives;

#[derive(Clone, Default, Debug)]
pub struct Translation(glam_primitives::Vec3A);

impl Translation {
    fn to_glam_vec3(&self) -> glam_primitives::Vec3 {
        glam_primitives::Vec3::from(self.0)
    }

    pub fn zero() -> Self {
        Self(glam_primitives::Vec3A::ZERO)
    }

    pub fn from_vector(vector: &[f32; 3]) -> Self {
        Self(glam_primitives::Vec3A::new(vector[0], vector[1], vector[2]))
    }

    pub fn to_vector(&self) -> [f32; 3] {
        [self.0.x, self.0.y, self.0.z]
    }
}

#[derive(Clone, Default, Debug)]
pub struct Rotation(glam_primitives::Mat3A);

impl Rotation {
    pub fn from_quaternion(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(glam_primitives::Mat3A::from_quat(
            glam_primitives::Quat::from_xyzw(x, y, z, w),
        ))
    }

    fn to_glam_quat(&self) -> glam_primitives::Quat {
        glam_primitives::Quat::from_mat3a(&self.0)
    }

    pub fn to_quaternion(&self) -> [f32; 4] {
        self.to_glam_quat().to_array()
    }

    pub fn identity() -> Self {
        Self(glam_primitives::Mat3A::IDENTITY)
    }

    pub fn to_matrix(&self) -> [[f32; 3]; 3] {
        self.0.to_cols_array_2d()
    }

    pub fn from_matrix(matrix: &[[f32; 3]; 3]) -> Self {
        Self(glam_primitives::Mat3A::from_cols_array_2d(matrix))
    }
}

#[derive(Default)]
pub struct Transform(glam_primitives::Affine3A);

impl Transform {
    pub fn from_translation_and_rotation(translation: Translation, rotation: Rotation) -> Self {
        Self(glam_primitives::Affine3A::from_rotation_translation(
            rotation.to_glam_quat(),
            translation.to_glam_vec3(),
        ))
    }

    pub fn identity() -> Self {
        Self(glam_primitives::Affine3A::IDENTITY)
    }

    pub fn to_matrix(&self) -> [[f32; 4]; 4] {
        glam_primitives::Mat4::from(self.0).to_cols_array_2d()
    }

    pub fn from_matrix(matrix: &[[f32; 4]; 4]) -> Self {
        Self(glam_primitives::Affine3A::from_mat4(
            glam_primitives::Mat4::from_cols_array_2d(matrix),
        ))
    }

    pub fn apply(&self, other: Transform) -> Transform {
        Self(self.0 * other.0)
    }
}

pub struct JointCommand {
    pub position: Option<f32>,
    pub velocity: Option<f32>,
    pub effort: Option<f32>,
}
