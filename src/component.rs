// TODO: Entity
// use crate::{entity::Entity, world::World};

pub trait Component: Send + Sync + 'static {
    // fn register_component_hooks(_hooks: &mut ComponentHooks);
    // fn register_required_components(
    //     _component_id: ComponentId,
    //     _components: &mut Components,
    //     _storages: &mut Storages,
    //     _required_components: &mut RequiredComponents,
    //     _inheritance_depth: u16,
    // );
}

pub struct ComponentId(usize);
// TODO: why usize here?

// impl ComponentId {
//     #[inline]
//     pub fn index(self) -> usize {
//         self.0
//     }
// }

// pub type ComponentHook = fn(World, Entity, ComponentId);
// TODO: The bevy implementation uses a DeferredWorld type that disallows any structural ECS modifications and requires a lifetime parameter (defined using for<'a> fn()).

// pub struct ComponentHooks {
//     pub(crate) on_add: Option<ComponentHook>,
//     pub(crate) on_insert: Option<ComponentHook>,
//     pub(crate) on_replace: Option<ComponentHook>,
//     pub(crate) on_remove: Option<ComponentHook>,
// }

// impl ComponentHooks {
//     pub fn on_add(&mut self, hook: ComponentHook) -> &mut Self {
//         if self.on_add.is_some() {
//             panic!("on_add hook already set");
//         }
//         self.on_add = Some(hook);
//         self
//     }
// }
