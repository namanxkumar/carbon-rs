use super::children::Children;
use super::parent::Parent;
use bevy_ecs::{
    entity::Entity,
    system::EntityCommands,
    world::{Command, EntityWorldMut, World},
};
// use bevy_utils::tracing::debug;

/// Despawns the given entity and all its children recursively
#[derive(Debug)]
pub struct DespawnRecursive<C: Children, P: Parent> {
    /// Target entity
    pub entity: Entity,
    /// Whether or not this command should output a warning if the entity does not exist
    pub warn: bool,
    _children_marker: std::marker::PhantomData<C>,
    _parent_marker: std::marker::PhantomData<P>,
}

/// Despawns the given entity's children recursively
#[derive(Debug)]
pub struct DespawnChildrenRecursive<C: Children, P: Parent> {
    /// Target entity
    pub entity: Entity,
    /// Whether or not this command should output a warning if the entity does not exist
    pub warn: bool,
    _children_marker: std::marker::PhantomData<C>,
    _parent_marker: std::marker::PhantomData<P>,
}

/// Function for despawning an entity and all its children
pub fn despawn_with_children_recursive<C: Children, P: Parent>(
    world: &mut World,
    entity: Entity,
    warn: bool,
) {
    // first, make the entity's own parent forget about it
    if let Some(parent) = world.get::<P>(entity).map(|parent| parent.get()) {
        if let Some(mut children) = world.get_mut::<C>(parent) {
            children.retain(|c| *c != entity);
        }
    }

    // then despawn the entity and all of its children
    despawn_with_children_recursive_inner::<C, P>(world, entity, warn);
}

// Should only be called by `despawn_with_children_recursive` and `try_despawn_with_children_recursive`!
fn despawn_with_children_recursive_inner<C: Children, P: Parent>(
    world: &mut World,
    entity: Entity,
    warn: bool,
) {
    if let Some(mut children) = world.get_mut::<C>(entity) {
        for e in core::mem::take(children.get_mut()) {
            despawn_with_children_recursive_inner::<C, P>(world, e, warn);
        }
    }

    if warn {
        if !world.despawn(entity) {
            // debug!("Failed to despawn entity {:?}", entity);
        }
    } else if !world.try_despawn(entity) {
        // debug!("Failed to despawn entity {:?}", entity);
    }
}

fn despawn_children_recursive<C: Children, P: Parent>(
    world: &mut World,
    entity: Entity,
    warn: bool,
) {
    if let Some(children) = world.entity_mut(entity).take::<C>() {
        for e in children.get() {
            despawn_with_children_recursive_inner::<C, P>(world, e, warn);
        }
    }
}

impl<C: Children, P: Parent> Command for DespawnRecursive<C, P> {
    fn apply(self, world: &mut World) {
        despawn_with_children_recursive::<C, P>(world, self.entity, self.warn);
    }
}

impl<C: Children, P: Parent> Command for DespawnChildrenRecursive<C, P> {
    fn apply(self, world: &mut World) {
        despawn_children_recursive::<C, P>(world, self.entity, self.warn);
    }
}

/// Trait that holds functions for despawning recursively down the transform hierarchy
pub trait DespawnRecursiveExt<C: Children, P: Parent> {
    /// Despawns the provided entity alongside all descendants.
    fn despawn_recursive(self);

    /// Despawns all descendants of the given entity.
    fn despawn_descendants(&mut self) -> &mut Self;

    /// Similar to [`Self::despawn_recursive`] but does not emit warnings
    fn try_despawn_recursive(self);

    /// Similar to [`Self::despawn_descendants`] but does not emit warnings
    fn try_despawn_descendants(&mut self) -> &mut Self;
}

impl<C: Children, P: Parent> DespawnRecursiveExt<C, P> for EntityCommands<'_> {
    /// Despawns the provided entity and its children.
    /// This will emit warnings for any entity that does not exist.
    fn despawn_recursive(mut self) {
        let entity = self.id();
        self.commands().queue(DespawnRecursive {
            entity,
            warn: true,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
    }

    fn despawn_descendants(&mut self) -> &mut Self {
        let entity = self.id();
        self.commands().queue(DespawnChildrenRecursive {
            entity,
            warn: true,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    /// Despawns the provided entity and its children.
    /// This will never emit warnings.
    fn try_despawn_recursive(mut self) {
        let entity = self.id();
        self.commands().queue(DespawnRecursive {
            entity,
            warn: false,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
    }

    fn try_despawn_descendants(&mut self) -> &mut Self {
        let entity = self.id();
        self.commands().queue(DespawnChildrenRecursive {
            entity,
            warn: false,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }
}

fn despawn_recursive_inner<C: Children, P: Parent>(world: EntityWorldMut, warn: bool) {
    let entity = world.id();

    despawn_with_children_recursive::<C, P>(world.into_world_mut(), entity, warn);
}

fn despawn_descendants_inner<'v, 'w, C: Children, P: Parent>(
    world: &'v mut EntityWorldMut<'w>,
    warn: bool,
) -> &'v mut EntityWorldMut<'w> {
    let entity = world.id();

    world.world_scope(|world| {
        despawn_children_recursive::<C, P>(world, entity, warn);
    });
    world
}

impl<'w, C: Children, P: Parent> DespawnRecursiveExt<C, P> for EntityWorldMut<'w> {
    /// Despawns the provided entity and its children.
    /// This will emit warnings for any entity that does not exist.
    fn despawn_recursive(self) {
        despawn_recursive_inner::<C, P>(self, true);
    }

    fn despawn_descendants(&mut self) -> &mut Self {
        despawn_descendants_inner::<C, P>(self, true)
    }

    /// Despawns the provided entity and its children.
    /// This will not emit warnings.
    fn try_despawn_recursive(self) {
        despawn_recursive_inner::<C, P>(self, false);
    }

    fn try_despawn_descendants(&mut self) -> &mut Self {
        despawn_descendants_inner::<C, P>(self, false)
    }
}
