use bevy_ecs::prelude::*;
use carbon_rs::components::{
    CommandVelocity, EncoderFeedback, Kangaroo, LIDARBundle, LeftDifferentialDrive, PointCloud,
    Port, RightDifferentialDrive, Transform, Wheel, WheelBundle, RPLIDAR,
};
use carbon_rs::resources::{BaseTransform, Timestamp};
use carbon_rs::systems::read_lidar_data;

fn main() {
    let mut world = World::new();

    // Spawn LIDAR
    world.spawn(LIDARBundle {
        lidar: RPLIDAR,
        transform: Transform {
            ..Default::default()
        },
        port: Port("COM1".to_string()),
        point_cloud: PointCloud { points: Vec::new() },
    });

    // Spawn Kangaroo
    world.spawn(Kangaroo);

    // Spawn Wheels
    world.spawn((
        WheelBundle {
            wheel: Wheel { radius: 0.1 },
            encoder_feedback: EncoderFeedback {
                ..Default::default()
            },
            transform: Transform {
                ..Default::default()
            },
        },
        LeftDifferentialDrive,
        CommandVelocity(0.0),
    ));

    world.spawn((
        WheelBundle {
            wheel: Wheel { radius: 0.1 },
            encoder_feedback: EncoderFeedback {
                ..Default::default()
            },
            transform: Transform {
                ..Default::default()
            },
        },
        RightDifferentialDrive,
        CommandVelocity(0.0),
    ));

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

    schedule.add_systems(read_lidar_data::<RPLIDAR>);

    loop {
        schedule.run(&mut world);
        // Simulate a timestep, e.g., sleep for a short duration
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
