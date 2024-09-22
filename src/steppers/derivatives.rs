use bevy::prelude::*;
use std::time::Duration;

use crate::prelude::PitchYawClamped;

pub trait TickDerivative {
    type Derivative;

    fn tick(&self, dt: Duration, derivative: Self::Derivative) -> Self;
}

impl TickDerivative for PitchYawClamped {
    type Derivative = Vec2;

    fn tick(&self, dt: Duration, derivative: Self::Derivative) -> Self {
        PitchYawClamped::new_with_clamps(
            self.y + derivative.x * dt.as_secs_f32(),
            self.p + derivative.y * dt.as_secs_f32(),
            self.clamp_p,
            self.clamp_y,
        )
        .normalize()
    }
}

impl TickDerivative for f32 {
    type Derivative = f32;
    fn tick(&self, dt: Duration, derivative: Self::Derivative) -> Self {
        self + derivative * dt.as_secs_f32()
    }
}

impl TickDerivative for Vec3 {
    type Derivative = Vec3;

    fn tick(&self, dt: Duration, derivative: Self::Derivative) -> Self {
        *self + derivative * dt.as_secs_f32()
    }
}

impl TickDerivative for Vec2 {
    type Derivative = Vec2;

    fn tick(&self, dt: Duration, derivative: Self::Derivative) -> Self {
        *self + derivative * dt.as_secs_f32()
    }
}
