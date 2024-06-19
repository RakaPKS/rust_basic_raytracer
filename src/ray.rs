// ray.rs
use super::vec::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
    inv_direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
            inv_direction: direction.invert(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.direction
    }

    pub fn inv_direction(&self) -> Point3 {
        self.inv_direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
