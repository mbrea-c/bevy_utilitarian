pub mod curves;
pub mod geometric;
pub mod steppers;

pub mod prelude {
    use super::*;
    pub use curves::{
        color_point::ColorPoint,
        constant::ConstantParamCurve,
        curve::{AsParamCurve, Gradient, ParamCurve},
        linear::LinearParamCurve,
        point::Point,
    };
    pub use geometric::{pitchyaw::PitchYaw, pitchyawclamped::PitchYawClamped};
    pub use steppers::{
        core::TickInterpolator,
        linear_stepper::LinearStepper,
        spring_stepper::{critical_damp_coeff, SpringStepper},
    };
}
