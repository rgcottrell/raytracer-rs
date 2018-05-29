use std::f32::consts::PI;

use math::{next_random, random_in_unit_disc};
use ray::Ray;
use vector::Vector3;

#[derive(Debug)]
pub struct Camera {
    origin: Vector3,
    lower_left: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32,
        time0: f32,
        time1: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);
        let origin = look_from;
        let lower_left = origin - focus_distance * (half_width * u + half_height * v + w);
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;
        Camera {
            origin: origin,
            lower_left: lower_left,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
            time0: time0,
            time1: time1,
        }
    }

    pub fn cast_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc();
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = self.time0 + next_random() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
