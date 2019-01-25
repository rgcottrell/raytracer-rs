use crate::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3, time: f32) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            time: time,
        }
    }

    #[inline]
    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    #[inline]
    pub fn time(&self) -> f32 {
        self.time
    }

    #[inline]
    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + t * self.direction
    }
}
