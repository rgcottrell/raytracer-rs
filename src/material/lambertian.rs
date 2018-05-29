use hit_record::HitRecord;
use material::Material;
use math::random_in_unit_sphere;
use ray::Ray;
use scatter_record::ScatterRecord;
use texture::Texture;

#[derive(Debug)]
pub struct Lambertian {
    albedo: Box<Texture>,
}

impl Lambertian {
    pub fn new(albedo: impl Texture) -> Lambertian {
        Lambertian {
            albedo: Box::new(albedo),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let ray_out = Ray::new(
            hit.point(),
            hit.normal() + random_in_unit_sphere(),
            ray.time(),
        );
        let scatter = ScatterRecord::new(ray_out, self.albedo.value(0.0, 0.0, hit.point()));
        Some(scatter)
    }
}
