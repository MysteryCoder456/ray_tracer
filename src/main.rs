use nannou::prelude::*;
use std::time::Duration;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .loop_mode(LoopMode::Rate {
            update_interval: Duration::from_secs_f32(1. / 60.),
        })
        .run();
}

struct Model {
    angle: f32,
}

fn model(_app: &App) -> Model {
    Model { angle: 0. }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.angle += update.since_last.as_secs_f32();
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(SKYBLUE);

    let draw = app.draw();
    draw.tri()
        .rgb(1., 1., 1.)
        .w_h(173., 200.)
        .z_radians(model.angle)
        .finish();

    draw.to_frame(app, &frame).unwrap();
}
