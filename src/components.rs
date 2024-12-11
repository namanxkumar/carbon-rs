use crate::primitives::{Point, Quaternion, Vector3};
use crate::traits::PortReader;
use bevy_ecs::prelude::*;

// Components for LIDAR data

#[derive(Component, Default)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
}

#[derive(Component)]
pub struct PointCloud {
    pub points: Vec<Point>,
}

#[derive(Component)]
pub struct Port(pub String);

#[derive(Component)]
pub enum LIDAR {
    RPLIDAR,
    VLP16,
}

impl PortReader for LIDAR {
    type Output = Vec<Point>;

    fn read_data(&self) -> Option<Self::Output> {
        match self {
            LIDAR::RPLIDAR => {
                // Read raw data from RPLIDAR
                Some(Vec::new())
            }
            LIDAR::VLP16 => {
                // Read raw data from VLP16
                Some(Vec::new())
            }
        }
    }
}

#[derive(Bundle)]
pub struct LIDARBundle {
    pub lidar: LIDAR,
    pub transform: Transform,
    pub port: Port,
    pub point_cloud: PointCloud,
}
