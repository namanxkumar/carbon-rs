pub trait PortReader {
    type Output;
    fn read_data(&self) -> Option<Self::Output>;
}
