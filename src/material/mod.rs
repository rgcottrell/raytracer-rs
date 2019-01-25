use std::fmt::Debug;

use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::scatter_record::ScatterRecord;

mod dielectric;
mod lambertian;
mod metal;

pub use self::dielectric::Dielectric;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

pub trait Material: Debug + Sync + Send + 'static {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}
