use nannou::{
    image::{DynamicImage, GenericImage, Rgba},
    prelude::*,
    wgpu::Texture,
};
use rand::Rng;

const WIN_WIDTH: usize = 1280;
const WIN_HEIGHT: usize = 720;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .run();
}

struct Model {
    image: DynamicImage,
    position: Vec3,
    radius: f32,
}

fn model(_app: &App) -> Model {
    Model {
        image: DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        position: Vec3::new(WIN_WIDTH as f32 / 2., WIN_HEIGHT as f32 / 2., 200.),
        radius: 50.,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let mut rng = rand::thread_rng();
    let circle = &mut model.position;
    circle.x -= 5. * update.since_last.as_secs_f32();

    for y in 0..WIN_HEIGHT {
        for x in 0..WIN_WIDTH {
            let pos = Vec3::new(x as f32, y as f32, 0.);
            let dir = Vec3::Z;

            let a = dir.x * dir.x + dir.y * dir.y + dir.z * dir.z;
            let b = 2. * pos.x * dir.x + 2. * pos.y * dir.y + 2. * pos.z * dir.z
                - 2. * dir.x * circle.x
                - 2. * dir.y * circle.y
                - 2. * dir.z * circle.z;
            let c = pos.x * pos.x + pos.y * pos.y + pos.z * pos.z
                - 2. * pos.x * circle.x
                - 2. * pos.y * circle.y
                - 2. * pos.z * circle.z
                + circle.x * circle.x
                + circle.y * circle.y
                + circle.z * circle.z
                - model.radius * model.radius;

            let det = b * b - 4. * a * c;

            let color = if det >= 0. {
                Rgba::<u8>([255, 255, 255, 255])
            } else {
                Rgba::<u8>([0, 0, 0, 255])
            };

            model.image.put_pixel(x as u32, y as u32, color);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let texture = Texture::from_image(app, &model.image);
    let draw = app.draw();
    draw.texture(&texture).finish();
    draw.to_frame(app, &frame).unwrap();
}
