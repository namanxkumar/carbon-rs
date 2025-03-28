use crate::primitives::{JointCommand, Point};
use bevy_ecs::prelude::*;

pub trait PortReader {
    type Output;
    fn read_data(&self) -> Option<Self::Output>;
}

pub trait LIDAR: PortReader<Output = Vec<Point>> {}

pub trait MotorController: PortReader<Output = f32> {
    fn send_motor_commands(&self);
}

pub trait Joint {
    fn get_joint(&self) -> (Entity, Entity);
    fn set_joint(&mut self, parent: Entity, child: Entity);
    fn update_joint(&mut self, command: JointCommand);
}
