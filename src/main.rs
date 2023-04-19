use nannou::{
    image::{DynamicImage, GenericImage, Rgba},
    prelude::*,
    wgpu::Texture,
};
use ray::{HitInfo, Shape, Sphere};

mod ray;

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
        lighting_direction: Vec3::new(0.5, 1., 0.25).normalize(),
        shapes: vec![
            Box::new(Sphere {
                position: Vec3::new(-2., 0., 20.),
                radius: 5.,
                color: [1., 0.25, 1.].into(),
            }),
            Box::new(Sphere {
                position: Vec3::new(2., 0., 10.),
                radius: 2.,
                color: [1., 1., 1.].into(),
            }),
        ],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Create fresh image
    //model.image = DynamicImage::new_rgba8(WIN_WIDTH as u32, WIN_HEIGHT as u32);

    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    for y in -half_win_height..half_win_height {
        for x in -half_win_width..half_win_width {
            let pos = Vec3::ZERO;
            let dir = Vec3::new(
                ASPECT_RATIO * (model.fov / 2.).tan() * x as f32 / half_win_width as f32,
                (model.fov / 2.).tan() * y as f32 / half_win_height as f32,
                1.,
            )
            .normalize();

            let mut closest_hit = HitInfo {
                hit_point: Vec3::ZERO,
                normal: Vec3::ZERO,
                color: [0., 0., 0.].into(),
                distance: f32::INFINITY,
            };

            for s in &model.shapes {
                let hit_info = s.ray_collision(pos, dir);

                if let Some(hit) = hit_info {
                    if hit.distance < closest_hit.distance {
                        closest_hit = hit;
                    }
                }
            }

            let lightness = (model.lighting_direction.dot(-closest_hit.normal) + 1.) / 2.;
            let color = closest_hit.color.0.map(|c| (c * lightness * 255.) as u8);
            let color = Rgba::<u8>([color[0], color[1], color[2], 255]);

            model.image.put_pixel(
                (x + half_win_width) as u32,
                (y + half_win_height) as u32,
                color,
            );
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
