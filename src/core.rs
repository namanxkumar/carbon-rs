use crate::{component::ComponentId, entity::Entity};

pub struct Core {
    id: u32,
    pub(crate) entities: Vec<Entity>,
    pub(crate) components: Vec<ComponentId>,
}
