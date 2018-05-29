use texture::Texture;
use vector::Vector3;

#[derive(Debug)]
pub struct ConstantTexture {
    color: Vector3,
}

impl ConstantTexture {
    pub fn new(color: Vector3) -> ConstantTexture {
        ConstantTexture { color: color }
    }

    #[inline]
    pub fn color(&self) -> Vector3 {
        self.color
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _point: Vector3) -> Vector3 {
        self.color
    }
}
