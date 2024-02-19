use bevy::prelude::*;
use std::f32::consts::PI;

pub trait RandValue {
    type Out;

    fn generate(&self) -> Self::Out;
    fn constant(value: Self::Out) -> Self;
}

#[derive(Debug, Clone, Copy, Reflect, Default)]
pub struct RandF32 {
    pub min: f32,
    pub max: f32,
}

impl RandValue for RandF32 {
    type Out = f32;

    fn generate(&self) -> f32 {
        rand::random::<f32>() * (self.max - self.min) + self.min
    }

    fn constant(value: f32) -> Self {
        Self {
            min: value,
            max: value,
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct RandVec3 {
    pub magnitude: RandF32,
    pub direction: Vec3,
    pub spread: f32,
}

impl RandValue for RandVec3 {
    type Out = Vec3;

    fn generate(&self) -> Vec3 {
        let dir = if self.spread > 0. {
            let spread_angle = rand::random::<f32>() * 2. * PI;
            let spread_radius = rand::random::<f32>() * self.spread;

            let local_dir = Quat::from_rotation_x(spread_angle)
                * Vec3::new(spread_radius.cos(), 0., spread_radius.sin());

            Quat::from_rotation_arc(Vec3::X, self.direction) * local_dir
        } else {
            self.direction.normalize_or_zero()
        };

        dir * self.magnitude.generate()
    }

    fn constant(value: Vec3) -> Self {
        Self {
            direction: value.normalize_or_zero(),
            magnitude: RandF32::constant(value.length()),
            spread: 0.,
        }
    }
}

impl Default for RandVec3 {
    fn default() -> Self {
        Self {
            magnitude: RandF32::default(),
            direction: Vec3::X,
            spread: 0.,
        }
    }
}
