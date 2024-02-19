use std::f32::consts::PI;

use super::{curve::AsParamCurve, point::Point};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Clone, Serialize, Deserialize)]
struct LinearSegment<P: Point> {
    pub start: P,
    pub end: P,
}

impl<P: Point> LinearSegment<P> {
    pub fn new(start: P, end: P) -> Self {
        Self { start, end }
    }

    pub fn get_percent(&self, percent: f32) -> P {
        self.start + (self.end - self.start) * percent
    }
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub struct LinearParamCurve<P: Point> {
    /// List of the `t` value at the start of the segment, followed by line segment
    segments: Vec<(f32, LinearSegment<P>)>,
}

impl<P: Point> LinearParamCurve<P> {
    pub fn new(segments: impl IntoIterator<Item = (f32, P, P)>) -> Self {
        let segments: Vec<(f32, LinearSegment<P>)> = segments
            .into_iter()
            .map(|(t, p1, p2)| (t, LinearSegment::new(p1, p2)))
            .collect();

        if segments.len() < 1 {
            panic!("A linear curve requires at least requires at least 1 segment");
        }

        Self { segments }
    }

    pub fn continuous_uniform(points: Vec<P>) -> Self {
        if points.len() < 2 {
            panic!("A linear curve requires at least 2 points");
        }

        let mut segments = vec![];

        for i in 0..points.len() - 1 {
            let t = i as f32 / (points.len() - 1) as f32;
            segments.push((t, LinearSegment::new(points[i], points[i + 1])));
        }

        Self { segments }
    }

    pub fn continuous(points: Vec<(f32, P)>) -> Self {
        if points.len() < 2 {
            panic!("A linear curve requires at least 2 points");
        }

        let mut segments = vec![];

        for i in 0..points.len() - 1 {
            let (t, p1) = points[i];
            let (_, p2) = points[i + 1];
            segments.push((t, LinearSegment::new(p1, p2)));
        }

        Self { segments }
    }

    fn segment_length(&self, segment_idx: usize) -> f32 {
        if segment_idx == self.segments.len() - 1 {
            1. - self.segments[segment_idx].0
        } else {
            self.segments[segment_idx + 1].0 - self.segments[segment_idx].0
        }
    }

    fn segment_start(&self, segment_idx: usize) -> f32 {
        self.segments[segment_idx].0
    }

    fn segment(&self, segment_idx: usize) -> &LinearSegment<P> {
        &self.segments[segment_idx].1
    }
}

impl<P: Point> AsParamCurve<P> for LinearParamCurve<P> {
    fn get(&self, t: f32) -> P {
        let t = t.clamp(0., 1.);

        if self.segments.len() == 0 {
            panic!("LinearCurve has no segments");
        }

        let segment_idx = match self
            .segments
            .binary_search_by(|(t2, _)| t2.partial_cmp(&t).unwrap())
        {
            Ok(i) => i,
            Err(i) => (i - 1).max(0),
        };

        let segment_percent =
            (t - self.segment_start(segment_idx)) / self.segment_length(segment_idx);

        self.segment(segment_idx).get_percent(segment_percent)
    }
}

impl LinearParamCurve<Vec3> {
    /// Creates a linearized version of a circular path over a sphere in polar coordinates
    /// Used to represent the path of the sun over the sky
    pub fn circular_on_sphere(
        normal: Vec3,
        offset_along_normal: f32,
        offset_t: f32,
        n_points: usize,
    ) -> Self {
        let circle_radius = (offset_along_normal * PI / 2.).cos();

        let mut normal_on_hor = normal;
        normal_on_hor.y = 0.;
        normal_on_hor = normal_on_hor.normalize_or_zero();

        let start_t = if normal_on_hor.x == 0. && normal_on_hor.z == 0. {
            0.
        } else {
            normal_on_hor.z.atan2(normal_on_hor.x) / (2. * PI)
        };

        let circle_transform = Transform {
            translation: normal * offset_along_normal,
            rotation: Quat::from_rotation_arc(Vec3::Y, normal),
            ..default()
        };

        let mut t_vals = vec![];
        for i in 0..n_points {
            let t_i = i as f32 / (n_points - 1) as f32;
            let mut t = start_t + offset_t + t_i;
            while t > 1. {
                t -= 1.;
            }
            t_vals.push(t);
        }
        let mut points = vec![];

        for t in t_vals {
            let point = circular_motion(t) * circle_radius;
            let point_3d = circle_transform.transform_point(point);

            points.push(point_3d);
        }

        Self::continuous_uniform(points)
    }
}

fn circular_motion(t: f32) -> Vec3 {
    let t = t * 2. * PI;

    Vec3::new(t.cos(), 0., t.sin())
}
