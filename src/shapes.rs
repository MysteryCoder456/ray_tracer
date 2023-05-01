use crate::{HitInfo, Material};
use nannou::prelude::*;

pub trait Shape: Clone {
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

#[derive(Clone)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub material: Material,
}

impl Shape for Triangle {
    fn translate(&mut self, v: Vec3) {
        self.v0 += v;
        self.v1 += v;
        self.v2 += v;
    }

    fn ray_collision(&self, ray_pos: Vec3, ray_dir: Vec3) -> Option<HitInfo> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = ray_dir.cross(edge2);
        let a = edge1.dot(h);

        if a.abs() < 1E-6 {
            return None;
        }

        let f = 1. / a;
        let s = ray_pos - self.v0;
        let u = f * s.dot(h);

        if u < 0. || u > 1. {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray_dir.dot(q);

        if v < 0. || u + v > 1. {
            return None;
        }

        let t = f * edge2.dot(q);

        if t < 1E-6 {
            return None;
        }

        // FIXME: HELP
        let normal = -Vec3::Y; //edge1.cross(edge2).normalize_or_zero();

        Some(HitInfo {
            hit_point: ray_pos + ray_dir * t,
            normal,
            material: self.material,
            distance: t,
        })
    }
}
