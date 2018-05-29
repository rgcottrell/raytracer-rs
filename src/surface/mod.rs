use std::fmt::Debug;

use hit_record::HitRecord;
use ray::Ray;

mod moving_sphere;
mod sphere;

pub use self::moving_sphere::MovingSphere;
pub use self::sphere::Sphere;

pub trait Surface: Debug + Sync + Send + 'static {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

impl Surface for Vec<Box<Surface>> {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut closest = tmax;
        let mut result: Option<HitRecord> = None;
        for surface in self {
            if let Some(hit) = surface.hit(ray, tmin, closest) {
                closest = hit.t();
                result = Some(hit);
            }
        }
        result
    }
}
