pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod point;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Lambertian, MaterialType, Metal};
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use utils::{degrees_to_radian, random_boundaries, random_number};
use vec::Vec3;
fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_heigth: u32 = image_width / aspect_ratio.round() as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    // World
    let material_ground = MaterialType::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = MaterialType::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = MaterialType::Metal(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = MaterialType::Metal(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let center1 = Vec3::new(0.0, -100.5, -1.0);
    let center2 = Vec3::new(0.0, 0.0, -1.0);
    let center3 = Vec3::new(0.0, 0.0, -1.0);
    let center4 = Vec3::new(1.0, -100.5, -1.0);

    let sphere_1 = Rc::new(Sphere::new(&center1, 100.0, Rc::new(material_ground)));
    let sphere_2 = Rc::new(Sphere::new(&center2, 0.5, Rc::new(material_center)));
    let sphere_3 = Rc::new(Sphere::new(&center3, 0.5, Rc::new(material_left)));
    let sphere_4 = Rc::new(Sphere::new(&center4, 0.5, Rc::new(material_right)));

    let mut world = HittableList::new();
    world.add(sphere_1);
    world.add(sphere_2);
    world.add(sphere_3);
    world.add(sphere_4);

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_heigth);

    for j in (0..image_heigth).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_number()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_number()) / (image_heigth as f64 - 1.0);

                let dir = cam.get_dir(u, v);
                let r = Ray::new(&cam.origin, &dir);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            pixel_color.write_color(samples_per_pixel);
        }
    }
}

// fn ray_color(r: &Ray) -> Color {
//     let t: f64 = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);

//     if t > 0.0 {
//         let vec = Vec3::from(r.at(t)) - Vec3::new(0.0, 0.0, -1.0);
//         let n = Vec3::unit_vector(&vec);

//         return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
//     }

//     let unit_direction = Vec3::unit_vector(r.direction());
//     let t = (unit_direction.y() + 1.0) / 2.0;
//     Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
// }

fn ray_color<T: Hittable>(r: &Ray, world: &HittableList<T>, depth: i32) -> Color {
    let mut rec = HitRecord {
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let (is_hit, m) = world.hit(r, 0.001, f64::INFINITY, &mut rec);
    if is_hit {
        let scattered: Ray;
        let color = Color::new(0.0, 0.0, 0.0);

        let target = &rec.p + &rec.normal + Vec3::random_unit_vector();

        let dir = &target - &rec.p;
        let ray = Ray::new(&rec.p, &dir);
        return ray_color(&ray, world, depth - 1) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

// fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
//     let oc = r.origin() - center;

//     let a = r.direction().norm().powf(2.0);
//     let half_b = r.direction().dot(&oc);
//     let c = oc.dot(&oc) - radius.powf(2.0);

//     let discrimant = half_b.powf(2.0) - a * c;

//     if discrimant < 0.0 {
//         -1.0
//     } else {
//         (-half_b - discrimant.sqrt()) / a
//     }
// }
