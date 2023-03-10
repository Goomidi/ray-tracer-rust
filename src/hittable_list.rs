use crate::hittable::{HitRecord, Hittable};
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::rc::Rc;

pub struct HittableList<T> {
    pub objects: Vec<Rc<T>>,
}

impl<T> HittableList<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<T>) {
        self.objects.push(object);
    }
}

impl<'a, T: Hittable> Hittable for HittableList<T> {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> (bool, Rc<MaterialType>) {
        let mut temp_rec = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut material: Rc<MaterialType>;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let (is_hit, m) = object.hit(r, t_min, closest_so_far, &mut temp_rec);
            if is_hit {
                hit_anything = true;
                material = m;
                closest_so_far = temp_rec.t;
            }
        }

        *rec = temp_rec;

        (hit_anything, material)
    }
}
