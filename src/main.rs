mod constants;
mod core;
mod display;

use display::Display;
use nannou::{app::LoopMode, color::Hsl, event::Update, frame::Frame, App};

fn main() {
    nannou::app(model)
        .update(update)
        .size(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}

struct Model {
    pub window_id: nannou::window::Id,
    pub display: Display,
    pub background_color: Hsl,
}

fn model<'model>(app: &App) -> Model {
    let path = constants::CONFIG_PATH.to_string();
    let config = core::config::load(path);

    let window_id = app.new_window().view(view).build().unwrap();
    let window = app.window(window_id).unwrap();
    let texture_size = [config.window.width as u32, config.window.height as u32];
    let display = Display::new(&window, texture_size);

    let background_color = Hsl::new(
        config.window.background_color.hue,
        config.window.background_color.saturation,
        config.window.background_color.lightness,
    );

    // Call the model init function here

    Model {
        window_id,
        display,
        background_color,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let draw = app.draw();

    draw.background().color(model.background_color);

    // Draw your objects here

    println!("\nCreating wgpu texture snapshot...");
    let window = app.window(model.window_id).unwrap();
    let snapshot = model.display.create_snapshot(&window, &draw);
    Display::save_snapshot(snapshot, captured_frame_path(app, "frame"));
    println!("Awaiting active wgpu snapshots...");
    model.display.wait(&window);
}

fn view(_app: &App, model: &Model, frame: Frame) {
    println!("Rendering texture to frame...");
    model.display.render(frame);
    println!("Done! You can exit the app.")
}

fn captured_frame_path(app: &nannou::app::App, name: &str) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("output")
        .join(name)
        .with_extension("png")
}
