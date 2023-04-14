use crate::{utils::degrees_to_radian, vec::Vec3};
pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    pub origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl<'a> Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radian(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_heigth: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_heigth;

        let focal_length: f64 = 1.0;

        let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, viewport_heigth, 0.0);

        Self {
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_heigth,
            viewport_width: viewport_width,
            focal_length: 1.0,

            origin: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_heigth, 0.0),
            lower_left_corner: &origin
                - &horizontal / 2.0
                - &vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_dir(&self, u: f64, v: f64) -> Vec3 {
        let dir_p =
            &self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin;
        dir_p
    }
}
