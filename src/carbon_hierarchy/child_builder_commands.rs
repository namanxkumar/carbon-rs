use bevy_ecs::{
    entity::Entity,
    world::{Command, EntityWorldMut, World},
};
use smallvec::SmallVec;

use super::{
    child_builder::BuildChildren,
    child_builder_utilities::{clear_children, remove_children},
    children::Children,
    parent::Parent,
};

/// Command that adds a child to an entity.
#[derive(Debug)]
pub struct AddChild<C: Children, P: Parent> {
    /// Parent entity to add the child to.
    pub parent: Entity,
    /// Child entity to add.
    pub child: Entity,
    pub _children_marker: std::marker::PhantomData<C>,
    pub _parent_marker: std::marker::PhantomData<P>,
}

impl<C: Children, P: Parent> Command for AddChild<C, P> {
    fn apply(self, world: &mut World) {
        <EntityWorldMut<'_> as BuildChildren<C, P>>::add_child(
            &mut world.entity_mut(self.parent),
            self.child,
        );
    }
}

/// Command that inserts a child at a given index of a parent's children, shifting following children back.
#[derive(Debug)]
pub struct InsertChildren<C: Children, P: Parent> {
    pub parent: Entity,
    pub children: SmallVec<[Entity; 8]>,
    pub index: usize,
    pub _children_marker: std::marker::PhantomData<C>,
    pub _parent_marker: std::marker::PhantomData<P>,
}

impl<C: Children, P: Parent> Command for InsertChildren<C, P> {
    fn apply(self, world: &mut World) {
        <EntityWorldMut<'_> as BuildChildren<C, P>>::insert_children(
            &mut world.entity_mut(self.parent),
            self.index,
            &self.children,
        );
    }
}

/// Command that pushes children to the end of the entity's [`Children`].
#[derive(Debug)]
pub struct AddChildren<C: Children, P: Parent> {
    pub parent: Entity,
    pub children: SmallVec<[Entity; 8]>,
    pub _children_marker: std::marker::PhantomData<C>,
    pub _parent_marker: std::marker::PhantomData<P>,
}

impl<C: Children, P: Parent> Command for AddChildren<C, P> {
    fn apply(self, world: &mut World) {
        <EntityWorldMut<'_> as BuildChildren<C, P>>::add_children(
            &mut world.entity_mut(self.parent),
            &self.children,
        );
    }
}

/// Command that removes children from an entity, and removes these children's parent.
pub struct RemoveChildren<C: Children, P: Parent> {
    pub parent: Entity,
    pub children: SmallVec<[Entity; 8]>,
    pub _children_marker: std::marker::PhantomData<C>,
    pub _parent_marker: std::marker::PhantomData<P>,
}

impl<C: Children, P: Parent> Command for RemoveChildren<C, P> {
    fn apply(self, world: &mut World) {
        remove_children::<C, P>(self.parent, &self.children, world);
    }
}

/// Command that clears all children from an entity and removes [`Parent`] component from those
/// children.
pub struct ClearChildren<C: Children, P: Parent> {
    pub parent: Entity,
    pub _children_marker: std::marker::PhantomData<C>,
    pub _parent_marker: std::marker::PhantomData<P>,
}

impl<C: Children, P: Parent> Command for ClearChildren<C, P> {
    fn apply(self, world: &mut World) {
        clear_children::<C, P>(self.parent, world);
    }
}

/// Command that clear all children from an entity, replacing them with the given children.
pub struct ReplaceChildren<C: Children, P: Parent> {
    pub parent: Entity,
    pub children: SmallVec<[Entity; 8]>,
    pub _children_marker: std::marker::PhantomData<C>,
    pub _parent_marker: std::marker::PhantomData<P>,
}

impl<C: Children, P: Parent> Command for ReplaceChildren<C, P> {
    fn apply(self, world: &mut World) {
        clear_children::<C, P>(self.parent, world);
        <EntityWorldMut<'_> as BuildChildren<C, P>>::add_children(
            &mut world.entity_mut(self.parent),
            &self.children,
        );
    }
}

/// Command that removes the parent of an entity, and removes that entity from the parent's [`Children`].
pub struct RemoveParent<C: Children, P: Parent> {
    /// `Entity` whose parent must be removed.
    pub child: Entity,
    pub _children_marker: std::marker::PhantomData<C>,
    pub _parent_marker: std::marker::PhantomData<P>,
}

impl<C: Children, P: Parent> Command for RemoveParent<C, P> {
    fn apply(self, world: &mut World) {
        <EntityWorldMut<'_> as BuildChildren<C, P>>::remove_parent(
            &mut world.entity_mut(self.child),
        );
    }
}
