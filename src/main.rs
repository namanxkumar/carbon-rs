use bevy_ecs::prelude::*;
use carbon_rs::components::{LIDARBundle, PointCloud, Port, Transform, LIDAR};
use carbon_rs::primitives::Point;
use carbon_rs::resources::{BaseTransform, Timestamp};
use carbon_rs::traits::PortReader;

fn read_lidar_data(mut query: Query<(&LIDAR, &Transform, &mut PointCloud)>) {
    // For each LIDAR entity
    for (lidar, transform, mut point_cloud) in query.iter_mut() {
        if let Some(data) = lidar.read_data() {
            let points = data
                .iter()
                .map(|point| Point {
                    position: transform.apply(point.position),
                    intensity: point.intensity,
                })
                .collect();
            point_cloud.points = points;
            println!("Read LIDAR data");
        }
    }
}

fn main() {
    let mut world = World::new();

    world.spawn(LIDARBundle {
        lidar: LIDAR::RPLIDAR,
        transform: Transform {
            ..Default::default()
        },
        port: Port("COM1".to_string()),
        point_cloud: PointCloud { points: Vec::new() },
    });

    world.insert_resource(Timestamp(0.0));

    world.insert_resource(BaseTransform::default());

    // Print entities and components
    println!("Entities:");
    for entity in world.iter_entities() {
        println!("Entity: {:#?}", entity.id());
        // for component in world.inspect_entity(entity.id()) {
        //     println!("Component: {:#?}", component);
        // }
    }

    let mut schedule = Schedule::default();

    schedule.add_systems(read_lidar_data);

    loop {
        schedule.run(&mut world);
        // Simulate a timestep, e.g., sleep for a short duration
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
