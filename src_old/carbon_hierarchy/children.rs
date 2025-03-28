use bevy_ecs::{entity::VisitEntitiesMut, prelude::*};
use smallvec::SmallVec;
use std::ops::Deref;

pub trait Children:
    Component + FromWorld + Deref<Target = [Entity]> + VisitEntitiesMut + Sized
// where
//     for<'a> &'a Self: IntoIterator<Item = &'a Entity, IntoIter = slice::Iter<'a, Entity>>,
{
    fn new(children: smallvec::SmallVec<[Entity; 8]>) -> Self;

    fn from_slice(slice: &[Entity]) -> Self {
        Self::new(SmallVec::<[Entity; 8]>::from_slice(slice))
    }

    fn get(self) -> SmallVec<[Entity; 8]>;

    fn get_ref(&self) -> &SmallVec<[Entity; 8]>;

    fn get_mut(&mut self) -> &mut SmallVec<[Entity; 8]>;

    fn swap(&mut self, swap_to_index: usize, swap_from_index: usize);

    fn push(&mut self, entity: Entity);

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Entity) -> bool;

    fn extend(&mut self, other: &[Entity]);

    fn insert_from_slice(&mut self, index: usize, slice: &[Entity]);
}
