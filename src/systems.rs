use crate::components::common::Port;
use crate::components::description::Pose;
use crate::components::drive::{
    CommandVelocity, EncoderFeedback, LeftDifferentialDrive, RightDifferentialDrive,
};
use crate::components::lidar::PointCloud;
use crate::primitives::Point;
use crate::traits::{MotorController, LIDAR};
use bevy_ecs::prelude::*;

pub fn connect_ports(query: Query<&Port>) {
    for port in query.iter() {
        println!("Connecting to port: {}", port.0);
        port.connect();
    }
}

pub fn read_lidar_data<T: LIDAR + Component>(mut query: Query<(&T, &Pose, &mut PointCloud)>) {
    // For each LIDAR entity
    for (lidar, pose, mut point_cloud) in query.iter_mut() {
        if let Some(data) = lidar.read_data() {
            let points = data
                .iter()
                .map(|point| Point {
                    position: pose.transform.apply(point.position),
                    intensity: point.intensity,
                })
                .collect();
            point_cloud.points = points;
            println!("Read LIDAR data");
        }
    }
}

pub fn read_wheel_encoder_data_via_controller<T: MotorController + Component>(
    mut left_wheel_query: Query<(&T, &mut EncoderFeedback), With<LeftDifferentialDrive>>,
    mut right_wheel_query: Query<(&T, &mut EncoderFeedback), With<RightDifferentialDrive>>,
) {
    todo!("Read wheel encoder data");
}

pub fn send_motor_commands_via_controller<T: MotorController + Component>(
    left_wheel_query: Query<(&T, &CommandVelocity), With<LeftDifferentialDrive>>,
    right_wheel_query: Query<(&T, &CommandVelocity), With<RightDifferentialDrive>>,
) {
    todo!("Send motor commands");
}
