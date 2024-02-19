use super::{curve::AsParamCurve, point::Point};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstantParamCurve<P: Point> {
    val: P,
}

impl<P: Point> ConstantParamCurve<P> {
    pub fn new(val: P) -> Self {
        Self { val }
    }
}

impl<P: Point> AsParamCurve<P> for ConstantParamCurve<P> {
    fn get(&self, _: f32) -> P {
        self.val
    }
}
