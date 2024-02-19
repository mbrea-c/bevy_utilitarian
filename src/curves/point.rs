use bevy::math::prelude::*;
use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

pub trait Point:
    Mul<f32, Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Add<f32, Output = Self>
    + Sum
    + Default
    + std::fmt::Debug
    + Clone
    + PartialEq
    + Copy
{
}

impl Point for f32 {}
impl Point for Vec2 {}
impl Point for Vec3 {}
