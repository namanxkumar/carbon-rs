use crate::primitives::{Point, Quaternion, Vector3};
use crate::traits::{MotorController, PortReader, LIDAR};
use bevy_ecs::prelude::*;

// COMPONENTS for LIDAR data

/// Transform with respect to the base frame
#[derive(Component, Default)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

impl Transform {
    pub fn apply(&self, vector: Vector3) -> Vector3 {
        // TODO: Parallelize this operation

        // Apply rotation
        let rotation_matrix = self.rotation.to_rotation_matrix();
        let x = vector.x * rotation_matrix[0][0]
            + vector.y * rotation_matrix[1][0]
            + vector.z * rotation_matrix[2][0];
        let y = vector.x * rotation_matrix[0][1]
            + vector.y * rotation_matrix[1][1]
            + vector.z * rotation_matrix[2][1];
        let z = vector.x * rotation_matrix[0][2]
            + vector.y * rotation_matrix[1][2]
            + vector.z * rotation_matrix[2][2];

        // Apply translation
        Vector3 {
            x: x + self.translation.x,
            y: y + self.translation.y,
            z: z + self.translation.z,
        }
    }
}

#[derive(Component, Default)]
pub struct PointCloud {
    pub points: Vec<Point>,
}

#[derive(Component)]
pub struct Port(pub String);

#[derive(Component)]
pub struct RPLIDAR;

impl PortReader for RPLIDAR {
    type Output = Vec<Point>;

    fn read_data(&self) -> Option<Self::Output> {
        // Read raw data from RPLIDAR
        Some(Vec::new())
    }
}

impl LIDAR for RPLIDAR {}

#[derive(Bundle)]
pub struct LIDARBundle<T: LIDAR + Component> {
    pub lidar: T,
    pub transform: Transform,
    pub port: Port,
    pub point_cloud: PointCloud,
}

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

#[derive(Component)]
pub enum Motor {
    Dynamixel,
}

#[derive(Component)]
pub struct EncoderFeedback {
    pub left_position: Option<f32>,
    pub right_position: Option<f32>,
    pub left_velocity: Option<f32>,
    pub right_velocity: Option<f32>,
}

#[derive(Component)]
pub struct DifferentialDriveCommand {
    pub left_velocity: f32,
    pub right_velocity: f32,
}

#[derive(Bundle)]
pub struct DifferentialDriveBundle<T: MotorController + Component> {
    pub motor_controller: T,
    pub motor: Motor,
    pub encoder_feedback: EncoderFeedback,
    pub differential_drive_command: DifferentialDriveCommand,
}

//TODO!> HOW TO IMPLEMENT TRANSFORMS FOR EACH WHEEL and connect to the feedback
//TODO!> HOW TO IMPLEMENT LINKS BETWEEN TRANSFORMS
