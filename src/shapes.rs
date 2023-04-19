use nannou::{image::Rgb, prelude::*};

pub struct HitInfo {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub color: Rgb<f32>,
    pub distance: f32,
}

pub trait Shape {
    fn translate(&mut self, v: Vec3);
    fn ray_collision(&self, ray_pos: Vec3, ray_dir: Vec3) -> Option<HitInfo>;
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: Rgb<f32>,
}

impl Shape for Sphere {
    fn translate(&mut self, v: Vec3) {
        self.position += v;
    }

    fn ray_collision(&self, ray_pos: Vec3, ray_dir: Vec3) -> Option<HitInfo> {
        let a = ray_dir.dot(ray_dir);
        let b = 2. * ray_pos.dot(ray_dir) - 2. * ray_dir.dot(self.position);
        let c = ray_pos.dot(ray_pos) - 2. * ray_pos.dot(self.position)
            + self.position.dot(self.position)
            - self.radius * self.radius;
        let d = b * b - 4. * a * c;

        if d < 0. {
            return None;
        }

        let t = (-b - d.sqrt()) / 2. * a;
        let hit_point = ray_pos + ray_dir * t;

        Some(HitInfo {
            hit_point,
            normal: (hit_point - self.position).normalize_or_zero(),
            color: self.color,
            distance: t,
        })
    }
}
