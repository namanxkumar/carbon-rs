use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Port(pub String);

impl Port {
    pub fn connect(&self) {}
}
