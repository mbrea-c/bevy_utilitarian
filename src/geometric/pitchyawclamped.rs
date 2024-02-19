use bevy::math::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    f32::consts::PI,
    ops::{Add, Mul, Sub},
};

use super::pitchyaw::PitchYaw;

/// Rotation without roll component. Clamps in both axes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PitchYawClamped {
    /// Pitch rotation
    pub p: f32,
    /// Yaw rotation
    pub y: f32,
    pub clamp_p: f32,
    pub clamp_y: f32,
}

impl Default for PitchYawClamped {
    fn default() -> Self {
        Self::new(0., 0.)
    }
}

impl PitchYawClamped {
    pub fn new(u: f32, v: f32) -> Self {
        Self {
            y: u,
            p: v,
            clamp_p: PI / 2. - 0.001,
            clamp_y: PI - 0.001,
        }
    }

    pub fn new_with_clamps(u: f32, v: f32, clamp_p: f32, clamp_y: f32) -> Self {
        Self {
            y: u,
            p: v,
            clamp_p,
            clamp_y,
        }
    }

    pub fn to_unit_vec(&self) -> Vec3 {
        sample_unit_sphere_surface(self.y, self.p)
    }

    pub fn from_vec(dir: Vec3) -> Self {
        let u = (-dir.x).atan2(-dir.z);
        let v = (dir.y / dir.length()).asin();

        Self::new(u, v)
    }

    pub fn length(&self) -> f32 {
        (self.y * self.y + self.p * self.p).sqrt()
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    pub fn clamp_u(&self, min: f32, max: f32) -> Self {
        Self::new(self.y.clamp(min, max), self.p)
    }

    pub fn clamp_v(&self, min: f32, max: f32) -> Self {
        let v = self.p.clamp(min, max);
        Self::new(self.y, v)
    }

    pub fn clamp(&self, min: f32, max: f32) -> Self {
        self.clamp_u(min, max).clamp_v(min, max)
    }

    pub fn normalize(&self) -> Self {
        Self {
            p: self.p.clamp(-self.clamp_p, self.clamp_p),
            y: self.y.clamp(-self.clamp_y, self.clamp_y),
            ..*self
        }
    }

    /// Takes into account the wrapping of yaw
    pub fn sub_pitchyaw(mut self, other: Self) -> Self {
        self.p -= other.p;
        self.y -= other.y;
        self
    }

    pub fn step_toward(&self, target: PitchYawClamped, dangle: f32) -> Self {
        let mut out = PitchYawClamped::default();
        let delta = target.sub_pitchyaw(*self);

        if delta.y.abs() < dangle {
            out.y = target.y;
        } else {
            out.y = self.y + dangle * delta.y.signum();
        }

        if delta.p.abs() < dangle {
            out.p = target.p;
        } else {
            out.p = self.p + dangle * delta.p.signum();
        }

        out.clamp_y = self.clamp_y;
        out.clamp_p = self.clamp_p;

        out.normalize()
    }

    pub fn to_quat(&self) -> Quat {
        Quat::from_rotation_y(self.y) * Quat::from_rotation_x(-self.p)
    }
}

impl From<PitchYaw> for PitchYawClamped {
    fn from(value: PitchYaw) -> Self {
        Self::new(value.y, value.p)
    }
}

impl From<Vec3> for PitchYawClamped {
    fn from(dir: Vec3) -> Self {
        Self::from_vec(dir)
    }
}

impl Sub<PitchYawClamped> for PitchYawClamped {
    type Output = PitchYawClamped;

    fn sub(self, rhs: PitchYawClamped) -> Self::Output {
        self.sub_pitchyaw(rhs)
    }
}

impl Add<PitchYawClamped> for PitchYawClamped {
    type Output = PitchYawClamped;

    fn add(self, rhs: PitchYawClamped) -> Self::Output {
        PitchYawClamped::new_with_clamps(self.y + rhs.y, self.p + rhs.p, self.clamp_p, self.clamp_y)
    }
}

impl Mul<f32> for PitchYawClamped {
    type Output = PitchYawClamped;

    fn mul(self, rhs: f32) -> Self::Output {
        PitchYawClamped::new_with_clamps(self.y * rhs, self.p * rhs, self.clamp_p, self.clamp_y)
    }
}

pub fn sample_unit_sphere_surface(u: f32, v: f32) -> Vec3 {
    let horizontal_y = v.sin();
    let xz_factor = v.cos();
    let horizontal_x = -u.sin() * xz_factor;
    let horizontal_z = -u.cos() * xz_factor;

    Vec3::new(horizontal_x, horizontal_y, horizontal_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    #[test]
    fn dir_to_spherical_to_dir_is_identity() {
        let dir = Vec3::new(223.3452, 5.22, 835.519);
        let spherical = PitchYawClamped::from_vec(dir);
        let dir2 = spherical.to_unit_vec();

        let dist = dir.normalize().distance(dir2);

        let dir = dir.normalize();
        assert!(
            dist < EPSILON,
            "Distance {dir} to {dir2} was {dist}, greater than allowed {EPSILON}"
        );
    }

    #[test]
    fn spherical_to_dir_to_spherical_is_identity() {
        let spherical = PitchYawClamped::new(0., 0.);
        let dir = spherical.to_unit_vec();
        let spherical2 = PitchYawClamped::from_vec(dir);

        let dist = (spherical - spherical2).length();

        assert!(
            dist < EPSILON,
            "Distance {spherical:?} to {spherical2:?} was {dist}, greater than allowed {EPSILON}"
        );
    }

    #[test]
    fn zero_spherical_is_neg_z() {
        let spherical = PitchYawClamped::new(0., 0.);
        let dir = spherical.to_unit_vec();
        let z = Vec3::NEG_Z;

        let dist = dir.distance(z);

        assert!(
            dist < EPSILON,
            "Distance {dir:?} to {z:?} was {dist}, greater than allowed {EPSILON}"
        );
    }

    #[test]
    fn pi_spherical_is_pos_z() {
        let spherical = PitchYawClamped::new(PI, 0.);
        let dir = spherical.to_unit_vec();
        let z = Vec3::Z;

        let dist = dir.distance(z);

        assert!(
            dist < EPSILON,
            "Distance {dir:?} to {z:?} was {dist}, greater than allowed {EPSILON}"
        );
    }

    #[test]
    fn normalize_is_idempotent() {
        let spherical = PitchYawClamped::new(8.21694, 12.032);

        assert_eq!(spherical.normalize(), spherical.normalize().normalize());
        assert_eq!(
            spherical.normalize(),
            spherical.normalize().normalize().normalize()
        );
    }
}
