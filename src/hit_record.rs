use crate::material::Material;
use crate::vector::Vector3;

#[derive(Debug)]
pub struct HitRecord<'a> {
    t: f32,
    point: Vector3,
    normal: Vector3,
    material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, point: Vector3, normal: Vector3, material: &Material) -> HitRecord {
        HitRecord {
            t: t,
            point: point,
            normal: normal,
            material: material,
        }
    }

    #[inline]
    pub fn t(&self) -> f32 {
        self.t
    }

    #[inline]
    pub fn point(&self) -> Vector3 {
        self.point
    }

    #[inline]
    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    #[inline]
    pub fn material(&self) -> &Material {
        self.material
    }
}
