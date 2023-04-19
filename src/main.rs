use nannou::{
    image::{DynamicImage, GenericImage, Rgb, Rgba},
    prelude::*,
    wgpu::Texture,
};
use ray::{ray_sphere_collision, Sphere};

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
    s: Sphere,
}

fn model(_app: &App) -> Model {
    Model {
        image: DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        fov: 80., // degrees
        lighting_direction: Vec3::new(0., -1., 0.5).normalize(),
        s: Sphere {
            position: Vec3::new(0., 0., 20.),
            radius: 10.,
            color: Rgb::<f32>([1., 0.25, 1.]),
        },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    for y in -half_win_height..half_win_height {
        for x in -half_win_width..half_win_width {
            let pos = Vec3::ZERO;
            let dir = Vec3::new(
                ASPECT_RATIO * (model.fov / 2.).tan() * x as f32 / half_win_width as f32,
                (model.fov / 2.).tan() * y as f32 / half_win_height as f32,
                1.,
            );

            let hit_info = ray_sphere_collision(pos, dir, &model.s);

            let color = if let Some(hit) = hit_info {
                let lightness = (model.lighting_direction.dot(-hit.normal) + 1.) / 2.;
                let color = hit.color.0.map(|c| (c * lightness * 255.) as u8);
                Rgba::<u8>([color[0], color[1], color[2], 255])
            } else {
                Rgba::<u8>([0, 0, 0, 255])
            };

            model.image.put_pixel(
                (x + half_win_width) as u32,
                (y + half_win_height) as u32,
                color,
            );
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let texture = Texture::from_image(app, &model.image);
    let draw = app.draw();
    draw.texture(&texture).finish();
    draw.to_frame(app, &frame).unwrap();
}
