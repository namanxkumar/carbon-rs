use crate::primitives::Transform;
use bevy_ecs::prelude::*;

// RESOURCES

#[derive(Resource)]
pub struct Timestamp(pub f64);

#[derive(Resource, Default)]
pub struct BaseTransform(pub Transform);
