use super::{
    super::geometric::pitchyawclamped::PitchYawClamped, core::TickInterpolator,
    derivatives::TickDerivative,
};
use bevy::math::{Vec2, Vec3};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SpringStepper<T, D = T>
where
    D: Serialize + DeserializeOwned,
    T: TickDerivative<Derivative = D>,
{
    pub current: T,
    pub target: T,
    pub velocity: <T as TickDerivative>::Derivative,
    pub spring: f32,
    pub damping: f32,
}

pub const SPRING_MASS: f32 = 1.;

impl<T, D> SpringStepper<T, D>
where
    T: TickDerivative<Derivative = D> + Clone + Default,
    D: Serialize + DeserializeOwned + Default,
{
    pub fn new(value: T, spring: f32, damping: f32) -> Self {
        Self {
            current: value.clone(),
            target: value,
            velocity: D::default(),
            spring,
            damping,
        }
    }

    pub fn new_with_initial_velocity(value: T, velocity: D, spring: f32, damping: f32) -> Self {
        Self {
            current: value.clone(),
            target: value,
            velocity,
            spring,
            damping,
        }
    }
}

// --- Concrete implementations
// -------------------------------------------------------------------------------

impl TickInterpolator<PitchYawClamped> for SpringStepper<PitchYawClamped, Vec2> {
    fn tick(&mut self, dt: Duration) {
        let damping_force = self.velocity * (-self.damping);
        let spring_force = (self.target - self.current) * self.spring;
        let spring_force = Vec2::new(spring_force.y, spring_force.p);
        self.velocity += (damping_force + spring_force) * (dt.as_secs_f32() / SPRING_MASS);
        self.current = self.current.tick(dt, self.velocity)
    }

    fn set_target(&mut self, target: PitchYawClamped) {
        self.target = target;
    }

    fn get(&self) -> PitchYawClamped {
        self.current
    }
}

impl TickInterpolator<f32> for SpringStepper<f32, f32> {
    fn tick(&mut self, dt: Duration) {
        let damping_force = self.velocity * (-self.damping);
        let spring_force = (self.target - self.current) * self.spring;
        self.velocity += (damping_force + spring_force) * (dt.as_secs_f32() / SPRING_MASS);
        self.current = self.current.tick(dt, self.velocity);
    }

    fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    fn get(&self) -> f32 {
        self.current
    }
}

impl TickInterpolator<Vec3> for SpringStepper<Vec3> {
    fn tick(&mut self, dt: Duration) {
        let damping_force = self.velocity * (-self.damping);
        let spring_force = (self.target - self.current) * self.spring;
        self.velocity += (damping_force + spring_force) * (dt.as_secs_f32() / SPRING_MASS);
        self.current = self.current.tick(dt, self.velocity);
    }

    fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    fn get(&self) -> Vec3 {
        self.current
    }
}

impl TickInterpolator<Vec2> for SpringStepper<Vec2> {
    fn tick(&mut self, dt: Duration) {
        let damping_force = self.velocity * (-self.damping);
        let spring_force = (self.target - self.current) * self.spring;
        self.velocity += (damping_force + spring_force) * (dt.as_secs_f32() / SPRING_MASS);
        self.current = self.current.tick(dt, self.velocity);
    }

    fn set_target(&mut self, target: Vec2) {
        self.target = target;
    }

    fn get(&self) -> Vec2 {
        self.current
    }
}

// -------------------------------------------------------------------------------

// --- Utility functions
// -------------------------------------------------------------------------------

/// Calculate the critical damping coefficient for a spring-damper system with the given spring
/// constant and a mass specified by the SPRING_MASS constant.
pub fn critical_damp_coeff(spring_constant: f32) -> f32 {
    2. * (spring_constant * SPRING_MASS).sqrt()
}

// -------------------------------------------------------------------------------
