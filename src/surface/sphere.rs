use hit_record::HitRecord;
use material::Material;
use ray::Ray;
use surface::Surface;
use vector::Vector3;

#[derive(Debug)]
pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: impl Material + 'static) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: Box::new(material),
        }
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_norm();
        let b = oc.dot(ray.direction());
        let c = oc.squared_norm() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            if t1 >= tmin && t1 < tmax {
                let point = ray.point_at_parameter(t1);
                let normal = (point - self.center).normalized();
                let hit = HitRecord::new(t1, point, normal, self.material.as_ref());
                return Some(hit);
            }
            let t2 = (-b + discriminant.sqrt()) / a;
            if t2 >= tmin && t2 < tmax {
                let point = ray.point_at_parameter(t2);
                let normal = (point - self.center).normalized();
                let hit = HitRecord::new(t2, point, normal, self.material.as_ref());
                return Some(hit);
            }
        }
        None
    }
}
