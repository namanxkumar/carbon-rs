use std::ops::Deref;

use bevy_ecs::{prelude::*, traversal::Traversal};

pub trait Parent: Component + Deref<Target = Entity> + Traversal + FromWorld {
    fn new(parent: Entity) -> Self;

    fn get(&self) -> Entity;
}
