use crate::primitives::{Point, Quaternion, Vector3};
use crate::traits::PortReader;
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
