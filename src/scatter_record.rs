use ray::Ray;
use vector::Vector3;

#[derive(Debug)]
pub struct ScatterRecord {
    ray: Ray,
    attenuation: Vector3,
}

impl ScatterRecord {
    pub fn new(ray: Ray, attenuation: Vector3) -> ScatterRecord {
        ScatterRecord {
            ray: ray,
            attenuation: attenuation,
        }
    }

    #[inline]
    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    #[inline]
    pub fn attenuation(&self) -> Vector3 {
        self.attenuation
    }
}
