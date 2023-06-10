use std::sync::Arc;

use nannou::{
    image::{DynamicImage, GenericImage},
    prelude::*,
    wgpu::Texture,
};
use shapes::{Sphere, Triangle};

mod renderer;
mod shapes;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;
const HALF_WIN_WIDTH: i32 = WIN_WIDTH / 2;
const HALF_WIN_HEIGHT: i32 = WIN_HEIGHT / 2;
const RAYS_PER_PIXEL: usize = 40;

pub struct HitInfo {
    hit_point: Vec3,
    normal: Vec3,
    material: Material,
    distance: f32,
}

/// Describes the material of an object.
/// If `emission_color` is `None`, it is assumed to be same as the `albedo`.
#[derive(Copy, Clone, Debug)]
pub struct Material {
    albedo: Vec3,
    roughness: f32,
    emission_color: Option<Vec3>,
    emission: f32,
}

/// Describes a ray-tracing scene and it's environment.
#[derive(Clone)]
pub struct Scene {
    fov: f32,
    sky_color: Vec3,
    camera_pos: Vec3,
    camera_dir: Vec3,
    camera_speed: f32,
    spheres: Vec<Sphere>,
    triangles: Vec<Triangle>,
}

fn main() {
    nannou::app(model)
        .event(event)
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

    let floor_material = Material {
        albedo: Vec3::new(0., 0.6, 0.09),
        roughness: 0.15,
        emission_color: None,
        emission: 0.4,
    };

    Model {
        image: DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32),
        thread_pool,
        scene: Scene {
            fov: 70.0.to_radians(), // degrees
            sky_color: Vec3::ZERO,  //Vec3::new(0.6, 0.87, 1.),
            camera_pos: Vec3::ZERO,
            camera_dir: Vec3::Z,
            camera_speed: 0.,
            spheres: vec![
                Sphere {
                    position: Vec3::new(3., 0., 8.),
                    radius: 2.,
                    material: Material {
                        albedo: Vec3::new(1., 0.35, 1.),
                        roughness: 0.15,
                        emission_color: None,
                        emission: 0.6,
                    },
                },
                Sphere {
                    position: Vec3::new(-3., 0., 8.),
                    radius: 2.,
                    material: Material {
                        albedo: Vec3::ONE, //Vec3::new(0.07, 0.06, 0.73),
                        roughness: 0.15,
                        emission_color: None,
                        emission: 1.,
                    },
                },
            ],
            triangles: vec![
                Triangle {
                    v0: Vec3::new(-20., 2., 20.),
                    v1: Vec3::new(-20., 2., -20.),
                    v2: Vec3::new(20., 2., -20.),
                    material: floor_material,
                },
                Triangle {
                    v0: Vec3::new(-20., 2., 20.),
                    v1: Vec3::new(20., 2., 20.),
                    v2: Vec3::new(20., 2., -20.),
                    material: floor_material,
                },
            ],
        },
    }
}

fn event(app: &App, model: &mut Model, ev: Event) {
    match ev {
        Event::WindowEvent { id, simple: win_ev } => {
            if id != app.window_id() || win_ev.is_none() {
                return;
            }

            match win_ev.unwrap() {
                WindowEvent::KeyPressed(Key::W) => model.scene.camera_speed = 1.,
                WindowEvent::KeyPressed(Key::S) => model.scene.camera_speed = -1.,
                WindowEvent::KeyReleased(Key::W | Key::S) => model.scene.camera_speed = 0.,
                _ => {}
            }
        }
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Create fresh image
    //model.image = DynamicImage::new_rgb8(WIN_WIDTH as u32, WIN_HEIGHT as u32);

    //model.scene.spheres[0].translate(-Vec3::X * update.since_last.as_secs_f32() * 0.5);

    // Camera movement
    model.scene.camera_pos += model.scene.camera_dir * model.scene.camera_speed;

    let arc_scene = Arc::new(model.scene.clone());
    let (tx, rx) = crossbeam::channel::unbounded::<(i32, i32, Vec3)>();

    for y in -HALF_WIN_HEIGHT..HALF_WIN_HEIGHT {
        let scene = arc_scene.clone();
        let tx_clone = tx.clone();

        model.thread_pool.spawn(move || {
            for x in -HALF_WIN_WIDTH..HALF_WIN_WIDTH {
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
            (x + HALF_WIN_WIDTH) as u32,
            (y + HALF_WIN_HEIGHT) as u32,
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
