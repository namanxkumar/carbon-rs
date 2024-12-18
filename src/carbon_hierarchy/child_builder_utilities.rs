use super::children::Children;
use super::hierarchy_event::HierarchyEvent;
use super::parent::Parent;
use bevy_ecs::{entity::Entity, prelude::Events, world::World};
use smallvec::SmallVec;

// Do not use `world.send_event_batch` as it prints error message when the Events are not available in the world,
// even though it's a valid use case to execute commands on a world without events. Loading a GLTF file for example
pub fn push_events(world: &mut World, events: impl IntoIterator<Item = HierarchyEvent>) {
    if let Some(mut moved) = world.get_resource_mut::<Events<HierarchyEvent>>() {
        moved.extend(events);
    }
}

/// Adds `child` to `parent`'s [`Children`], without checking if it is already present there.
///
/// This might cause unexpected results when removing duplicate children.
pub fn add_child_unchecked<C: Children, P: Parent>(
    world: &mut World,
    parent: Entity,
    child: Entity,
) {
    let mut parent = world.entity_mut(parent);
    if let Some(mut children) = parent.get_mut::<C>() {
        children.push(child);
    } else {
        parent.insert(C::from_slice(&[child]));
    }
}

/// Sets [`Parent`] of the `child` to `new_parent`. Inserts [`Parent`] if `child` doesn't have one.
pub fn update_parent<C: Children, P: Parent>(
    world: &mut World,
    child: Entity,
    new_parent: Entity,
) -> Option<Entity> {
    let mut child = world.entity_mut(child);
    if let Some(mut parent) = child.get_mut::<P>() {
        let previous = parent.get();
        *parent = P::new(new_parent);
        Some(previous)
    } else {
        child.insert(P::new(new_parent));
        None
    }
}

/// Remove child from the parent's [`Children`] component.
///
/// Removes the [`Children`] component from the parent if it's empty.
pub fn remove_from_children<C: Children, P: Parent>(
    world: &mut World,
    parent: Entity,
    child: Entity,
) {
    let Ok(mut parent) = world.get_entity_mut(parent) else {
        return;
    };
    let Some(mut children) = parent.get_mut::<C>() else {
        return;
    };
    children.retain(|x| *x != child);
    if children.is_empty() {
        parent.remove::<C>();
    }
}

/// Update the [`Parent`] component of the `child`.
/// Removes the `child` from the previous parent's [`Children`].
///
/// Does not update the new parents [`Children`] component.
///
/// Does nothing if `child` was already a child of `parent`.
///
/// Sends [`HierarchyEvent`]'s.
pub fn update_old_parent<C: Children, P: Parent>(world: &mut World, child: Entity, parent: Entity) {
    let previous = update_parent::<C, P>(world, child, parent);
    if let Some(previous_parent) = previous {
        // Do nothing if the child was already parented to this entity.
        if previous_parent == parent {
            return;
        }
        remove_from_children::<C, P>(world, previous_parent, child);

        push_events(
            world,
            [HierarchyEvent::ChildMoved {
                child,
                previous_parent,
                new_parent: parent,
            }],
        );
    } else {
        push_events(world, [HierarchyEvent::ChildAdded { child, parent }]);
    }
}

/// Update the [`Parent`] components of the `children`.
/// Removes the `children` from their previous parent's [`Children`].
///
/// Does not update the new parents [`Children`] component.
///
/// Does nothing for a child if it was already a child of `parent`.
///
/// Sends [`HierarchyEvent`]'s.
pub fn update_old_parents<C: Children, P: Parent>(
    world: &mut World,
    parent: Entity,
    children: &[Entity],
) {
    let mut events: SmallVec<[HierarchyEvent; 8]> = SmallVec::with_capacity(children.len());
    for &child in children {
        if let Some(previous) = update_parent::<C, P>(world, child, parent) {
            // Do nothing if the entity already has the correct parent.
            if parent == previous {
                continue;
            }

            remove_from_children::<C, P>(world, previous, child);
            events.push(HierarchyEvent::ChildMoved {
                child,
                previous_parent: previous,
                new_parent: parent,
            });
        } else {
            events.push(HierarchyEvent::ChildAdded { child, parent });
        }
    }
    push_events(world, events);
}

/// Removes entities in `children` from `parent`'s [`Children`], removing the component if it ends up empty.
/// Also removes [`Parent`] component from `children`.
pub fn remove_children<C: Children, P: Parent>(
    parent: Entity,
    children: &[Entity],
    world: &mut World,
) {
    let mut events: SmallVec<[HierarchyEvent; 8]> = SmallVec::new();
    if let Some(parent_children) = world.get::<C>(parent) {
        for &child in children {
            if parent_children.contains(&child) {
                events.push(HierarchyEvent::ChildRemoved { child, parent });
            }
        }
    } else {
        return;
    }
    for event in &events {
        if let &HierarchyEvent::ChildRemoved { child, .. } = event {
            world.entity_mut(child).remove::<P>();
        }
    }
    push_events(world, events);

    let mut parent = world.entity_mut(parent);
    if let Some(mut parent_children) = parent.get_mut::<C>() {
        parent_children.retain(|parent_child| !children.contains(parent_child));

        if parent_children.is_empty() {
            parent.remove::<C>();
        }
    }
}

/// Removes all children from `parent` by removing its [`Children`] component, as well as removing
/// [`Parent`] component from its children.
pub fn clear_children<C: Children, P: Parent>(parent: Entity, world: &mut World) {
    if let Some(children) = world.entity_mut(parent).take::<C>() {
        for child in children.get() {
            world.entity_mut(child).remove::<P>();
        }
    }
}
