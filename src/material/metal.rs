use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::math::{random_in_unit_sphere, reflect};
use crate::ray::Ray;
use crate::scatter_record::ScatterRecord;
use crate::texture::Texture;

#[derive(Debug)]
pub struct Metal {
    albedo: Box<Texture>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: impl Texture, fuzz: f32) -> Metal {
        Metal {
            albedo: Box::new(albedo),
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(ray.direction().normalized(), hit.normal());
        if reflected.dot(hit.normal()) > 0.0 {
            let ray_out = Ray::new(
                hit.point(),
                reflected + self.fuzz * random_in_unit_sphere(),
                ray.time(),
            );
            let scatter = ScatterRecord::new(ray_out, self.albedo.value(0.0, 0.0, hit.point()));
            Some(scatter)
        } else {
            None
        }
    }
}
