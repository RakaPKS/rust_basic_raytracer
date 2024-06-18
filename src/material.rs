//material.rs
use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::{Color, Vec3};

use rand::Rng;

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        let scattered = Ray::new(record.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(record.normal).normalized();
        let scattered = Ray::new(
            record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.direction().dot(record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().normalized();
        let cos_theta = ((-1.0) * unit_direction).dot(record.normal).abs().min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect =
            cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>();
        let direction = if will_reflect {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, refraction_ratio)
        };
        let scattered = Ray::new(record.point, direction);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
