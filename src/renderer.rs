use crate::{
    shapes::Shape, HitInfo, Scene, HALF_WIN_HEIGHT, HALF_WIN_WIDTH, WIN_HEIGHT, WIN_WIDTH,
};
use nannou::prelude::*;

const ASPECT_RATIO: f32 = WIN_WIDTH as f32 / WIN_HEIGHT as f32;
const MAX_RAY_BOUNCES: usize = 3;

pub fn per_pixel(x: f32, y: f32, scene: &Scene) -> Vec3 {
    let rng = fastrand::Rng::new();

    let mut ray_origin = scene.camera_pos;
    let mut ray_dir = Vec3::new(
        ASPECT_RATIO * (scene.fov / 2.).tan() * x / HALF_WIN_WIDTH as f32,
        (scene.fov / 2.).tan() * y / HALF_WIN_HEIGHT as f32,
        1.,
    )
    .normalize();

    let mut color_contribution = Vec3::ONE;
    let mut final_color = Vec3::ZERO;

    for _ in 0..MAX_RAY_BOUNCES {
        let closest_hit = trace_ray(ray_origin, ray_dir, scene);

        if let Some(hit) = closest_hit {
            // ray gets diffused in random direction
            // TODO: double normalize = yikes
            ray_origin = hit.hit_point + hit.normal * 0.01;
            ray_dir = (Vec3::new(
                rng.f32() * 2. - 1.,
                rng.f32() * 2. - 1.,
                rng.f32() * 2. - 1.,
            )
            .normalize()
                + hit.normal)
                .normalize();

            color_contribution *= hit.material.albedo;
            final_color +=
                hit.material.emission_color.unwrap_or(hit.material.albedo) * hit.material.emission;
        } else {
            final_color += scene.sky_color * color_contribution;
            break;
        }
    }

    final_color
}

fn trace_ray(ray_origin: Vec3, ray_direction: Vec3, scene: &Scene) -> Option<HitInfo> {
    let mut closest_hit: Option<HitInfo> = None;

    for s in &scene.spheres {
        let hit_info = s.ray_collision(ray_origin, ray_direction);

        if let Some(hit) = hit_info {
            if let Some(ref ch) = closest_hit {
                if hit.distance < ch.distance {
                    closest_hit = Some(hit);
                }
            } else {
                closest_hit = Some(hit);
            }
        }
    }

    for t in &scene.triangles {
        let hit_info = t.ray_collision(ray_origin, ray_direction);

        if let Some(hit) = hit_info {
            if let Some(ref ch) = closest_hit {
                if hit.distance < ch.distance {
                    closest_hit = Some(hit);
                }
            } else {
                closest_hit = Some(hit);
            }
        }
    }

    closest_hit
}
