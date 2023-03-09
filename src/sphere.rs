use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere<'a> {
    pub center: &'a Vec3,
    pub radius: f64,
}

impl<'a> Sphere<'a> {
    pub fn new(center: &'a Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;

        let a = r.direction().norm().powf(2.0);
        let half_b = r.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius.powf(2.0);

        let discrimant = half_b.powf(2.0) - a * c;

        if discrimant < 0.0 {
            return false;
        }
        let sqrtd = discrimant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            };
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (&rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
