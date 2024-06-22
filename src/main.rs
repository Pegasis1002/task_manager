mod app;
use app::app::App;
mod ui;
use ui::ui::*;
mod utils;

fn main() {
    // Initialize the application
    let mut app = App::new();

    // Run the main loop
    app.run();
}

