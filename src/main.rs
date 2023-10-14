use nannou::prelude::*;

// Model is run once at start of program to initalise windows and ui things
struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model)
    .event(event)
    .simple_window(view)
    .run();
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("patrick.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Model { texture }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let c = app.draw();
    c.background().color(WHITE);
    
    let win = app.window_rect();
    let r = Rect::from_w_h(500.0, 500.0).middle_of(win);
    c.texture(&model.texture).xy(r.xy()).wh(r.wh());

    // Make image black/white
    c.to_frame(app, &frame).unwrap();
}

