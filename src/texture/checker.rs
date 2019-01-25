use crate::texture::Texture;
use crate::vector::Vector3;

#[derive(Debug)]
pub struct CheckerTexture {
    texture0: Box<Texture>,
    texture1: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(texture0: impl Texture, texture1: impl Texture) -> CheckerTexture {
        CheckerTexture {
            texture0: Box::new(texture0),
            texture1: Box::new(texture1),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vector3) -> Vector3 {
        let sines = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin();
        if sines < 0.0 {
            self.texture0.value(u, v, point)
        } else {
            self.texture1.value(u, v, point)
        }
    }
}
