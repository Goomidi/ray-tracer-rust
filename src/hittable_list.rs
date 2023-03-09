use crate::hittable::{HitRecord, Hittable};
use crate::vec::Vec3;
use std::rc::Rc;

pub struct HittableList<'a, T> {
    pub objects: Vec<Rc<&'a T>>,
}

impl<'a, T: Hittable> HittableList<'a, T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<&'a T>) {
        self.objects.push(object);
    }
}

impl<'a, T: Hittable> Hittable for HittableList<'a, T> {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        *rec = temp_rec;

        hit_anything
    }
}
