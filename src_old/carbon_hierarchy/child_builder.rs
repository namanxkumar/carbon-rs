use super::child_builder_commands::{
    AddChild, ClearChildren, InsertChildren, RemoveChildren, RemoveParent, ReplaceChildren,
};
use super::child_builder_utilities::{
    add_child_unchecked, clear_children, push_events, remove_children, remove_from_children,
    update_old_parent, update_old_parents,
};
use super::hierarchy_event::HierarchyEvent;
use super::parent::Parent;
use super::{child_builder_commands::AddChildren, children::Children};
use bevy_ecs::system::EntityCommands;
use bevy_ecs::{
    bundle::Bundle,
    entity::Entity,
    system::Commands,
    world::{Command, EntityWorldMut, World},
};
use smallvec::SmallVec;

/// Struct for building children entities and adding them to a parent entity.
pub struct ChildBuilder<'a, C: Children, P: Parent> {
    commands: Commands<'a, 'a>,
    add_children: AddChildren<C, P>,
}

/// Trait for building children entities and adding them to a parent entity. This is used in
/// implementations of [`BuildChildren`] as a bound on the [`Builder`](BuildChildren::Builder)
/// associated type. The closure passed to [`BuildChildren::with_children`] accepts an
/// implementation of `ChildBuild` so that children can be spawned via [`ChildBuild::spawn`].
pub trait ChildBuild<C: Children, P: Parent> {
    /// Spawn output type. Both [`spawn`](Self::spawn) and [`spawn_empty`](Self::spawn_empty) return
    /// an implementation of this type so that children can be operated on via method-chaining.
    /// Implementations of `ChildBuild` reborrow `self` when spawning entities (see
    /// [`Commands::spawn_empty`] and [`World::get_entity_mut`]). Lifetime `'a` corresponds to this
    /// reborrowed self, and `Self` outlives it.
    type SpawnOutput<'a>: BuildChildren<C, P>
    where
        Self: 'a;

    /// Spawns an entity with the given bundle and inserts it into the parent entity's [`Children`].
    /// Also adds [`Parent`] component to the created entity.
    fn spawn(&mut self, bundle: impl Bundle) -> Self::SpawnOutput<'_>;

    /// Spawns an [`Entity`] with no components and inserts it into the parent entity's [`Children`].
    /// Also adds [`Parent`] component to the created entity.
    fn spawn_empty(&mut self) -> Self::SpawnOutput<'_>;

    /// Returns the parent entity.
    fn parent_entity(&self) -> Entity;

    /// Adds a command to be executed, like [`Commands::queue`].
    fn queue_command<Cd: Command>(&mut self, command: Cd) -> &mut Self;
}

impl<C: Children, P: Parent> ChildBuild<C, P> for ChildBuilder<'_, C, P> {
    type SpawnOutput<'a>
        = EntityCommands<'a>
    where
        Self: 'a;

    fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands {
        let e = self.commands.spawn(bundle);
        self.add_children.children.push(e.id());
        e
    }

    fn spawn_empty(&mut self) -> EntityCommands {
        let e = self.commands.spawn_empty();
        self.add_children.children.push(e.id());
        e
    }

    fn parent_entity(&self) -> Entity {
        self.add_children.parent
    }

    fn queue_command<Cd: Command>(&mut self, command: Cd) -> &mut Self {
        self.commands.queue(command);
        self
    }
}

/// Trait for removing, adding and replacing children and parents of an entity.
pub trait BuildChildren<C: Children, P: Parent> {
    /// Child builder type.
    type Builder<'a>: ChildBuild<C, P>;

    /// Takes a closure which builds children for this entity using [`ChildBuild`].
    ///
    /// For convenient spawning of a single child, you can use [`with_child`].
    ///
    /// [`with_child`]: BuildChildren::with_child
    fn with_children(&mut self, f: impl FnOnce(&mut Self::Builder<'_>)) -> &mut Self;

    /// Spawns the passed bundle and adds it to this entity as a child.
    ///
    /// For efficient spawning of multiple children, use [`with_children`].
    ///
    /// [`with_children`]: BuildChildren::with_children
    fn with_child<B: Bundle>(&mut self, bundle: B) -> &mut Self;

    /// Pushes children to the back of the builder's children. For any entities that are
    /// already a child of this one, this method does nothing.
    ///
    /// If the children were previously children of another parent, that parent's [`Children`] component
    /// will have those children removed from its list. Removing all children from a parent causes its
    /// [`Children`] component to be removed from the entity.
    ///
    /// # Panics
    ///
    /// Panics if any of the children are the same as the parent.
    fn add_children(&mut self, children: &[Entity]) -> &mut Self;

    /// Inserts children at the given index.
    ///
    /// If the children were previously children of another parent, that parent's [`Children`] component
    /// will have those children removed from its list. Removing all children from a parent causes its
    /// [`Children`] component to be removed from the entity.
    ///
    /// # Panics
    ///
    /// Panics if any of the children are the same as the parent.
    fn insert_children(&mut self, index: usize, children: &[Entity]) -> &mut Self;

    /// Removes the given children
    ///
    /// Removing all children from a parent causes its [`Children`] component to be removed from the entity.
    fn remove_children(&mut self, children: &[Entity]) -> &mut Self;

    /// Adds a single child.
    ///
    /// If the child was previously the child of another parent, that parent's [`Children`] component
    /// will have the child removed from its list. Removing all children from a parent causes its
    /// [`Children`] component to be removed from the entity.
    ///
    /// # Panics
    ///
    /// Panics if the child is the same as the parent.
    fn add_child(&mut self, child: Entity) -> &mut Self;

    /// Removes all children from this entity. The [`Children`] component will be removed if it exists, otherwise this does nothing.
    fn clear_children(&mut self) -> &mut Self;

    /// Removes all current children from this entity, replacing them with the specified list of entities.
    ///
    /// The removed children will have their [`Parent`] component removed.
    ///
    /// # Panics
    ///
    /// Panics if any of the children are the same as the parent.
    fn replace_children(&mut self, children: &[Entity]) -> &mut Self;

    /// Sets the parent of this entity.
    ///
    /// If this entity already had a parent, the parent's [`Children`] component will have this
    /// child removed from its list. Removing all children from a parent causes its [`Children`]
    /// component to be removed from the entity.
    ///
    /// # Panics
    ///
    /// Panics if the parent is the same as the child.
    fn set_parent(&mut self, parent: Entity) -> &mut Self;

    /// Removes the [`Parent`] of this entity.
    ///
    /// Also removes this entity from its parent's [`Children`] component. Removing all children from a parent causes
    /// its [`Children`] component to be removed from the entity.
    fn remove_parent(&mut self) -> &mut Self;
}

impl<C: Children, P: Parent> BuildChildren<C, P> for EntityCommands<'_> {
    type Builder<'a> = ChildBuilder<'a, C, P>;

    fn with_children(&mut self, spawn_children: impl FnOnce(&mut Self::Builder<'_>)) -> &mut Self {
        let parent = self.id();
        let mut builder = ChildBuilder {
            commands: self.commands(),
            add_children: AddChildren {
                children: SmallVec::default(),
                parent,
                _children_marker: std::marker::PhantomData::<C>,
                _parent_marker: std::marker::PhantomData::<P>,
            },
        };

        spawn_children(&mut builder);
        let children = builder.add_children;
        if children.children.contains(&parent) {
            panic!("Entity cannot be a child of itself.");
        }
        self.commands().queue(children);
        self
    }

    fn with_child<B: Bundle>(&mut self, bundle: B) -> &mut Self {
        let parent = self.id();
        let child = self.commands().spawn(bundle).id();
        self.commands().queue(AddChild {
            parent,
            child,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn add_children(&mut self, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        if children.contains(&parent) {
            panic!("Cannot push entity as a child of itself.");
        }
        self.commands().queue(AddChildren {
            children: SmallVec::from(children),
            parent,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn insert_children(&mut self, index: usize, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        if children.contains(&parent) {
            panic!("Cannot insert entity as a child of itself.");
        }
        self.commands().queue(InsertChildren {
            children: SmallVec::from(children),
            index,
            parent,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn remove_children(&mut self, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        self.commands().queue(RemoveChildren {
            children: SmallVec::from(children),
            parent,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn add_child(&mut self, child: Entity) -> &mut Self {
        let parent = self.id();
        if child == parent {
            panic!("Cannot add entity as a child of itself.");
        }
        self.commands().queue(AddChild {
            child,
            parent,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn clear_children(&mut self) -> &mut Self {
        let parent = self.id();
        self.commands().queue(ClearChildren {
            parent,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn replace_children(&mut self, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        if children.contains(&parent) {
            panic!("Cannot replace entity as a child of itself.");
        }
        self.commands().queue(ReplaceChildren {
            children: SmallVec::from(children),
            parent,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn set_parent(&mut self, parent: Entity) -> &mut Self {
        let child = self.id();
        if child == parent {
            panic!("Cannot set parent to itself");
        }
        self.commands().queue(AddChild {
            child,
            parent,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }

    fn remove_parent(&mut self) -> &mut Self {
        let child = self.id();
        self.commands().queue(RemoveParent {
            child,
            _children_marker: std::marker::PhantomData::<C>,
            _parent_marker: std::marker::PhantomData::<P>,
        });
        self
    }
}

/// Struct for adding children to an entity directly through the [`World`] for use in exclusive systems.
#[derive(Debug)]
pub struct WorldChildBuilder<'w> {
    world: &'w mut World,
    parent: Entity,
}

impl<C: Children, P: Parent> ChildBuild<C, P> for WorldChildBuilder<'_> {
    type SpawnOutput<'a>
        = EntityWorldMut<'a>
    where
        Self: 'a;

    fn spawn(&mut self, bundle: impl Bundle) -> EntityWorldMut {
        let entity = self.world.spawn((bundle, P::new(self.parent))).id();
        add_child_unchecked::<C, P>(self.world, self.parent, entity);
        push_events(
            self.world,
            [HierarchyEvent::ChildAdded {
                child: entity,
                parent: self.parent,
            }],
        );
        self.world.entity_mut(entity)
    }

    fn spawn_empty(&mut self) -> EntityWorldMut {
        let entity = self.world.spawn(P::new(self.parent)).id();
        add_child_unchecked::<C, P>(self.world, self.parent, entity);
        push_events(
            self.world,
            [HierarchyEvent::ChildAdded {
                child: entity,
                parent: self.parent,
            }],
        );
        self.world.entity_mut(entity)
    }

    fn parent_entity(&self) -> Entity {
        self.parent
    }

    fn queue_command<Cd: Command>(&mut self, command: Cd) -> &mut Self {
        command.apply(self.world);
        self
    }
}

impl<C: Children, P: Parent> BuildChildren<C, P> for EntityWorldMut<'_> {
    type Builder<'a> = WorldChildBuilder<'a>;

    fn with_children(&mut self, spawn_children: impl FnOnce(&mut WorldChildBuilder)) -> &mut Self {
        let parent = self.id();
        self.world_scope(|world| {
            spawn_children(&mut WorldChildBuilder { world, parent });
        });
        self
    }

    fn with_child<B: Bundle>(&mut self, bundle: B) -> &mut Self {
        let parent = self.id();
        let child = self.world_scope(|world| world.spawn((bundle, P::new(parent))).id());
        if let Some(mut children_component) = self.get_mut::<C>() {
            children_component.retain(|value| child != *value);
            children_component.push(child);
        } else {
            self.insert(C::from_slice(&[child]));
        }
        self
    }

    fn add_child(&mut self, child: Entity) -> &mut Self {
        let parent = self.id();
        if child == parent {
            panic!("Cannot add entity as a child of itself.");
        }
        self.world_scope(|world| {
            update_old_parent::<C, P>(world, child, parent);
        });
        if let Some(mut children_component) = self.get_mut::<C>() {
            children_component.retain(|value| child != *value);
            children_component.push(child);
        } else {
            self.insert(C::from_slice(&[child]));
        }
        self
    }

    fn add_children(&mut self, children: &[Entity]) -> &mut Self {
        if children.is_empty() {
            return self;
        }

        let parent = self.id();
        if children.contains(&parent) {
            panic!("Cannot push entity as a child of itself.");
        }
        self.world_scope(|world| {
            update_old_parents::<C, P>(world, parent, children);
        });
        if let Some(mut children_component) = self.get_mut::<C>() {
            children_component.retain(|value| !children.contains(value));
            children_component.extend(children);
        } else {
            self.insert(C::from_slice(children));
        }
        self
    }

    fn insert_children(&mut self, index: usize, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        if children.contains(&parent) {
            panic!("Cannot insert entity as a child of itself.");
        }
        self.world_scope(|world| {
            update_old_parents::<C, P>(world, parent, children);
        });
        if let Some(mut children_component) = self.get_mut::<C>() {
            children_component.retain(|value| !children.contains(value));
            children_component.insert_from_slice(index, children);
        } else {
            self.insert(C::from_slice(children));
        }
        self
    }

    fn remove_children(&mut self, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        self.world_scope(|world| {
            remove_children::<C, P>(parent, children, world);
        });
        self
    }

    fn set_parent(&mut self, parent: Entity) -> &mut Self {
        let child = self.id();
        self.world_scope(|world| {
            let mut parent = world.entity_mut(parent);
            <EntityWorldMut<'_> as BuildChildren<C, P>>::add_child(&mut parent, child);
        });
        self
    }

    fn remove_parent(&mut self) -> &mut Self {
        let child = self.id();
        if let Some(parent) = self.take::<P>().map(|p| p.get()) {
            self.world_scope(|world| {
                remove_from_children::<C, P>(world, parent, child);
                push_events(world, [HierarchyEvent::ChildRemoved { child, parent }]);
            });
        }
        self
    }

    fn clear_children(&mut self) -> &mut Self {
        let parent = self.id();
        self.world_scope(|world| {
            clear_children::<C, P>(parent, world);
        });
        self
    }

    fn replace_children(&mut self, children: &[Entity]) -> &mut Self {
        <EntityWorldMut<'_> as BuildChildren<C, P>>::add_children(
            <EntityWorldMut<'_> as BuildChildren<C, P>>::clear_children(self),
            children,
        )
    }
}
