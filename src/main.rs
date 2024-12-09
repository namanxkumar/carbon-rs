use carbon_rs::{
    component::Component,
    core::components::{Point, Quaternion, Transform, Vector3},
    core::resources::Timestamp,
    core::Core,
    entity::Entity,
};

#[derive(EntityTemplate)]
struct LidarEntity {
    lidar: Lidar,
    point_cloud: PointCloud,
    transform: Transform,
}

#[derive(Component)]
struct Lidar {
    lidar_type: String,
    port: String,
    max_distance: f32,
}

#[derive(Component)]
struct PointCloud {
    points: Vec<Point>,
}

fn read_lidar_data(core: &Core, query: Query<LidarEntity>) {
    let timestamp: Timestamp = core.resources.get::<Timestamp>().unwrap();

    for lidar_entity in query.iter() {
        // Do something with the data, update pointcloud
    }
}

fn main() {
    let lidar = LidarEntity {
        lidar: Lidar {
            lidar_type: "Velodyne".to_string(),
            port: "/dev/ttyUSB0".to_string(),
            max_distance: 100.0,
        },
        transform: Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
        },
        ..Default::default()
    };

    let core = Core::new()
        .enable_recording() // Record a replayable bag style file
        .add_entity(lidar)
        .add_system(read_lidar_data);

    core.run();
}
