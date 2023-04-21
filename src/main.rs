use nannou::{
    image::{DynamicImage, GenericImage},
    prelude::*,
    wgpu::Texture,
};
use shapes::{Shape, Sphere};

mod renderer;
mod shapes;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;

pub struct HitInfo {
    hit_point: Vec3,
    normal: Vec3,
    material: Material,
    distance: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    albedo: Vec3,
    roughness: f32,
    metallic: f32,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .run();
}

pub struct Model {
    image: DynamicImage,
    fov: f32,
    lighting_direction: Vec3,
    sky_color: Vec3,
    shapes: Vec<Box<dyn Shape>>,
}

fn model(_app: &App) -> Model {
    Model {
        image: DynamicImage::new_rgba8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        fov: 70., // degrees
        lighting_direction: Vec3::new(0.4, 1., 0.4).normalize(),
        sky_color: Vec3::new(0.34, 0.62, 0.93),
        shapes: vec![
            Box::new(Sphere {
                position: Vec3::new(0., 202., 15.),
                radius: 200.,
                material: Material {
                    albedo: [0.22, 0.45, 0.96].into(),
                    roughness: 0.05,
                    metallic: 1.,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0., 0., 10.),
                radius: 2.,
                material: Material {
                    albedo: [1., 0.25, 1.].into(),
                    roughness: 0.,
                    metallic: 1.,
                },
            }),
        ],
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // Create fresh image
    //model.image = DynamicImage::new_rgba8(WIN_WIDTH as u32, WIN_HEIGHT as u32);

    model.shapes[1]
        .as_mut()
        .translate(Vec3::Z * update.since_last.as_secs_f32() * 0.5);

    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    for y in -half_win_height..half_win_height {
        for x in -half_win_width..half_win_width {
            let pixel_color = renderer::per_pixel(x as f32, y as f32, &model);

            model.image.put_pixel(
                (x + half_win_width) as u32,
                (y + half_win_height) as u32,
                [
                    (pixel_color.x * 255.) as u8,
                    (pixel_color.y * 255.) as u8,
                    (pixel_color.z * 255.) as u8,
                    255,
                ]
                .into(),
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
