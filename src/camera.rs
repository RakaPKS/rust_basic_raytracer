use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    llc: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPOINT_HEIGHT: f64 = 2.0;
        const VIEWPOINT_WIDTH: f64 = ASPECT_RATIO * VIEWPOINT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPOINT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPOINT_HEIGHT, 0.0);
        Camera {
            origin,
            horizontal,
            vertical,
            llc: origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.llc + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
