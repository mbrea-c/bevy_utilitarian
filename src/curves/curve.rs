use super::{
    color_point::ColorPoint, constant::ConstantParamCurve, linear::LinearParamCurve, point::Point,
};
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

pub trait AsParamCurve<P: Point> {
    /// Get a point on the curve at the given `t` parameter value
    ///
    /// `t` is a value between 0.0 and 1.0.
    fn get(&self, t: f32) -> P;
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub enum ParamCurve<P: Point> {
    Linear(LinearParamCurve<P>),
    Constant(ConstantParamCurve<P>),
}

impl<P: Point> ParamCurve<P> {
    pub fn linear_uniform(points: Vec<P>) -> Self {
        Self::Linear(LinearParamCurve::continuous_uniform(points))
    }

    pub fn linear(points: Vec<(f32, P)>) -> Self {
        Self::Linear(LinearParamCurve::continuous(points))
    }

    pub fn constant(val: P) -> Self {
        Self::Constant(ConstantParamCurve::new(val))
    }
}

impl<P: Point> AsParamCurve<P> for ParamCurve<P> {
    fn get(&self, t: f32) -> P {
        match self {
            ParamCurve::Linear(c) => c.get(t),
            ParamCurve::Constant(c) => c.get(t),
        }
    }
}

pub type Gradient = ParamCurve<ColorPoint>;
