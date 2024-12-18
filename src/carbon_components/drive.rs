use super::description::LinkBundle;
use crate::traits::{MotorController, PortReader};
use bevy_ecs::prelude::*;

#[derive(Component, Default)]
pub struct EncoderFeedback {
    pub position: Option<f32>,
    pub velocity: Option<f32>,
}

#[derive(Component)]
pub struct Wheel;

#[derive(Bundle)]
pub struct WheelBundle {
    pub wheel: Wheel,
    pub link: LinkBundle,
    pub encoder_feedback: EncoderFeedback,
    pub command_velocity: CommandVelocity,
}

#[derive(Component)]
pub struct LeftDifferentialDrive;

#[derive(Component)]
pub struct RightDifferentialDrive;

#[derive(Component)]
pub struct CommandVelocity(pub f32);

#[derive(Component)]
pub struct Kangaroo;

impl PortReader for Kangaroo {
    type Output = f32;

    fn read_data(&self) -> Option<Self::Output> {
        // TODO: Read raw data from Kangaroo
        Some(0.0)
    }
}

impl MotorController for Kangaroo {
    fn send_motor_commands(&self) {}
}
