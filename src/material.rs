use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &Color,
        scattered: &Ray,
    ) -> (bool, Ray, &Color);
}

pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for MaterialType {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &Color,
        scattered: &Ray,
    ) -> (bool, Ray, &Color) {
        let result = match self {
            MaterialType::Lambertian(l) => l.scatter(r_in, rec, attenuation, scattered),
            MaterialType::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
        };
        result
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &Color,
        scattered: &Ray,
    ) -> (bool, Ray, &Color) {
        let mut scatter_direction = &rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        (true, Ray::new(&rec.p, &scatter_direction), &self.albedo)
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &Color,
        scattered: &Ray,
    ) -> (bool, Ray, &Color) {
        let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);

        (
            (scattered.direction().dot(&rec.normal)) > 0.0,
            Ray::new(&rec.p, &reflected),
            &self.albedo,
        )
    }
}
