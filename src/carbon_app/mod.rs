use bevy_ecs::{
    prelude::*,
    schedule::{InternedScheduleLabel, ScheduleLabel},
    system::SystemId,
};
use std::fmt::Debug;

pub struct SubApp {
    world: World,
    pub update_schedule: Option<InternedScheduleLabel>,
}

impl Debug for SubApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubApp")
    }
}

impl Default for SubApp {
    fn default() -> Self {
        let mut world = World::new();
        world.init_resource::<Schedules>();
        Self {
            world,
            update_schedule: None,
        }
    }
}

impl SubApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn mut_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn run_default_schedule(&mut self) {
        if let Some(label) = self.update_schedule {
            self.world.run_schedule(label);
        }
    }

    pub fn update(&mut self) {
        self.run_default_schedule();
        self.world.clear_trackers();
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(schedule, systems);

        self
    }

    pub fn register_system<I, O, M>(
        &mut self,
        system: impl IntoSystem<I, O, M> + 'static,
    ) -> SystemId<I, O>
    where
        I: SystemInput + 'static,
        O: 'static,
    {
        self.world.register_system(system)
    }
}
