use std::fmt::Debug;

use crate::vector::Vector3;

mod checker;
mod constant;

pub use self::checker::CheckerTexture;
pub use self::constant::ConstantTexture;

pub trait Texture: Debug + Sync + Send + 'static {
    fn value(&self, u: f32, v: f32, point: Vector3) -> Vector3;
}
