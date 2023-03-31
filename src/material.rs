use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait LightReflection {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> (bool, Vec3);
}

pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl MaterialType {
    pub fn get_albedo(&self) -> Color {
        let albedo = match self {
            MaterialType::Lambertian(l) => l.albedo,
            MaterialType::Metal(m) => m.albedo,
            _ => Color::new(0.0, 0.0, 0.0),
        };
        albedo
    }

    pub fn get_fuzz(&self) -> f64 {
        let fuzz = match self {
            MaterialType::Metal(m) => m.fuzz,
            _ => 0.0,
        };
        fuzz
    }

    pub fn get_ir(&self) -> f64 {
        let ir = match self {
            MaterialType::Dielectric(d) => d.ir,
            _ => 0.0,
        };
        ir
    }
}
impl LightReflection for MaterialType {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> (bool, Vec3) {
        let result = match self {
            MaterialType::Lambertian(l) => l.scatter(r_in, rec, attenuation),
            MaterialType::Metal(m) => m.scatter(r_in, rec, attenuation),
            MaterialType::Dielectric(d) => d.scatter(r_in, rec, attenuation),
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

impl LightReflection for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> (bool, Vec3) {
        let mut scatter_direction = &rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *attenuation = self.albedo;
        (true, scatter_direction)
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl LightReflection for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> (bool, Vec3) {
        let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);

        let scatter_direction = reflected + Vec3::random_in_unit_sphere() * self.fuzz;
        *attenuation = self.albedo;

        (
            (Ray::new(
                &rec.p,
                &(&reflected + &Vec3::random_in_unit_sphere() * self.fuzz),
            )
            .direction()
            .dot(&rec.normal))
                > 0.0,
            scatter_direction,
        )
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }
}

impl LightReflection for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> (bool, Vec3) {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let scatter_direction = Vec3::refract(
            &Vec3::unit_vector(r_in.direction()),
            &rec.normal,
            refraction_ratio,
        );

        (true, scatter_direction)
    }
}
