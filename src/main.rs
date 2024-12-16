use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use carbon_rs::components::common::Port;
use carbon_rs::components::description::{
    BaseFrame, Frame, FrameBundle, Geometry, LinkBundle, Pose,
};
use carbon_rs::components::drive::{
    CommandVelocity, EncoderFeedback, Kangaroo, LeftDifferentialDrive, RightDifferentialDrive,
    Wheel, WheelBundle,
};
use carbon_rs::components::lidar::{LIDARBundle, PointCloud, RPLIDAR};
use carbon_rs::primitives::Transform;
use carbon_rs::resources::{BaseTransform, Timestamp};
use carbon_rs::systems::{connect_ports, read_lidar_data};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Main;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Startup;

fn main() {
    let mut world = World::new();

    world.insert_resource(Timestamp(0.0));
    world.insert_resource(BaseTransform(Transform::default()));

    // Spawn Base Link (Frame)
    let base_frame_link = world
        .spawn(FrameBundle {
            marker: BaseFrame,
            pose: Pose {
                transform: Transform::default(),
                reference_frame: None,
            },
            frame: Frame,
        })
        .id();

    // Spawn LIDAR
    world.spawn(LIDARBundle {
        lidar: RPLIDAR,
        link: LinkBundle {
            geometry: Geometry::Cylinder {
                radius: 0.1,
                height: 0.1,
            },
            pose: Pose {
                transform: Transform::default(),
                reference_frame: Some(base_frame_link),
            },
        },
        port: Port("COM3".to_string()),
        point_cloud: PointCloud { points: Vec::new() },
    });

    /*
    // Spawn Kangaroo
    world.spawn((Kangaroo, Port("COM2".to_string())));

    // Spawn Wheels
    world.spawn((
        WheelBundle {
            wheel: Wheel,
            link: LinkBundle {
                geometry: Geometry::Cylinder {
                    radius: 0.1,
                    height: 0.1,
                },
                pose: Pose {
                    transform: Transform::default(),
                    reference_frame: Some(base_frame_link),
                },
            },
            encoder_feedback: EncoderFeedback::default(),
            command_velocity: CommandVelocity(0.0),
        },
        LeftDifferentialDrive,
    ));

    world.spawn((
        WheelBundle {
            wheel: Wheel,
            link: LinkBundle {
                geometry: Geometry::Cylinder {
                    radius: 0.1,
                    height: 0.1,
                },
                pose: Pose {
                    transform: Transform::default(),
                    reference_frame: Some(base_frame_link),
                },
            },
            encoder_feedback: EncoderFeedback::default(),
            command_velocity: CommandVelocity(0.0),
        },
        RightDifferentialDrive,
    ));

    // Print entities and components
    println!("Entities:");
    for entity in world.iter_entities() {
        println!("Entity: {:#?}", entity.id());
        // for component in world.inspect_entity(entity.id()) {
        //     println!("Component: {:#?}", component);
        // }
    }

    */

    let mut startup = Schedule::new(Startup);
    startup.add_systems(connect_ports);

    let mut main_loop = Schedule::new(Main);
    main_loop.add_systems(read_lidar_data::<RPLIDAR>);

    world.add_schedule(startup);
    world.add_schedule(main_loop);

    world.run_schedule(Startup);

    // Main loop
    loop {
        world.run_schedule(Main);
        world.clear_trackers();

        // Simulate a timestep, e.g., sleep for a short duration
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
