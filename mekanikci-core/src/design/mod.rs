mod conveyor;
mod motor;

pub use conveyor::ConveyorSpec;
pub use motor::{MotorMount, MotorSpec, NemaFrame};

pub trait DesignSpec {
    fn to_cad_model(&self) -> anyhow::Result<crate::cad::CadAssembly>;
}
