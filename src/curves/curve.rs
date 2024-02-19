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

#[derive(Reflect, Clone, Serialize, Deserialize, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::color::Color;

    #[test]
    fn gradient_alpha_blending_works() {
        let grad = Gradient::linear_uniform(vec![
            ColorPoint::rgba(1., 1., 1., 1.),
            ColorPoint::rgba(1., 1., 1., 0.),
        ]);

        let mid = grad.get(0.5);

        assert!(mid.a() - 0.5 < 0.000001);
    }

    #[test]
    fn gradient_alpha_blending_works_large() {
        let grad = Gradient::linear(vec![
            (0., Color::rgba(300., 100., 1., 1.).into()),
            (0.7, Color::rgba(3., 1., 1., 1.).into()),
            (0.8, Color::rgba(1., 0.3, 0.3, 1.).into()),
            (0.9, Color::rgba(0.3, 0.3, 0.3, 1.).into()),
            (1., Color::rgba(0.1, 0.1, 0.1, 0.).into()),
        ]);

        let col = *grad.get(0.9343);
        assert!(col.a() - (1. - 0.343) < 0.000001);
    }
}
