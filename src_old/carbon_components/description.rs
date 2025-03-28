use std::collections::HashMap;

use crate::primitives::{Rotation, Transform, Translation};
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Link;

#[derive(Component)]
pub struct Dynamic;

#[derive(Component)]
pub struct Frame;

pub struct FrameTreeNode {
    pub reference: Entity,
    pub transform: Transform,
}

#[derive(Resource)]
pub struct FrameTree(HashMap<Entity, FrameTreeNode>);

#[derive(Component)]
pub struct Pose {
    pub transform: Transform,
    pub reference_frame: Entity,
}

impl Pose {
    pub fn from_translation_and_rotation(
        translation: Translation,
        rotation: Rotation,
        reference_frame: Entity,
    ) -> Self {
        Self {
            transform: Transform::from_translation_and_rotation(translation, rotation),
            reference_frame: reference_frame,
        }
    }
}

#[derive(Bundle)]
pub struct LinkBundle {
    pub geometry: Geometry,
    pub pose: Pose,
}

#[derive(Bundle)]
pub struct FrameBundle<T: Component> {
    pub marker: T,
    pub pose: Pose,
    pub frame_label: Frame,
}

// ENUM for now, might replace with individual components implementing geometry trait later
#[derive(Component)]
pub enum Geometry {
    Cylinder {
        radius: f32,
        height: f32,
    },
    Sphere {
        radius: f32,
    },
    Box {
        height: f32,
        width: f32,
        depth: f32,
    },
    Plane {
        width: f32,
        depth: f32,
    },
    Mesh {
        vertices: Vec<(f32, f32, f32)>,
        indices: Vec<u32>,
    },
}

#[derive(Component)]
pub struct BaseFrame;
