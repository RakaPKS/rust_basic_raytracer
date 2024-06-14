use super::hit::{Hit, HitRecord};
use super::material::Scatter;
use super::ray::Ray;
use super::vec::Point3;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length().powi(2);
        let b = oc.dot(ray.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut record = HitRecord {
            time: root,
            point: ray.at(root),
            material: self.material.clone(),
            normal: (ray.at(root) - self.center) / self.radius,
            front_face: false,
        };

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }
}
