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
        position: Vec3::new(0., 0., 50.),
        radius: 50.,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut rng = rand::thread_rng();

    for y in 0..WIN_HEIGHT {
        for x in 0..WIN_WIDTH {
            let r = (x as f32 / WIN_WIDTH as f32 * 255.) as u8;
            let g = (y as f32 / WIN_HEIGHT as f32 * 255.) as u8;
            let b = 130;

            model
                .image
                .put_pixel(x as u32, y as u32, Rgba::<u8>([r, g, b, 255]));
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
