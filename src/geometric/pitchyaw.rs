use bevy::math::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    f32::consts::PI,
    ops::{Add, Mul, Sub},
};

use super::pitchyawclamped::PitchYawClamped;

/// Rotation without roll component. Wraps yaw around [-PI, PI] and
/// clamps pitch to [-PI/2, PI/2]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PitchYaw {
    /// Pitch rotation
    pub p: f32,
    /// Yaw rotation
    pub y: f32,
}

impl Default for PitchYaw {
    fn default() -> Self {
        Self::new(0., 0.)
    }
}

impl PitchYaw {
    pub fn new(u: f32, v: f32) -> Self {
        Self { y: u, p: v }
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
            p: self.p.clamp(-PI / 2., PI / 2.),
            y: (self.y + PI).rem_euclid(2. * PI) - PI,
        }
    }

    pub fn flip(&self) -> Self {
        Self::new(
            2. * PI - (PI + self.y) - PI,
            (PI - (PI / 2. + self.p)) - PI / 2.,
        )
    }

    /// Takes into account the wrapping of yaw
    pub fn sub_pitchyaw(self, other: Self) -> Self {
        let pitch = self.p - other.p;

        // None that simply subtracting two yaws will be incorrect,
        // as the resulting delta will never "cross" the y = PI boundary
        let yaw = (self.y - other.y + PI).rem_euclid(2. * PI) - PI;

        Self::new(yaw, pitch)
    }

    pub fn step_toward(&self, target: PitchYaw, dangle: f32) -> Self {
        let mut out = PitchYaw::default();
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

        out.normalize()
    }

    pub fn to_quat(&self) -> Quat {
        Quat::from_rotation_y(self.y) * Quat::from_rotation_x(-self.p)
    }
}

impl From<Vec3> for PitchYaw {
    fn from(dir: Vec3) -> Self {
        Self::from_vec(dir)
    }
}

impl Sub<PitchYaw> for PitchYaw {
    type Output = PitchYaw;

    fn sub(self, rhs: PitchYaw) -> Self::Output {
        self.sub_pitchyaw(rhs)
    }
}

impl Add<PitchYaw> for PitchYaw {
    type Output = PitchYaw;

    fn add(self, rhs: PitchYaw) -> Self::Output {
        PitchYaw::new(self.y + rhs.y, self.p + rhs.p)
    }
}

impl Mul<f32> for PitchYaw {
    type Output = PitchYaw;

    fn mul(self, rhs: f32) -> Self::Output {
        PitchYaw::new(self.y * rhs, self.p * rhs)
    }
}

impl From<PitchYawClamped> for PitchYaw {
    fn from(value: PitchYawClamped) -> Self {
        Self::new(value.y, value.p)
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
        let spherical = PitchYaw::from_vec(dir);
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
        let spherical = PitchYaw::new(0., 0.);
        let dir = spherical.to_unit_vec();
        let spherical2 = PitchYaw::from_vec(dir);

        let dist = (spherical - spherical2).length();

        assert!(
            dist < EPSILON,
            "Distance {spherical:?} to {spherical2:?} was {dist}, greater than allowed {EPSILON}"
        );
    }

    #[test]
    fn zero_spherical_is_neg_z() {
        let spherical = PitchYaw::new(0., 0.);
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
        let x = Vec3::Z;

        let dist = dir.distance(x);

        assert!(
            dist < EPSILON,
            "Distance {dir:?} to {x:?} was {dist}, greater than allowed {EPSILON}"
        );
    }

    #[test]
    fn normalize_is_idempotent() {
        let spherical = PitchYaw::new(8.21694, 12.032);

        assert_eq!(spherical.normalize(), spherical.normalize().normalize());
        assert_eq!(
            spherical.normalize(),
            spherical.normalize().normalize().normalize()
        );
    }

    #[test]
    fn step_toward_near_wrap() {
        let spherical_a = PitchYaw::new(PI - 0.1, 0.);
        let spherical_b = PitchYaw::new(-PI + 0.1, 0.);

        let stepped = spherical_a.step_toward(spherical_b, 0.05);
        let target = PitchYaw::new(PI - 0.05, 0.);

        assert!(
            stepped.distance(&target) < EPSILON,
            "Stepped from {:?} to {:?} by 0.05, ended up in {:?}, expected {:?}",
            spherical_a,
            spherical_b,
            stepped,
            target
        );
    }

    #[test]
    fn step_toward_near_wrap_inverse() {
        let spherical_a = PitchYaw::new(-PI + 0.1, 0.);
        let spherical_b = PitchYaw::new(PI - 0.1, 0.);

        let stepped = spherical_a.step_toward(spherical_b, 0.05);
        let target = PitchYaw::new(-PI + 0.05, 0.);
        let dist = stepped.distance(&target);

        assert!(
            stepped.distance(&target) < EPSILON,
            "Distance {stepped:?} to {target:?} was {dist}, greater than allowed {EPSILON}",
        );
    }

    #[test]
    fn step_toward_near_overshoot() {
        let spherical_a = PitchYaw::new(PI - 0.1, 0.);
        let spherical_b = PitchYaw::new(-PI + 0.1, 0.);

        let stepped = spherical_a.step_toward(spherical_b, 0.5);
        let target = spherical_b;

        assert!(stepped.distance(&target) < EPSILON);
    }

    #[test]
    fn wrapped_sub_works_as_expected() {
        let spherical_a = PitchYaw::new(PI - 0.1, 0.);
        let spherical_b = PitchYaw::new(-PI + 0.1, 0.);

        let delta = spherical_b - spherical_a;
        let applied = (spherical_a + delta).normalize();

        assert!(spherical_b.distance(&applied) < EPSILON);
        assert!(delta.length() - 0.2 < EPSILON);
    }
}
