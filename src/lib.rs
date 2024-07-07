pub mod curves;
pub mod geometric;
pub mod randomized_values;
pub mod steppers;

pub mod prelude {
    use super::*;
    pub use curves::{
        constant::ConstantParamCurve,
        curve::{AsParamCurve, Gradient, ParamCurve},
        linear::LinearParamCurve,
    };
    pub use geometric::{pitchyaw::PitchYaw, pitchyawclamped::PitchYawClamped};
    pub use randomized_values::{RandF32, RandValue, RandVec3};
    pub use steppers::{
        core::TickInterpolator,
        linear_stepper::LinearStepper,
        spring_stepper::{critical_damp_coeff, SpringStepper},
    };
}
