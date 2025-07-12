use gtk4 as gtk;

use crate::ui::DesktopFileManagerWindow;

pub struct DesktopFileManagerApp {
    window: DesktopFileManagerWindow,
}

impl DesktopFileManagerApp {
    pub fn new(app: &gtk::Application) -> Self {
        let window = DesktopFileManagerWindow::new(app);
        Self { window }
    }

    pub fn run(self) {
        self.window.show();
    }
} 