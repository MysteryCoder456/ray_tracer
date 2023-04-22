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
const RAYS_PER_PIXEL: usize = 6;

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

#[derive(Clone)]
pub struct Scene {
    fov: f32,
    lighting_direction: Vec3,
    sky_color: Vec3,
    spheres: Vec<Sphere>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .run();
}

struct Model {
    image: DynamicImage,
    scene: Scene,
}

fn model(_app: &App) -> Model {
    Model {
        image: DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        scene: Scene {
            fov: 70., // degrees
            lighting_direction: Vec3::new(0.4, 1., 0.4).normalize(),
            sky_color: Vec3::new(0.34, 0.62, 0.93),
            spheres: vec![
                Sphere {
                    position: Vec3::new(0., 201., 10.),
                    radius: 200.,
                    material: Material {
                        albedo: [0.3, 0.5, 0.9].into(),
                        roughness: 0.2,
                        metallic: 1.,
                    },
                },
                Sphere {
                    position: Vec3::new(-5., -1., 10.),
                    radius: 2.,
                    material: Material {
                        albedo: [1., 0.25, 1.].into(),
                        roughness: 0.,
                        metallic: 1.,
                    },
                },
            ],
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // Create fresh image
    //model.image = DynamicImage::new_rgba8(WIN_WIDTH as u32, WIN_HEIGHT as u32);

    model.scene.spheres[1].translate(Vec3::X * update.since_last.as_secs_f32() * 0.5);

    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    // FIXME: Ray tracer is not ray tracing
    for y in -half_win_height..half_win_height {
        for x in -half_win_width..half_win_width {
            let mut pixel_color = Vec3::ZERO;
            (0..RAYS_PER_PIXEL)
                .map(|_| {
                    let scene = model.scene.clone();
                    std::thread::spawn(move || renderer::per_pixel(x as f32, y as f32, &scene))
                })
                .for_each(|h| {
                    pixel_color += h.join().unwrap();
                });
            pixel_color /= RAYS_PER_PIXEL as f32;

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
    let texture = Texture::from_image(app, &model.image);
    let draw = app.draw();
    draw.texture(&texture).finish();
    draw.to_frame(app, &frame).unwrap();
}
