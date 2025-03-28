use std::time::Duration;

use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
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

#[derive(Component)]
pub struct WorldFrame;

fn setup(mut commands: Commands) {
    let world_frame = commands.spawn((WorldFrame,)).id();
    let base_frame_link = commands
        .spawn(FrameBundle {
            marker: BaseFrame,
            pose: Pose {
                transform: Transform::default(),
                reference_frame: world_frame,
            },
            frame_label: Frame,
        })
        .id();

    commands.spawn(LIDARBundle {
        lidar: RPLIDAR,
        link: LinkBundle {
            geometry: Geometry::Cylinder {
                radius: 0.1,
                height: 0.1,
            },
            pose: Pose {
                transform: Transform::default(),
                reference_frame: base_frame_link,
            },
        },
        port: Port("COM3".to_string()),
        point_cloud: PointCloud { points: Vec::new() },
    });
    commands.insert_resource(Timestamp(0.0));
    commands.insert_resource(BaseTransform(Transform::default()));
}

fn main() {
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

    App::new()
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1)))
        .add_systems(PreStartup, setup)
        .add_systems(Startup, connect_ports)
        .add_systems(Update, read_lidar_data::<RPLIDAR>)
        .run();
}
