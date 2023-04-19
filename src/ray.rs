use nannou::{image::Rgb, prelude::*};

pub struct HitInfo {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub color: Rgb<f32>,
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: Rgb<f32>,
}

pub fn ray_sphere_collision(ray_pos: Vec3, ray_dir: Vec3, sphere: &Sphere) -> Option<HitInfo> {
    let a = ray_dir.dot(ray_dir);
    let b = 2. * ray_pos.dot(ray_dir) - 2. * ray_dir.dot(sphere.position);
    let c = ray_pos.dot(ray_pos) - 2. * ray_pos.dot(sphere.position)
        + sphere.position.dot(sphere.position)
        - sphere.radius * sphere.radius;
    let d = b * b - 4. * a * c;

    if d < 0. {
        return None;
    }

    let t = (-b - d.sqrt()) / 2. * a;
    let hit_point = ray_pos + ray_dir * t;

    Some(HitInfo {
        hit_point,
        normal: (hit_point - sphere.position).normalize_or_zero(),
        color: sphere.color,
    })
}
