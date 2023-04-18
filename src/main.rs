use nannou::{
    image::{DynamicImage, GenericImage, Rgba},
    prelude::*,
    wgpu::Texture,
};

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 800;

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
    position: Vec3,
    radius: f32,
}

fn model(_app: &App) -> Model {
    Model {
        image: DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        fov: 80., // degrees
        position: Vec3::new(0., 0., 50.),
        radius: 10.,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let circle = model.position;

    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    for y in -half_win_height..half_win_height {
        for x in -half_win_width..half_win_width {
            let pos = Vec3::ZERO;
            let dir = Vec3::new(
                (model.fov / 2.).tan() * x as f32 / half_win_width as f32,
                (model.fov / 2.).tan() * y as f32 / half_win_height as f32,
                1.,
            )
            .normalize();

            let a = dir.dot(dir);
            let b = 2. * pos.dot(dir) - 2. * dir.dot(circle);
            let c = pos.dot(pos) - 2. * pos.dot(circle) + circle.dot(circle)
                - model.radius * model.radius;
            let d = b * b - 4. * a * c;

            let color = if d >= 0. {
                Rgba::<u8>([255, 255, 255, 255])
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
