use crate::primitives::{Quaternion, Vector3};
use bevy_ecs::prelude::*;

// RESOURCES

#[derive(Resource)]
pub struct Timestamp(pub f64);

#[derive(Resource, Default)]
pub struct BaseTransform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}
