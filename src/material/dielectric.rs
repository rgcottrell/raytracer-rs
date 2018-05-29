use hit_record::HitRecord;
use material::Material;
use math::{next_random, reflect, refract, schlick};
use ray::Ray;
use scatter_record::ScatterRecord;
use vector::Vector3;

#[derive(Debug)]
pub struct Dielectric {
    ri: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Dielectric {
        Dielectric { ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let ray_direction = ray.direction();
        let hit_normal = hit.normal();
        let dot = ray_direction.dot(hit_normal);

        let normal_out: Vector3;
        let cosine: f32;
        let ni_over_nt: f32;
        if dot > 0.0 {
            normal_out = -hit_normal;
            cosine = self.ri * dot / ray_direction.length();
            ni_over_nt = self.ri;
        } else {
            normal_out = hit_normal;
            cosine = -dot / ray_direction.length();
            ni_over_nt = 1.0 / self.ri;
        }

        if let Some(refracted) = refract(ray_direction, normal_out, ni_over_nt) {
            let reflect_probability = schlick(cosine, self.ri);
            if next_random() > reflect_probability {
                let ray_out = Ray::new(hit.point(), refracted, ray.time());
                let scatter = ScatterRecord::new(ray_out, Vector3::new(1.0, 1.0, 1.0));
                return Some(scatter);
            }
        }

        let ray_out = Ray::new(hit.point(), reflect(ray_direction, hit_normal), ray.time());
        let scatter = ScatterRecord::new(ray_out, Vector3::new(1.0, 1.0, 1.0));
        Some(scatter)
    }
}
