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

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(SKYBLUE);
}
