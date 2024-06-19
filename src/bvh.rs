use super::hit::{Hit, HitRecord};
use super::ray::Ray;
use super::vec::{Point3, Vec3};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hit>,
    right: Arc<dyn Hit>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(mut objects: Vec<Arc<dyn Hit>>) -> BvhNode {
        let axis = Self::find_best_split_axis(&objects);

        let comparator = move |a: &Arc<dyn Hit>, b: &Arc<dyn Hit>| -> Ordering {
            let box_a = a.bounding_box().unwrap();
            let box_b = b.bounding_box().unwrap();

            if box_a.min()[axis] < box_b.min()[axis] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        };

        let object_span = objects.len();

        let (left, right): (Arc<dyn Hit>, Arc<dyn Hit>) = match object_span {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                objects.sort_by(comparator);
                let (left_objects, right_objects) =
                    Self::split_objects(&objects, axis);
                (
                    Arc::new(BvhNode::new(left_objects)) as Arc<dyn Hit>,
                    Arc::new(BvhNode::new(right_objects)) as Arc<dyn Hit>,
                )
            }
        };

        let box_left = left.bounding_box();
        let box_right = right.bounding_box();

        // Calculate bounding box for current node
        let bounding_box = match (box_left, box_right) {
            (Some(bl), Some(br)) => Aabb::surrounding_box(bl, br),
            _ => None,
        };

        BvhNode {
            left,
            right,
            bounding_box: bounding_box.unwrap(),
        }
    }

    fn find_best_split_axis(objects: &[Arc<dyn Hit>]) -> usize {
        let mut best_axis = 0;
        let mut best_cost = f64::INFINITY;

        for axis in 0..3 {
            let comparator = move |a: &Arc<dyn Hit>, b: &Arc<dyn Hit>| -> Ordering {
                let box_a = a.bounding_box().unwrap();
                let box_b = b.bounding_box().unwrap();

                if box_a.min()[axis] < box_b.min()[axis] {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            };

            let mut sorted_objects = objects.to_vec();
            sorted_objects.sort_by(comparator);

            for i in 1..sorted_objects.len() {
                let (left_objects, right_objects) = sorted_objects.split_at(i);
                let left_box = Aabb::surrounding_box(
                    left_objects[0].bounding_box().unwrap(),
                    left_objects[left_objects.len() - 1].bounding_box().unwrap(),
                );
                let right_box = Aabb::surrounding_box(
                    right_objects[0].bounding_box().unwrap(),
                    right_objects[right_objects.len() - 1].bounding_box().unwrap(),
                );
                let cost = Self::calculate_sah_cost(
                    left_box.unwrap(),
                    right_box.unwrap(),
                    left_objects.len(),
                    right_objects.len(),
                );

                if cost < best_cost {
                    best_axis = axis;
                    best_cost = cost;
                }
            }
        }

        best_axis
    }

    fn calculate_sah_cost(
        left_box: Aabb,
        right_box: Aabb,
        left_count: usize,
        right_count: usize,
    ) -> f64 {
        let left_surface_area = left_box.surface_area();
        let right_surface_area = right_box.surface_area();
        let total_surface_area = left_surface_area + right_surface_area;
        let cost = ((left_count as f64) * left_surface_area
            + (right_count as f64) * right_surface_area)
            / total_surface_area;
        cost
    }

    fn split_objects(
        objects: &[Arc<dyn Hit>],
        _axis: usize,
    ) -> (Vec<Arc<dyn Hit>>, Vec<Arc<dyn Hit>>) {
        let mid = objects.len() / 2;
        let left_objects = objects[..mid].to_vec();
        let right_objects = objects[mid..].to_vec();
        (left_objects, right_objects)
    }
}

impl Hit for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = if hit_left.is_none() {
            self.right.hit(ray, t_min, t_max)
        } else {
            self.right.hit(ray, t_min, hit_left.as_ref().unwrap().time)
        };

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}

#[derive(Clone, Copy)]
pub struct Aabb {
    min: Point3,
    max: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Aabb { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        // Early termination if the ray's origin is inside the AABB
        if ray.origin().x() >= self.min.x() && ray.origin().x() <= self.max.x() &&
           ray.origin().y() >= self.min.y() && ray.origin().y() <= self.max.y() &&
           ray.origin().z() >= self.min.z() && ray.origin().z() <= self.max.z() {
            return true;
        }
        let mut t_min = t_min;
        let mut t_max = t_max;
    
        for a in 0..3 {
            let t0 = (self.min()[a] - ray.origin()[a]) * ray.inv_direction()[a];
            let t1 = (self.max()[a] - ray.origin()[a]) * ray.inv_direction()[a];
    
            let t_near = t0.min(t1);
            let t_far = t0.max(t1);
    
            t_min = t_min.max(t_near);
            t_max = t_max.min(t_far);
    
            if t_max <= t_min {
                return false;
            }
        }
    
        true
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Option<Aabb> {
        let small = Vec3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );

        let big = Vec3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        Some(Aabb::new(small, big))
    }

    fn surface_area(&self) -> f64 {
        let x = (self.max.x() - self.min.x()) * 2.0;
        let y = (self.max.y() - self.min.y()) * 2.0;
        let z = (self.max.z() - self.min.z()) * 2.0;
        x * y + x * z + y * z
    }
}