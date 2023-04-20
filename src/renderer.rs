use crate::{HitInfo, Model, WIN_HEIGHT, WIN_WIDTH};
use nannou::prelude::*;

const ASPECT_RATIO: f32 = WIN_WIDTH as f32 / WIN_HEIGHT as f32;
const MAX_RAY_BOUNCES: usize = 2;

pub fn per_pixel(x: f32, y: f32, model: &Model) -> Vec3 {
    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    let mut ray_origin = Vec3::ZERO;
    let mut ray_dir = Vec3::new(
        ASPECT_RATIO * (model.fov / 2.).tan() * x as f32 / half_win_width as f32,
        (model.fov / 2.).tan() * y as f32 / half_win_height as f32,
        1.,
    )
    .normalize();

    let mut color_multiplier = 1.;
    let mut bounce_count = 0;
    let mut final_color = Vec3::ZERO; // Initial value of this is the sky color

    for _ in 0..MAX_RAY_BOUNCES {
        let closest_hit = trace_ray(ray_origin, ray_dir, &model);

        if let Some(hit) = closest_hit {
            let light_intensity = model.lighting_direction.dot(-hit.normal).max(0.);
            let shape_color = hit.color * light_intensity;
            final_color += shape_color * color_multiplier;
            bounce_count += 1;
            color_multiplier *= 0.7;

            // ray gets reflected about the normal
            ray_origin = hit.hit_point + hit.normal * 0.01;
            ray_dir = ray_dir + 2. * hit.normal.dot(ray_dir) * hit.normal;
        } else {
            break;
        }
    }

    final_color / bounce_count as f32
}

fn trace_ray(ray_origin: Vec3, ray_direction: Vec3, model: &Model) -> Option<HitInfo> {
    let mut closest_hit: Option<HitInfo> = None;

    for s in &model.shapes {
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

    closest_hit
}