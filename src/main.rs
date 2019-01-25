extern crate png;
extern crate rand;

use std::error::Error;
use std::f32;
use std::sync::{Arc, Mutex};
use std::thread;

mod camera;
mod hit_record;
mod image;
mod material;
mod math;
mod ray;
mod scatter_record;
mod surface;
mod texture;
mod vector;

pub use crate::camera::Camera;
pub use crate::hit_record::HitRecord;
pub use crate::image::Image;
pub use crate::material::{Dielectric, Lambertian, Material, Metal};
pub use crate::math::{next_random, random_in_unit_disc, random_in_unit_sphere, reflect, refract, schlick};
pub use crate::ray::Ray;
pub use crate::scatter_record::ScatterRecord;
pub use crate::surface::{MovingSphere, Sphere, Surface};
pub use crate::texture::{CheckerTexture, ConstantTexture, Texture};
pub use crate::vector::Vector3;

const MAX_DEPTH: i32 = 50;

fn build_world() -> impl Surface {
    let mut world: Vec<Box<Surface>> = Vec::new();
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(CheckerTexture::new(
            ConstantTexture::new(Vector3::new(0.2, 0.3, 0.1)),
            ConstantTexture::new(Vector3::new(0.9, 0.9, 0.9)),
        )),
    )));

    let deadzone1 = Vector3::new(-4.0, 0.2, 0.0);
    let deadzone2 = Vector3::new(0.0, 0.2, 0.0);
    let deadzone3 = Vector3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let center = Vector3::new(
                a as f32 + 0.6 * next_random(),
                0.2,
                b as f32 + 0.6 * next_random(),
            );
            if center.distance_to(deadzone1) > 0.9 && center.distance_to(deadzone2) > 0.9
                && center.distance_to(deadzone3) > 0.9
            {
                let chance = next_random();
                if chance < 0.75 {
                    let material = Lambertian::new(ConstantTexture::new(Vector3::new(
                        next_random() * next_random(),
                        next_random() * next_random(),
                        next_random() * next_random(),
                    )));
                    world.push(Box::new(MovingSphere::new(
                        center + Vector3::new(0.0, next_random() * 0.1, 0.0),
                        center + Vector3::new(0.0, next_random() * 0.1, 0.0),
                        0.0,
                        10.0,
                        0.2,
                        material,
                    )));
                } else if chance < 0.9 {
                    let material = Metal::new(
                        ConstantTexture::new(Vector3::new(
                            0.5 * (1.0 + next_random()),
                            0.5 * (1.0 + next_random()),
                            0.5 * next_random(),
                        )),
                        0.1 * next_random(),
                    );
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(ConstantTexture::new(Vector3::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));
    world.push(Box::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(ConstantTexture::new(Vector3::new(0.7, 0.6, 0.5)), 0.0),
    )));

    world
}

fn build_camera(nx: u32, ny: u32) -> Camera {
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect = (nx as f32) / (ny as f32);
    let aperture = 0.1;
    let focus_distance = look_from.distance_to(look_at);
    Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect,
        aperture,
        focus_distance,
        0.0,
        1.0,
    )
}

fn color(world: &Surface, ray: &Ray) -> Vector3 {
    fn color_rec(world: &Surface, ray: &Ray, depth: i32) -> Vector3 {
        if depth >= MAX_DEPTH {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        if let Some(hit) = world.hit(ray, 0.01, f32::MAX) {
            if let Some(scatter) = hit.material().scatter(ray, &hit) {
                return scatter.attenuation() * color_rec(world, scatter.ray(), depth + 1);
            }
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (1.0 + unit_direction.y());
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.6, 1.0)
    }
    color_rec(world, ray, 0)
}

fn render(
    world: impl Surface,
    camera: Camera,
    nx: u32,
    ny: u32,
    nsamples: u32,
    nthreads: u32,
) -> Image {
    let world = Arc::new(world);
    let camera = Arc::new(camera);
    let image = Arc::new(Mutex::new(Image::new(nx, ny)));

    let mut threads = Vec::new();
    for threadid in 0..nthreads {
        let world = Arc::clone(&world);
        let camera = Arc::clone(&camera);
        let image = Arc::clone(&image);
        let handle = thread::spawn(move || {
            for y in 0..ny {
                if y % nthreads != threadid {
                    continue;
                }
                for x in 0..nx {
                    let mut c = Vector3::new(0.0, 0.0, 0.0);
                    for _ in 0..nsamples {
                        let u = (x as f32 + next_random()) / (nx as f32);
                        let v = (y as f32 + next_random()) / (ny as f32);
                        let ray = camera.cast_ray(u, v);
                        c = c + color(world.as_ref(), &ray)
                    }
                    // Average the samples to determine pixel color.
                    c = c / nsamples as f32;
                    // Apply approximate gamma correction to the color.
                    c = c.sqrt();
                    // Update image with the newly calculated pixel, flipping the y
                    // axis to match the expected coordinate system.
                    let mut image = image.lock().unwrap();
                    image.set_pixel(x, ny - y - 1, c);
                }
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    Arc::try_unwrap(image).unwrap().into_inner().unwrap()
}

fn main() -> Result<(), Box<Error>> {
    let nx = 640;
    let ny = 480;
    let nsamples = 128;
    let nthreads = 8;

    let world = build_world();
    let camera = build_camera(nx, ny);
    let image = render(world, camera, nx, ny, nsamples, nthreads);
    image.save("raytracer.png")?;

    Ok(())
}
