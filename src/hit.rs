// hit.rs
use std::sync::Arc;

use super::bvh::Aabb;
use super::material::Scatter;
use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Scatter>,
    pub time: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<Aabb>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_record = None;
        let mut closest_so_far = t_max;
        for object in self {
            if let Some(record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = record.time;
                tmp_record = Some(record);
            }
        }
        tmp_record
    }
    fn bounding_box(&self) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut box_first = self[0].bounding_box()?;

        for object in &self[1..] {
            if let Some(box_object) = object.bounding_box() {
                box_first = Aabb::surrounding_box(box_first, box_object).unwrap();
            } else {
                return None;
            }
        }

        Some(box_first)
    }
}
