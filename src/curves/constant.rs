use super::curve::AsParamCurve;
use bevy::{math::VectorSpace, reflect::Reflect};
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct ConstantParamCurve<P: VectorSpace> {
    val: P,
}

impl<P: VectorSpace> ConstantParamCurve<P> {
    pub fn new(val: P) -> Self {
        Self { val }
    }
}

impl<P: VectorSpace> AsParamCurve<P> for ConstantParamCurve<P> {
    fn get(&self, _: f32) -> P {
        self.val
    }
}
