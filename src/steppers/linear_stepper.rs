use super::{
    super::geometric::{pitchyaw::PitchYaw, pitchyawclamped::PitchYawClamped},
    core::TickInterpolator,
};
use bevy::math::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LinearStepper<T> {
    pub current: T,
    pub target: T,
    pub speed: f32,
}

// --- Concrete implementations
// -------------------------------------------------------------------------------

impl<T: Clone> LinearStepper<T> {
    pub fn new(value: T, speed: f32) -> Self {
        Self {
            current: value.clone(),
            target: value,
            speed,
        }
    }
}

impl TickInterpolator<f32> for LinearStepper<f32> {
    fn tick(&mut self, dt: Duration) {
        let delta_abs = self.speed * dt.as_secs_f32();
        if (self.target - self.current).abs() > delta_abs {
            self.current += (self.target - self.current).signum() * delta_abs;
        } else {
            self.current = self.target;
        }
    }

    fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    fn get(&self) -> f32 {
        self.current
    }
}

impl TickInterpolator<Vec3> for LinearStepper<Vec3> {
    fn tick(&mut self, dt: Duration) {
        let delta = self.target - self.current;
        let delta_length = delta.length();
        let max_delta = self.speed * dt.as_secs_f32();
        if delta_length < max_delta {
            self.current = self.target;
        } else if delta_length > 0. {
            let delta = delta.normalize() * max_delta;
            self.current += delta;
        }
    }

    fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    fn get(&self) -> Vec3 {
        self.current
    }
}

impl TickInterpolator<Quat> for LinearStepper<Quat> {
    fn tick(&mut self, dt: Duration) {
        let delta_angle = self.current.angle_between(self.target);
        let max_delta_angle = self.speed * dt.as_secs_f32();
        if delta_angle < max_delta_angle {
            self.current = self.target;
        } else if delta_angle > 0. {
            let ratio = max_delta_angle / delta_angle;
            self.current = self.current.slerp(self.target, ratio);
        }
    }

    fn set_target(&mut self, target: Quat) {
        self.target = target;
    }

    fn get(&self) -> Quat {
        self.current
    }
}

impl TickInterpolator<PitchYaw> for LinearStepper<PitchYaw> {
    fn tick(&mut self, dt: Duration) {
        self.current = self
            .current
            .step_toward(self.target, self.speed * dt.as_secs_f32());
    }

    fn set_target(&mut self, target: PitchYaw) {
        self.target = target.normalize();
    }

    fn get(&self) -> PitchYaw {
        self.current
    }
}

impl TickInterpolator<PitchYawClamped> for LinearStepper<PitchYawClamped> {
    fn tick(&mut self, dt: Duration) {
        self.current = self
            .current
            .step_toward(self.target, self.speed * dt.as_secs_f32());
    }

    fn set_target(&mut self, target: PitchYawClamped) {
        self.target = target.normalize();
    }

    fn get(&self) -> PitchYawClamped {
        self.current
    }
}
// -------------------------------------------------------------------------------
