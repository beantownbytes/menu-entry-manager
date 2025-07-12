use gtk::prelude::*;
use gtk4 as gtk;

mod app;
mod desktop_file;
mod ui;

use app::DesktopFileManagerApp;

fn main() {
    let app = gtk::Application::new(Some("com.example.desktopfilemanager"), Default::default());
    app.connect_activate(|app| {
        let manager = DesktopFileManagerApp::new(app);
        manager.run();
    });
    app.run();
}
