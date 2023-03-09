use crate::vec::Vec3;

#[derive(Clone, Copy)]
pub struct Ray<'a> {
    pub origin: &'a Vec3,
    pub dir: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, dir: &'a Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}
