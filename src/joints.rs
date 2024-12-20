use std::collections::HashMap;

use crate::carbon_components::description::{Link, Pose};

use bevy_ecs::prelude::*;
use bevy_ecs::traversal::Traversal;

#[derive(Resource)]
pub struct PoseTree {
    pub root: Entity,
    pub links: HashMap<Entity, Pose>,
    pub joints: HashMap<Entity, Joint>,
}

impl PoseTree {
    pub fn new(root: Entity) -> Self {
        Self {
            root,
            links: HashMap::new(),
            joints: HashMap::new(),
        }
    }

    pub fn add_link(&mut self, entity: Entity, pose: Pose) {
        self.links.insert(entity, pose);
    }

    pub fn add_joint(&mut self, entity: Entity, joint: ContinuousJoint) {
        self.joints.insert(entity, joint);
    }

    pub fn get_pose(&self, entity: Entity) -> Option<&Pose> {
        self.links.get(&entity)
    }

    pub fn get_joint(&self, entity: Entity) -> Option<&ContinuousJoint> {
        self.joints.get(&entity)
    }

    pub fn add_relationship(&mut self, parent: Entity, child: Entity) {
        if let Some(pose) = self.links.get(&parent) {
            self.links.insert(child, pose.clone());
        } else if let Some(joint) = self.joints.get(&parent) {
            self.joints.insert(child, joint.clone());
        } else {
            panic!("Parent entity does not exist in the PoseTree");
        }
    }

    pub fn get_children(&self, parent: Entity) -> Vec<Entity> {
        let mut children = Vec::new();
        for (&entity, pose) in &self.links {
            if pose.parent == parent {
                children.push(entity);
            }
        }
        for (&entity, joint) in &self.joints {
            if joint.parent == parent {
                children.push(entity);
            }
        }
        children
    }
}

#[derive(Component)]
pub struct ContinuousJoint {
    pub axis_of_rotation: Vector3,
    child: Entity,
}

impl Traversal for &ContinuousJoint {
    fn traverse(item: Self::Item<'_>) -> Option<Entity> {
        Some(item.child)
    }
}

impl Joint for ContinuousJoint {
    fn get_child(&self) -> Entity {
        self.child
    }
}

pub trait Joint: Component // where
//     for<'a> &'a Self: Traversal,
{
    fn get_child(&self) -> Entity;
}

// Joint pose propagation systems
fn joint_propagation<J: Joint>(
    joint_query: Query<(&J, &Pose)>,
    mut link_query: Query<&Pose, With<Link>>,
) {
    // For each joint, update the pose of the child entity
    for (joint, pose) in joint_query.iter() {
        let child_pose = link_query
            .get_mut(joint.get_child())
            .expect("Child entity does not have a pose component");

        // Update the child pose relative to the parent link
        pose.apply(child_pose);
    }
}

// Sample heirarchy
// BaseLink Entity: (BaseLink, Pose(relative_to: World), Geometry)
// Joint Entity: (ContinuousJoint, Pose(relative_to: BaseLink))
// LeftWheel Entity: (Wheel, Pose(relative_to: Joint), Geometry, LeftWheelMarker)
// RightWheel Entity: (Wheel, Pose(relative_to: Joint), Geometry, RightWheelMarker)
//
// BaseLink -> Joint -> LeftWheel
//
//  Fully managed by the hierarchy system,
//  - All poses are relative to their parent link
//  - All joints also have a pose relative to their parent link, and contain child entity reference
//  i.e. when joint is controlled, it will update the pose of the child entity relative to the child's parent entity (which may be the same as the joint's parent entity)
