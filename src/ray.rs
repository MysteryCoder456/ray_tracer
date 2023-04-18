use nannou::prelude::*;

pub struct HitInfo {
    pub hit_point: Vec3,
    pub normal: Vec3,
}

pub fn ray_circle_collision(
    ray_pos: Vec3,
    ray_dir: Vec3,
    circle_pos: Vec3,
    circle_radius: f32,
) -> Option<HitInfo> {
    let a = ray_dir.dot(ray_dir);
    let b = 2. * ray_pos.dot(ray_dir) - 2. * ray_dir.dot(circle_pos);
    let c = ray_pos.dot(ray_pos) - 2. * ray_pos.dot(circle_pos) + circle_pos.dot(circle_pos)
        - circle_radius * circle_radius;
    let d = b * b - 4. * a * c;

    if d < 0. {
        return None;
    }

    let t = (-b - d.sqrt()) / 2. * a;
    let hit_point = ray_pos + ray_dir * t;

    Some(HitInfo {
        hit_point,
        normal: (hit_point - circle_pos).normalize_or_zero(),
    })
}
