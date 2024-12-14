use crate::primitives::Point;

pub trait PortReader {
    type Output;
    fn read_data(&self) -> Option<Self::Output>;
}

pub trait LIDAR: PortReader<Output = Vec<Point>> {}

pub trait MotorController: PortReader<Output = f32> {
    fn send_motor_commands(&self);
}
