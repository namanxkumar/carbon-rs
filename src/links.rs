type TransformTreeNodeIndex = u32;

pub struct CarbonMetadata {
    pub name: String,
    pub description: String,
    pub timestamp: u64,
}

pub struct CarbonData<T> {
    data: T,
    pub metadata: CarbonMetadata,
}

trait CarbonDataPacket {}

impl CarbonDataPacket for () {}
impl<T> CarbonDataPacket for CarbonData<T> {}

pub struct CarbonTaskConfiguration;

pub trait Task {
    type Input: CarbonDataPacket;
    type Output: CarbonDataPacket;
    fn setup(&self, configuration: Option<&CarbonTaskConfiguration>);
    fn process(&self, input: Self::Input) -> Self::Output;
}

// L2 Traits
pub trait Actuator<I>: Task<Input = CarbonData<I>, Output = ()> {}

pub trait Sensor<O>: Task<Input = (), Output = CarbonData<O>> {}

pub trait Controller<I, O>: Task<Input = CarbonData<I>, Output = CarbonData<O>> {}
