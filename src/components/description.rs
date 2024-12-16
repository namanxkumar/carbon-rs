use crate::{
    primitives::{JointCommand, Transform, Vector3},
    traits::Joint,
};
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Dynamic;

#[derive(Component)]
pub struct Frame;

#[derive(Component, Default)]
pub struct Pose {
    pub transform: Transform,
    /// None means World frame
    pub reference_frame: Option<Entity>,
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
    pub frame: Frame,
}

pub struct JointRelation {
    pub parent: Entity,
    pub child: Entity,
}

#[derive(Component)]
pub struct RevoluteJoint {
    relation: JointRelation,
    pub axis: Vector3,
    pub angle: f32,
}

impl Joint for RevoluteJoint {
    fn set_joint(&mut self, parent: Entity, child: Entity) {
        self.relation.parent = parent;
        self.relation.child = child;
    }
    fn get_joint(&self) -> (Entity, Entity) {
        (self.relation.parent, self.relation.child)
    }
    fn update_joint(&mut self, command: JointCommand) {
        // Implement joint update
    }
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
        vertices: Vec<Vector3>,
        indices: Vec<u32>,
    },
}

#[derive(Component)]
pub struct BaseFrame;
