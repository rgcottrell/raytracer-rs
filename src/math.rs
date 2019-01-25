use rand::random;
use std::f32::consts::{FRAC_PI_2, PI};

use crate::vector::Vector3;

/// Returns a random number between 0 and 1.
#[inline]
pub fn next_random() -> f32 {
    random::<f32>()
}

/// Returns a random point within a unit sphere.
#[inline]
pub fn random_in_unit_sphere() -> Vector3 {
    let radius = random::<f32>();
    let theta = 2.0 * PI * random::<f32>();
    let phi = PI * random::<f32>() - FRAC_PI_2;
    Vector3::new(
        radius * theta.cos(),
        radius * theta.sin(),
        radius * phi.sin(),
    )
}

/// Returns a random point within a unit disc on the XY plane.
#[inline]
pub fn random_in_unit_disc() -> Vector3 {
    let radius = random::<f32>();
    let theta = 2.0 * PI * random::<f32>();
    Vector3::new(radius * theta.cos(), radius * theta.sin(), 0.0)
}

/// Reflects an incoming vector v hitting a surface with a given normal and
/// returns the reflected vector.
#[inline]
pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
    v - 2.0 * v.dot(normal) * normal
}

/// Refracts an incoming vector v hitting a surface with the given normal and
/// refraction index and returns the refracted vector if refraction was possible.
#[inline]
pub fn refract(v: Vector3, normal: Vector3, ni_over_nt: f32) -> Option<Vector3> {
    let uv = v.normalized();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - dt * normal) - discriminant.sqrt() * normal;
        Some(refracted)
    } else {
        None
    }
}

/// Schlick approximation to varying reflectivity of dialectric by angle.
#[inline]
pub fn schlick(cosine: f32, ri: f32) -> f32 {
    let r0 = (1.0 - ri) / (1.0 + ri);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
