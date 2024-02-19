![Crates.io](https://img.shields.io/crates/v/bevy_utilitarian) ![Crates.io](https://img.shields.io/crates/d/bevy_utilitarian)
[![CI](https://github.com/mbrea-c/bevy_utilitarian/actions/workflows/ci.yaml/badge.svg)](https://github.com/mbrea-c/bevy_utilitarian/actions/workflows/ci.yaml)

# Bevy Utilitarian

Contains a varied set of utilities to make bevy programming easier. Currently,
mostly geometric, maths and interpolation utilities.

## What?

Currently, this library offers:

- `curves` module: Parametric curves.
- `steppers` module: _Dynamic_ interpolation helpers: used for when you cannot
  express the change of the value you want to interpolate easily as a parametric
  curve, such as when using a spring-damper system as an interpolator or when
  you want to update the interpolation _target_ before it has been reached.

  You simply set the current value, target value and some interpolator-specific
  parameters (e.g. spring and damping coefficient for spring interpolator, speed
  for linear interpolator) and call `.tick()` every frame to update the current
  value. Using `.get()` you access the current value.

- `geometric` module: Offers `PitchYaw` and `PitchYawClamped` types,
  representing spherical positions (i.e. Euler rotations without the _roll_
  field). `PitchYaw` wraps around the boundary, whereas `PitchYawClamped` is
  clamped to the allowable angle ranges. The latter is useful for use in
  interpolation helpers where you don't want the interpolator to move the
  current value across the `-PI->PI` boundary. Specific use case examples of
  either of these types are:
  aiming/look direction in first/third person character controllers, look
  direction parameter for animation (where you don't want the character's head to
  turn 360 degrees).

- `randomized_values` module: Offers `RandValue` trait and implementations for
  `RandVec3` and `RandF32`. They are self-contained data types with an ergonomic
  API for generating random values based on some parameters.

## Version table

| `bevy_utilitarian` | `bevy` |
| ------------------ | ------ |
| 0.1                | 0.12   |
| 0.2                | 0.12   |
