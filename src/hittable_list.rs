use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                if temp_rec.t < closest_so_far {
                    closest_so_far = temp_rec.t;
                    closest_hit = Some(temp_rec);
                }
            }
        }
        closest_hit
    }
}
