use hit_record::HitRecord;
use material::Material;
use ray::Ray;
use surface::Surface;
use vector::Vector3;

#[derive(Debug)]
pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Box<Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vector3,
        center1: Vector3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: impl Material + 'static,
    ) -> MovingSphere {
        MovingSphere {
            center0: center0,
            center1: center1,
            time0: time0,
            time1: time1,
            radius: radius,
            material: Box::new(material),
        }
    }

    fn center(&self, time: f32) -> Vector3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Surface for MovingSphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let center = self.center(ray.time());
        let oc = ray.origin() - center;
        let a = ray.direction().squared_norm();
        let b = oc.dot(ray.direction());
        let c = oc.squared_norm() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            if t1 >= tmin && t1 < tmax {
                let point = ray.point_at_parameter(t1);
                let normal = (point - center).normalized();
                let hit = HitRecord::new(t1, point, normal, self.material.as_ref());
                return Some(hit);
            }
            let t2 = (-b + discriminant.sqrt()) / a;
            if t2 >= tmin && t2 < tmax {
                let point = ray.point_at_parameter(t2);
                let normal = (point - center).normalized();
                let hit = HitRecord::new(t2, point, normal, self.material.as_ref());
                return Some(hit);
            }
        }
        None
    }
}
