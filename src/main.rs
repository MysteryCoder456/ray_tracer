use nannou::{
    image::{DynamicImage, GenericImage, Pixel, Rgb},
    prelude::*,
    wgpu::Texture,
};
use rand::Rng;
use shapes::{HitInfo, Shape, Sphere};

mod shapes;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;
const ASPECT_RATIO: f32 = WIN_WIDTH as f32 / WIN_HEIGHT as f32;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .run();
}

struct Model {
    image: DynamicImage,
    fov: f32,
    lighting_direction: Vec3,
    shapes: Vec<Box<dyn Shape>>,
}

fn model(_app: &App) -> Model {
    Model {
        image: DynamicImage::new_rgba8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        fov: 70., // degrees
        lighting_direction: Vec3::new(0., 0., 1.).normalize(),
        shapes: vec![
            Box::new(Sphere {
                position: Vec3::new(0., 2., 20.),
                radius: 3.,
                color: [1., 1., 1.].into(),
            }),
            Box::new(Sphere {
                position: Vec3::new(-6., -2., 14.),
                radius: 3.,
                color: [1., 0.25, 1.].into(),
            }),
        ],
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // Create fresh image
    model.image = DynamicImage::new_rgba8(WIN_WIDTH as u32, WIN_HEIGHT as u32);

    model.shapes[1]
        .as_mut()
        .translate(Vec3::X * update.since_last.as_secs_f32());

    let mut rng = rand::thread_rng();
    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    for y in -half_win_height..half_win_height {
        for x in -half_win_width..half_win_width {
            let mut ray_origin = Vec3::ZERO;
            let mut ray_dir = Vec3::new(
                ASPECT_RATIO * (model.fov / 2.).tan() * x as f32 / half_win_width as f32,
                (model.fov / 2.).tan() * y as f32 / half_win_height as f32,
                1.,
            )
            .normalize();
            let mut pixel_color: Option<Rgb<f32>> = None;

            for _ in 0..2 {
                let mut closest_hit: Option<HitInfo> = None;

                for s in &model.shapes {
                    let hit_info = s.ray_collision(ray_origin, ray_dir);

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

                if let Some(closest_hit) = closest_hit {
                    let lightness = model.lighting_direction.dot(-closest_hit.normal).max(0.);
                    let color = if let Some(pc) = pixel_color {
                        let hit_color = closest_hit.color.0;
                        [
                            hit_color[0] * pc.0[0] * lightness,
                            hit_color[1] * pc.0[1] * lightness,
                            hit_color[2] * pc.0[2] * lightness,
                        ]
                    } else {
                        closest_hit.color.0.map(|c| c * lightness)
                    };
                    pixel_color = Some(color.into());

                    // ray gets reflected about the normal
                    ray_origin = closest_hit.hit_point;
                    ray_dir = (ray_dir
                        - 2. * ray_dir.dot(closest_hit.normal) * -closest_hit.normal)
                        + Vec3::new(
                            rng.gen_range(-8..8) as f32 / 100.,
                            rng.gen_range(-8..8) as f32 / 100.,
                            rng.gen_range(-8..8) as f32 / 100.,
                        );
                } else {
                    break;
                }
            }

            if let Some(color) = pixel_color {
                let mapped_color = color.to_rgba().0.map(|c| (c * 255.) as u8).into();
                model.image.put_pixel(
                    (x + half_win_width) as u32,
                    (y + half_win_height) as u32,
                    mapped_color,
                );
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let texture = Texture::from_image(app, &model.image);
    let draw = app.draw();
    draw.texture(&texture).finish();
    draw.to_frame(app, &frame).unwrap();
}
