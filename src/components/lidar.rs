use super::common::Port;
use super::description::LinkBundle;
use crate::primitives::Point;
use crate::traits::{PortReader, LIDAR};
use bevy_ecs::prelude::*;

// COMPONENTS for LIDAR data
#[derive(Component, Default)]
pub struct PointCloud {
    pub points: Vec<Point>,
}

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
    pub link: LinkBundle,
    pub port: Port,
    pub point_cloud: PointCloud,
}
