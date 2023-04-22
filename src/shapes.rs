use crate::{HitInfo, Material};
use nannou::prelude::*;

pub trait Shape: Clone {
    fn material(&self) -> Material;
    fn translate(&mut self, v: Vec3);
    fn ray_collision(&self, ray_pos: Vec3, ray_dir: Vec3) -> Option<HitInfo>;
}

#[derive(Clone)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Shape for Sphere {
    fn material(&self) -> Material {
        self.material
    }

    fn translate(&mut self, v: Vec3) {
        self.position += v;
    }

    fn ray_collision(&self, ray_pos: Vec3, ray_dir: Vec3) -> Option<HitInfo> {
        let ray_pos = ray_pos - self.position;

        let a = ray_dir.dot(ray_dir);
        let b = 2. * ray_pos.dot(ray_dir);
        let c = ray_pos.dot(ray_pos) - self.radius * self.radius;
        let d = b * b - 4. * a * c;

        if d < 0. {
            return None;
        }

        let t = (-b - d.sqrt()) / 2. * a;

        if t < 0. {
            return None;
        }

        let hit_point = ray_pos + self.position + ray_dir * t;

        Some(HitInfo {
            hit_point,
            normal: (hit_point - self.position).normalize_or_zero(),
            material: self.material,
            distance: t,
        })
    }
}
