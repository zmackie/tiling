use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).run();
}

const COLORS: [nannou::color::Rgb<u8>;2] = [ALICEBLUE, HOTPINK];

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    draw.background().hsv(0.2, 0.5, 0.2);
    let win_p = win.pad(30.0);
    let mut rect = Rect::from_w_h(120.0,120.0).top_left_of(win_p);
    rect = draw_tiles(&draw, &rect, 12, 120, 0.0).right_of(rect);
    draw_tiles(&draw, &rect, 12, 120, 0.0);
    draw.to_frame(app, &frame).unwrap();
}


fn draw_tiles(draw: &Draw, rect: &Rect<f32>, steps: i32, size: i32, rot: f32) -> Rect<f32> {
    let overall = *rect;
    let sub_size: i32 = size / steps;
    
    for tile in 0..steps as i32 {
        let x = overall.x() as i32+ tile * sub_size;
        let step: usize = tile as usize % 2;
        draw.rect()
        .x_y(x as f32, overall.y() as f32)
        .w_h(sub_size as f32, overall.h() as f32)
        .z_degrees(rot)
        .color(COLORS[step]);
    }
    
    overall
}
