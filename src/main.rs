use std::{
    io::{stderr, Write},
    sync::Arc,
};

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec;

use camera::Camera;
use hit::{Hit, World};
use material::{Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = record.material.scatter(ray, &record) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().normalized();
        let time = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - time) * Color::new(1.0, 1.0, 1.0) + time * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // World
    let mut world = World::new();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    // Camera
    let cam = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
