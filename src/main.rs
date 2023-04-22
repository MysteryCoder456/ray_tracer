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
const RAYS_PER_PIXEL: usize = 20;

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
    thread_pool: rayon::ThreadPool,
    scene: Scene,
}

fn model(_app: &App) -> Model {
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    Model {
        image: DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        thread_pool,
        scene: Scene {
            fov: 70.0.to_radians(), // degrees
            lighting_direction: Vec3::new(0.4, 1., 0.4).normalize(),
            sky_color: Vec3::new(0.34, 0.62, 0.93),
            spheres: vec![
                Sphere {
                    position: Vec3::new(0., 201., 10.),
                    radius: 200.,
                    material: Material {
                        albedo: [0.3, 0.5, 0.9].into(),
                        roughness: 0.15,
                        metallic: 1.,
                    },
                },
                Sphere {
                    position: Vec3::new(0., -1.25, 10.),
                    radius: 2.,
                    material: Material {
                        albedo: [1., 0.25, 1.].into(),
                        roughness: 0.1,
                        metallic: 1.,
                    },
                },
            ],
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // Create fresh image
    //model.image = DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32);

    //model.scene.spheres[1].translate(Vec3::X * update.since_last.as_secs_f32() * 0.5);
    //model.scene.fov = (model.scene.fov - 0.1 * update.since_last.as_secs_f32()).max(0.01);

    let half_win_width = WIN_WIDTH / 2;
    let half_win_height = WIN_HEIGHT / 2;

    let (tx, rx) = crossbeam::channel::unbounded::<(i32, i32, Vec3)>();

    for y in -half_win_height..half_win_height {
        let tx_clone = tx.clone();
        let scene = model.scene.clone();

        model.thread_pool.spawn(move || {
            for x in -half_win_width..half_win_width {
                let pixel_color = (0..RAYS_PER_PIXEL)
                    .map(|_| renderer::per_pixel(x as f32, y as f32, &scene))
                    .reduce(|a, b| a + b)
                    .unwrap()
                    / RAYS_PER_PIXEL as f32;
                tx_clone.send((x, y, pixel_color)).unwrap();
            }
        });
    }

    for (i, (x, y, pixel_color)) in rx.iter().enumerate() {
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

        if i + 1 == (WIN_WIDTH * WIN_HEIGHT) as usize {
            break;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let texture = Texture::from_image(app, &model.image);
    let draw = app.draw();
    draw.texture(&texture).finish();
    draw.to_frame(app, &frame).unwrap();
}
