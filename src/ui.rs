use gtk4 as gtk;
use gtk::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use glib::Propagation;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::Path;
use std::env;
use std::collections::HashMap;

use crate::desktop_file::{DesktopFile, get_desktop_file_paths};

const COMMON_CATEGORIES: &[&str] = &[
    "AudioVideo", "Development", "Education", "Game", "Graphics", "Network", "Office", "Settings", "System", "Utility"
];

pub struct DesktopFileManagerWindow {
    window: adw::ApplicationWindow,
    #[allow(dead_code)]
    current_file: Rc<RefCell<Option<DesktopFile>>>,
    #[allow(dead_code)]
    file_path: Rc<RefCell<Option<String>>>,
    #[allow(dead_code)]
    updating_ui: Rc<Cell<bool>>,
    
    #[allow(dead_code)]
    name_entry: gtk::Entry,
    #[allow(dead_code)]
    exec_entry: gtk::Entry,
    #[allow(dead_code)]
    comment_entry: gtk::Entry,
    #[allow(dead_code)]
    icon_entry: gtk::Entry,
    #[allow(dead_code)]
    path_entry: gtk::Entry,
    #[allow(dead_code)]
    categories_popover: gtk::Popover,
    #[allow(dead_code)]
    categories_checkboxes: Vec<gtk::CheckButton>,
    #[allow(dead_code)]
    categories_custom_entry: gtk::Entry,
    #[allow(dead_code)]
    categories_visible_entry: gtk::Entry,
    #[allow(dead_code)]
    keywords_entry: gtk::Entry,
    #[allow(dead_code)]
    terminal_switch: gtk::Switch,
    #[allow(dead_code)]
    hidden_switch: gtk::Switch,
    #[allow(dead_code)]
    type_combo: gtk::ComboBoxText,
    #[allow(dead_code)]
    url_entry: gtk::Entry,
    #[allow(dead_code)]
    mime_type_entry: gtk::Entry,
    
    #[allow(dead_code)]
    file_list: gtk::ListBox,
    #[allow(dead_code)]
    search_entry: gtk::Entry,
}

impl DesktopFileManagerWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = adw::ApplicationWindow::new(app);
        window.set_title(Some("Menu Entry Manager"));
        window.set_default_size(1200, 800);
        
        let current_file = Rc::new(RefCell::new(None));
        let file_path = Rc::new(RefCell::new(None));
        let updating_ui = Rc::new(Cell::new(false));
        
        let categories_popover = gtk::Popover::new();
        let categories_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let mut categories_checkboxes = Vec::new();
        for &cat in COMMON_CATEGORIES {
            let check = gtk::CheckButton::with_label(cat);
            categories_box.append(&check);
            categories_checkboxes.push(check);
        }
        let categories_custom_entry = gtk::Entry::new();
        categories_custom_entry.set_placeholder_text(Some("Custom categories (semicolon separated)"));
        println!("Created categories entry with ID: {:?}", categories_custom_entry.as_ptr());
        categories_box.append(&categories_custom_entry);
        categories_popover.set_child(Some(&categories_box));
        
        let (name_entry, exec_entry, comment_entry, icon_entry, path_entry, 
             keywords_entry, terminal_switch, hidden_switch,
             type_combo, url_entry, mime_type_entry) = Self::create_form_fields();
        
        let file_list = gtk::ListBox::new();
        file_list.set_selection_mode(gtk::SelectionMode::Single);
        
        let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);
        
        let (left_panel, new_button, search_entry) = Self::create_left_panel(&file_list);
        left_panel.set_hexpand(false);
        left_panel.set_vexpand(true);
        
        let (right_panel, save_button, delete_button, categories_visible_entry) = Self::create_right_panel(
            &name_entry, &exec_entry, &comment_entry, &icon_entry, &path_entry,
            &keywords_entry, &terminal_switch, &hidden_switch,
            &type_combo, &url_entry, &mime_type_entry, &categories_popover, &categories_checkboxes, &categories_custom_entry
        );
        right_panel.set_hexpand(true);
        right_panel.set_vexpand(true);
        
        main_box.append(&left_panel);
        main_box.append(&right_panel);
        
        window.set_content(Some(&main_box));
        
        Self::connect_signals(
            &window, &current_file, &file_path, &updating_ui, &file_list,
            &name_entry, &exec_entry, &comment_entry, &icon_entry, &path_entry,
            &keywords_entry, &terminal_switch, &hidden_switch,
            &type_combo, &url_entry, &mime_type_entry, &new_button, &save_button, &delete_button,
            &categories_popover, &categories_checkboxes, &categories_custom_entry, &categories_visible_entry, &search_entry
        );
        
        Self::load_desktop_files(&file_list);
        
        Self {
            window,
            current_file,
            file_path,
            updating_ui,
            name_entry,
            exec_entry,
            comment_entry,
            icon_entry,
            path_entry,
            categories_popover,
            categories_checkboxes,
            categories_custom_entry,
            categories_visible_entry,
            keywords_entry,
            terminal_switch,
            hidden_switch,
            type_combo,
            url_entry,
            mime_type_entry,
            file_list,
            search_entry,
        }
    }
    
    fn create_form_fields() -> (
        gtk::Entry, gtk::Entry, gtk::Entry, gtk::Entry, gtk::Entry,
        gtk::Entry, gtk::Switch, gtk::Switch,
        gtk::ComboBoxText, gtk::Entry, gtk::Entry
    ) {
        let name_entry = gtk::Entry::new();
        name_entry.set_placeholder_text(Some("Application Name"));
        
        let exec_entry = gtk::Entry::new();
        exec_entry.set_placeholder_text(Some("Command to execute"));
        
        let comment_entry = gtk::Entry::new();
        comment_entry.set_placeholder_text(Some("Description"));
        
        let icon_entry = gtk::Entry::new();
        icon_entry.set_placeholder_text(Some("Icon path or name"));
        
        let path_entry = gtk::Entry::new();
        path_entry.set_placeholder_text(Some("Working directory"));
        
        let keywords_entry = gtk::Entry::new();
        keywords_entry.set_placeholder_text(Some("Keywords (semicolon separated)"));
        
        let terminal_switch = gtk::Switch::new();
        terminal_switch.set_active(false);
        
        let hidden_switch = gtk::Switch::new();
        hidden_switch.set_active(false);
        
        let type_combo = gtk::ComboBoxText::new();
        type_combo.append_text("Application");
        type_combo.append_text("Link");
        type_combo.append_text("Directory");
        type_combo.set_active(Some(0));
        
        let url_entry = gtk::Entry::new();
        url_entry.set_placeholder_text(Some("URL (for Link type)"));
        url_entry.set_sensitive(false);
        
        let mime_type_entry = gtk::Entry::new();
        mime_type_entry.set_placeholder_text(Some("MIME types (semicolon separated)"));
        
        (name_entry, exec_entry, comment_entry, icon_entry, path_entry,
         keywords_entry, terminal_switch, hidden_switch,
         type_combo, url_entry, mime_type_entry)
    }
    
    #[allow(dead_code)]
    fn create_file_list() -> gtk::ListBox {
        let list = gtk::ListBox::new();
        list.set_selection_mode(gtk::SelectionMode::Single);
        list
    }
    
    fn create_left_panel(file_list: &gtk::ListBox) -> (gtk::Box, gtk::Button, gtk::Entry) {
        let panel = gtk::Box::new(gtk::Orientation::Vertical, 12);
        panel.set_margin_start(12);
        panel.set_margin_end(12);
        panel.set_margin_top(12);
        panel.set_margin_bottom(12);
        panel.set_hexpand(false);
        panel.set_vexpand(true);
        
        let header = adw::HeaderBar::new();
        let title = gtk::Label::new(Some("Desktop Files"));
        title.add_css_class("title-2");
        header.set_title_widget(Some(&title));
        
        let new_button = gtk::Button::from_icon_name("document-new-symbolic");
        new_button.set_tooltip_text(Some("Create New Desktop File"));
        header.pack_start(&new_button);
        
        let search_entry = gtk::Entry::new();
        search_entry.set_placeholder_text(Some("Search desktop files..."));
        search_entry.set_hexpand(true);
        
        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_child(Some(file_list));
        scrolled.set_min_content_width(300);
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        
        panel.append(&header);
        panel.append(&search_entry);
        panel.append(&scrolled);
        
        (panel, new_button, search_entry)
    }
    
    fn create_right_panel(
        name_entry: &gtk::Entry, exec_entry: &gtk::Entry, comment_entry: &gtk::Entry,
        icon_entry: &gtk::Entry, path_entry: &gtk::Entry,
        keywords_entry: &gtk::Entry, terminal_switch: &gtk::Switch, hidden_switch: &gtk::Switch,
        type_combo: &gtk::ComboBoxText, url_entry: &gtk::Entry, mime_type_entry: &gtk::Entry,
        _categories_popover: &gtk::Popover, _categories_checkboxes: &[gtk::CheckButton], _categories_custom_entry: &gtk::Entry
    ) -> (gtk::Box, gtk::Button, gtk::Button, gtk::Entry) {
        let panel = gtk::Box::new(gtk::Orientation::Vertical, 12);
        panel.set_margin_start(12);
        panel.set_margin_end(12);
        panel.set_margin_top(12);
        panel.set_margin_bottom(12);
        panel.set_hexpand(true);
        panel.set_vexpand(true);
        
        let header = adw::HeaderBar::new();
        let title = gtk::Label::new(Some("Desktop File Editor"));
        title.add_css_class("title-2");
        header.set_title_widget(Some(&title));
        
        let delete_button = gtk::Button::from_icon_name("user-trash-symbolic");
        delete_button.set_tooltip_text(Some("Delete Desktop File"));
        delete_button.add_css_class("destructive-action");
        delete_button.set_visible(false);
        header.pack_end(&delete_button);
        
        let save_button = gtk::Button::from_icon_name("document-save-symbolic");
        save_button.set_tooltip_text(Some("Save Desktop File"));
        header.pack_end(&save_button);
        
        let content = gtk::Box::new(gtk::Orientation::Vertical, 24);
        
        let basic_group = adw::PreferencesGroup::new();
        basic_group.set_title("Basic Information");
        
        let name_row = adw::ActionRow::new();
        name_row.set_title("Name");
        name_row.add_suffix(name_entry);
        basic_group.add(&name_row);
        
        let type_row = adw::ActionRow::new();
        type_row.set_title("Type");
        type_row.add_suffix(type_combo);
        basic_group.add(&type_row);
        
        let exec_row = adw::ActionRow::new();
        exec_row.set_title("Exec");
        exec_row.add_suffix(exec_entry);
        basic_group.add(&exec_row);
        
        let comment_row = adw::ActionRow::new();
        comment_row.set_title("Comment");
        comment_row.add_suffix(comment_entry);
        basic_group.add(&comment_row);
        
        let link_group = adw::PreferencesGroup::new();
        link_group.set_title("Link Settings");
        
        let url_row = adw::ActionRow::new();
        url_row.set_title("URL");
        url_row.add_suffix(url_entry);
        link_group.add(&url_row);
        
        let app_group = adw::PreferencesGroup::new();
        app_group.set_title("Application Settings");
        
        let icon_row = adw::ActionRow::new();
        icon_row.set_title("Icon");
        icon_row.add_suffix(icon_entry);
        app_group.add(&icon_row);
        
        let path_row = adw::ActionRow::new();
        path_row.set_title("Working Directory");
        path_row.add_suffix(path_entry);
        app_group.add(&path_row);
        
        let terminal_row = adw::ActionRow::new();
        terminal_row.set_title("Run in Terminal");
        terminal_row.add_suffix(terminal_switch);
        app_group.add(&terminal_row);
        
        let mime_row = adw::ActionRow::new();
        mime_row.set_title("MIME Types");
        mime_row.add_suffix(mime_type_entry);
        app_group.add(&mime_row);
        
        let cat_group = adw::PreferencesGroup::new();
        cat_group.set_title("Categories &amp; Keywords");
        
        let categories_row = adw::ActionRow::new();
        categories_row.set_title("Categories");
        
        let categories_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        categories_box.set_hexpand(true);
        
        let categories_visible_entry = gtk::Entry::new();
        categories_visible_entry.set_placeholder_text(Some("Categories (semicolon separated)"));
        categories_visible_entry.set_hexpand(true);
        categories_box.append(&categories_visible_entry);
        
        let categories_button = gtk::Button::with_label("Select");
        categories_button.set_valign(gtk::Align::Center);
        categories_box.append(&categories_button);
        
        categories_row.add_suffix(&categories_box);
        cat_group.add(&categories_row);
        
        let popover = _categories_popover.clone();
        categories_button.connect_clicked(move |btn| {
            popover.set_parent(btn);
            popover.popup();
        });
        
        let keywords_row = adw::ActionRow::new();
        keywords_row.set_title("Keywords");
        keywords_row.add_suffix(keywords_entry);
        cat_group.add(&keywords_row);
        
        let vis_group = adw::PreferencesGroup::new();
        vis_group.set_title("Visibility");
        
        let hidden_row = adw::ActionRow::new();
        hidden_row.set_title("Hidden");
        hidden_row.add_suffix(hidden_switch);
        vis_group.add(&hidden_row);
        
        content.append(&basic_group);
        content.append(&link_group);
        content.append(&app_group);
        content.append(&cat_group);
        content.append(&vis_group);
        
        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_child(Some(&content));
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        
        panel.append(&header);
        panel.append(&scrolled);
        
        (panel, save_button, delete_button, categories_visible_entry)
    }
    
    fn connect_signals(
        _window: &adw::ApplicationWindow,
        current_file: &Rc<RefCell<Option<DesktopFile>>>,
        file_path: &Rc<RefCell<Option<String>>>,
        updating_ui: &Rc<Cell<bool>>,
        file_list: &gtk::ListBox,
        name_entry: &gtk::Entry,
        exec_entry: &gtk::Entry,
        comment_entry: &gtk::Entry,
        icon_entry: &gtk::Entry,
        path_entry: &gtk::Entry,
        keywords_entry: &gtk::Entry,
        terminal_switch: &gtk::Switch,
        hidden_switch: &gtk::Switch,
        type_combo: &gtk::ComboBoxText,
        url_entry: &gtk::Entry,
        mime_type_entry: &gtk::Entry,
        new_button: &gtk::Button,
        save_button: &gtk::Button,
        delete_button: &gtk::Button,
        _categories_popover: &gtk::Popover,
        categories_checkboxes: &[gtk::CheckButton],
        _categories_custom_entry: &gtk::Entry,
        categories_visible_entry: &gtk::Entry,
        search_entry: &gtk::Entry,
    ) {
        {
            let current_file = current_file.clone();
            let file_path = file_path.clone();
            let updating_ui = updating_ui.clone();
            let name_entry = name_entry.clone();
            let exec_entry = exec_entry.clone();
            let comment_entry = comment_entry.clone();
            let icon_entry = icon_entry.clone();
            let path_entry = path_entry.clone();
            let keywords_entry = keywords_entry.clone();
            let terminal_switch = terminal_switch.clone();
            let hidden_switch = hidden_switch.clone();
            let type_combo = type_combo.clone();
            let url_entry = url_entry.clone();
            let mime_type_entry = mime_type_entry.clone();
            let categories_visible_entry = categories_visible_entry.clone();
            let categories_checkboxes = categories_checkboxes.to_vec();
            
            new_button.connect_clicked(move |_| {
                let new_file = DesktopFile::new("New Application".to_string(), "".to_string());
                *current_file.borrow_mut() = Some(new_file.clone());
                *file_path.borrow_mut() = None;
                updating_ui.set(true);
                Self::update_ui_fields(
                    &new_file, &name_entry, &exec_entry, &comment_entry, &icon_entry, &path_entry,
                    &keywords_entry, &terminal_switch, &hidden_switch,
                    &type_combo, &url_entry, &mime_type_entry, &categories_visible_entry, &categories_checkboxes
                );
                updating_ui.set(false);
            });
        }
        
        {
            let current_file = current_file.clone();
            let file_path = file_path.clone();
            let file_list = file_list.clone();
            
            save_button.connect_clicked(move |_| {
                let file_opt = current_file.borrow().clone();
                let path_opt = file_path.borrow().clone();
                // No RefCell borrow is held after this point
                if let Some(file) = file_opt {
                    println!("Saving file with categories: {:?}", file.desktop_entry.categories);
                    if let Some(path) = path_opt {
                        // Save to existing file
                        if let Err(e) = file.save(&path) {
                            eprintln!("Error saving file: {}", e);
                        } else {
                            println!("File saved successfully to: {}", path);
                            // Refresh the file list to show updated categories
                            Self::load_desktop_files(&file_list);
                        }
                    } else {
                        // Save as new file
                        if let Some(home) = env::var("HOME").ok() {
                            let user_apps = format!("{}/.local/share/applications", home);
                            let filename = format!("{}.desktop", file.desktop_entry.name.replace(" ", "-").to_lowercase());
                            let full_path = format!("{}/{}", user_apps, filename);
                            // Create directory if it doesn't exist
                            if let Err(e) = std::fs::create_dir_all(&user_apps) {
                                eprintln!("Error creating directory: {}", e);
                                return;
                            }
                            if let Err(e) = file.save(&full_path) {
                                eprintln!("Error saving file: {}", e);
                            } else {
                                println!("File saved successfully to: {}", full_path);
                                *file_path.borrow_mut() = Some(full_path);
                                // Refresh the file list to show the new file
                                Self::load_desktop_files(&file_list);
                            }
                        }
                    }
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let file_path = file_path.clone();
            let name_entry = name_entry.clone();
            let exec_entry = exec_entry.clone();
            let comment_entry = comment_entry.clone();
            let icon_entry = icon_entry.clone();
            let path_entry = path_entry.clone();
            let keywords_entry = keywords_entry.clone();
            let terminal_switch = terminal_switch.clone();
            let hidden_switch = hidden_switch.clone();
            let type_combo = type_combo.clone();
            let url_entry = url_entry.clone();
            let mime_type_entry = mime_type_entry.clone();
            let categories_visible_entry = categories_visible_entry.clone();
            let categories_checkboxes = categories_checkboxes.to_vec();
            let file_list = file_list.clone();
            let delete_button = delete_button.clone();
            
            file_list.connect_row_selected(move |_list, row| {
                if let Some(row) = row {
                    if row.has_css_class("category-header") {
                        let expanded = unsafe {
                            row.data::<bool>("expanded").map(|ptr| *ptr.as_ptr()).unwrap_or(true)
                        };
                        
                        let new_expanded = !expanded;
                        unsafe {
                            row.set_data("expanded", new_expanded);
                        }
                        
                        if let Some(child) = row.child() {
                            if let Some(box_widget) = child.downcast::<gtk::Box>().ok() {
                                if let Some(icon) = box_widget.first_child() {
                                    if let Some(icon_widget) = icon.downcast::<gtk::Image>().ok() {
                                        if new_expanded {
                                            icon_widget.set_from_icon_name(Some("pan-down-symbolic"));
                                        } else {
                                            icon_widget.set_from_icon_name(Some("pan-end-symbolic"));
                                        }
                                    }
                                }
                            }
                        }
                        
                        let mut next_row = row.next_sibling();
                        while let Some(sibling) = next_row {
                            if sibling.has_css_class("category-header") {
                                break; // Stop at next category
                            }
                            if sibling.has_css_class("file-item") {
                                sibling.set_visible(new_expanded);
                            }
                            next_row = sibling.next_sibling();
                        }
                        
                    }
                    else if row.has_css_class("file-item") {
                        let file_path_str = unsafe {
                            row.data::<String>("file_path").map(|ptr| (*ptr.as_ptr()).clone())
                        };
                        
                        if let Some(path) = file_path_str {
                            if let Some(file_name) = Path::new(&path).file_name() {
                                if let Some(name) = file_name.to_str() {
                                    Self::load_desktop_file(
                                        &current_file, &file_path, name,
                                        &name_entry, &exec_entry, &comment_entry, &icon_entry, &path_entry,
                                        &keywords_entry, &terminal_switch, &hidden_switch,
                                        &type_combo, &url_entry, &mime_type_entry, &categories_visible_entry, &categories_checkboxes
                                    );
                                    delete_button.set_visible(true);
                                }
                            }
                        }
                    }
                }
            });
        }
        
        // Delete button
        {
            let window = _window.clone();
            delete_button.connect_clicked(
                glib::clone!(@weak delete_button, @weak current_file, @weak file_path, @weak file_list, @weak window => move |_| {
                    let path_opt = file_path.borrow().clone();
                    if let Some(path) = path_opt {
                        let dialog = gtk::MessageDialog::builder()
                            .transient_for(&window)
                            .modal(true)
                            .message_type(gtk::MessageType::Warning)
                            .buttons(gtk::ButtonsType::OkCancel)
                            .text("Delete Desktop File?")
                            .secondary_text(&format!("Are you sure you want to delete this file?\n{}", path))
                            .build();
                        dialog.connect_response(glib::clone!(@weak delete_button, @weak current_file, @weak file_path, @weak file_list => move |dialog, response| {
                            if response == gtk::ResponseType::Ok {
                                if let Err(e) = std::fs::remove_file(&path) {
                                    eprintln!("Error deleting file: {}", e);
                                } else {
                                    *current_file.borrow_mut() = None;
                                    *file_path.borrow_mut() = None;
                                    delete_button.set_visible(false);
                                    DesktopFileManagerWindow::load_desktop_files(&file_list);
                                }
                            }
                            dialog.close();
                        }));
                        dialog.show();
                    }
                })
            );
        }
        
        // Type combo change
        {
            let exec_entry = exec_entry.clone();
            let url_entry = url_entry.clone();
            type_combo.connect_changed(move |combo| {
                if let Some(active_id) = combo.active_id() {
                    match active_id.as_str() {
                        "Application" => {
                            exec_entry.set_sensitive(true);
                            url_entry.set_sensitive(false);
                        },
                        "Link" => {
                            exec_entry.set_sensitive(false);
                            url_entry.set_sensitive(true);
                        },
                        "Directory" => {
                            exec_entry.set_sensitive(false);
                            url_entry.set_sensitive(false);
                        },
                        _ => {}
                    }
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            name_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.name = text.to_string();
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            exec_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.exec = Some(text.to_string());
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            comment_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.comment = Some(text.to_string());
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            icon_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.icon = Some(text.to_string());
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            path_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.path = Some(text.to_string());
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            keywords_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.keywords = Some(text.to_string());
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            terminal_switch.connect_state_set(move |_, state| {
                if updating_ui.get() { return Propagation::Proceed; }
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.terminal = Some(state);
                }
                Propagation::Proceed
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            hidden_switch.connect_state_set(move |_, state| {
                if updating_ui.get() { return Propagation::Proceed; }
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.hidden = Some(state);
                }
                Propagation::Proceed
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            url_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.url = Some(text.to_string());
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            mime_type_entry.connect_changed(move |entry| {
                if updating_ui.get() { return; }
                let text = entry.text();
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    file.desktop_entry.mime_type = Some(text.to_string());
                }
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            let categories_visible_entry = categories_visible_entry.clone();
            
            println!("Connecting categories visible entry signal handler for entry ID: {:?}", categories_visible_entry.as_ptr());
            categories_visible_entry.connect_changed(move |entry| {
                println!("Categories visible entry changed signal triggered for entry ID: {:?}", entry.as_ptr());
                println!("Categories visible entry changed signal triggered!");
                if updating_ui.get() { return; }
                let text = entry.text();
                println!("Categories changed to: '{}'", text);
                if let Some(ref mut file) = *current_file.borrow_mut() {
                    if text.is_empty() {
                        file.desktop_entry.categories = None;
                        println!("Categories cleared");
                    } else {
                        file.desktop_entry.categories = Some(text.to_string());
                        println!("Categories set to: '{}'", text);
                    }
                } else {
                    println!("No current file to update");
                }
            });
        }
        
        {
            let file_list = file_list.clone();
            let search_entry = search_entry.clone();
            
            search_entry.connect_changed(move |entry| {
                let search_text = entry.text().to_lowercase();
                Self::filter_file_list(&file_list, &search_text);
            });
        }
        
        {
            let current_file = current_file.clone();
            let updating_ui = updating_ui.clone();
            let categories_visible_entry = categories_visible_entry.clone();
            let categories_checkboxes = categories_checkboxes.to_vec();
            
            for (_i, checkbox) in categories_checkboxes.iter().enumerate() {
                let checkbox = checkbox.clone();
                let current_file = current_file.clone();
                let updating_ui = updating_ui.clone();
                let categories_visible_entry = categories_visible_entry.clone();
                let categories_checkboxes = categories_checkboxes.clone();
                
                checkbox.connect_toggled(move |_| {
                    if updating_ui.get() { return; }
                    
                    let mut selected_categories = Vec::new();
                    for (j, cb) in categories_checkboxes.iter().enumerate() {
                        if cb.is_active() {
                            selected_categories.push(COMMON_CATEGORIES[j]);
                        }
                    }
                    
                    let categories_text = selected_categories.join(";");
                    categories_visible_entry.set_text(&categories_text);
                    
                    if let Some(ref mut file) = *current_file.borrow_mut() {
                        if categories_text.is_empty() {
                            file.desktop_entry.categories = None;
                        } else {
                            file.desktop_entry.categories = Some(categories_text);
                        }
                    }
                });
            }
        }
    }
    
    fn load_desktop_files(list: &gtk::ListBox) {
        while let Some(child) = list.first_child() {
            list.remove(&child);
        }
        
        let paths = get_desktop_file_paths();
        let mut category_groups: HashMap<String, Vec<(String, String)>> = HashMap::new();
        
        for path in paths {
            if let Some(file_name) = Path::new(&path).file_name() {
                if let Some(name) = file_name.to_str() {
                    let categories = match DesktopFile::from_file(&path) {
                        Ok(file) => {
                            file.desktop_entry.categories
                                .as_ref()
                                .map(|cats| cats.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<_>>())
                                .unwrap_or_else(|| vec!["Uncategorized".to_string()])
                        },
                        Err(_) => vec!["Uncategorized".to_string()],
                    };
                    
                    for category in categories {
                        category_groups.entry(category.clone())
                            .or_insert_with(Vec::new)
                            .push((name.to_string(), path.clone()));
                    }
                }
            }
        }
        
        let mut sorted_categories: Vec<_> = category_groups.into_iter().collect();
        sorted_categories.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (category, files) in sorted_categories {
            // Create expander row for category
            let expander_row = gtk::ListBoxRow::new();
            let expander_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            expander_box.set_margin_start(12);
            expander_box.set_margin_end(12);
            expander_box.set_margin_top(6);
            expander_box.set_margin_bottom(6);
            
            let expand_icon = gtk::Image::from_icon_name("pan-end-symbolic");
            expand_icon.add_css_class("expand-icon");
            
            let category_label = gtk::Label::new(Some(&category));
            category_label.add_css_class("heading");
            category_label.set_halign(gtk::Align::Start);
            category_label.set_hexpand(true);
            
            let count_label = gtk::Label::new(Some(&format!("({})", files.len())));
            count_label.add_css_class("dim-label");
            
            expander_box.append(&expand_icon);
            expander_box.append(&category_label);
            expander_box.append(&count_label);
            expander_row.set_child(Some(&expander_box));
            expander_row.add_css_class("category-header");
            
            unsafe {
                expander_row.set_data("category_files", files.clone());
                expander_row.set_data("expanded", false);
            }
            
            list.append(&expander_row);
            
            for (file_name, file_path) in files {
                let file_row = gtk::ListBoxRow::new();
                let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                file_box.set_margin_start(24); // Indent files under category
                file_box.set_margin_end(12);
                file_box.set_margin_top(3);
                file_box.set_margin_bottom(3);
                
                let file_label = gtk::Label::new(Some(&file_name));
                file_label.set_halign(gtk::Align::Start);
                file_label.set_hexpand(true);
                
                file_box.append(&file_label);
                file_row.set_child(Some(&file_box));
                file_row.add_css_class("file-item");
                
                unsafe {
                    file_row.set_data("file_path", file_path);
                }
                
                file_row.set_visible(false); // Initially hidden
                list.append(&file_row);
            }
        }
    }
    
    fn load_desktop_file(
        current_file: &Rc<RefCell<Option<DesktopFile>>>,
        file_path: &Rc<RefCell<Option<String>>>,
        file_name: &str,
        name_entry: &gtk::Entry,
        exec_entry: &gtk::Entry,
        comment_entry: &gtk::Entry,
        icon_entry: &gtk::Entry,
        path_entry: &gtk::Entry,
        keywords_entry: &gtk::Entry,
        terminal_switch: &gtk::Switch,
        hidden_switch: &gtk::Switch,
        type_combo: &gtk::ComboBoxText,
        url_entry: &gtk::Entry,
        mime_type_entry: &gtk::Entry,
        categories_visible_entry: &gtk::Entry,
        categories_checkboxes: &[gtk::CheckButton],
    ) {
        let paths = get_desktop_file_paths();
        for path in paths {
            if let Some(path_file_name) = Path::new(&path).file_name() {
                if let Some(name) = path_file_name.to_str() {
                    if name == file_name {
                        match DesktopFile::from_file(&path) {
                            Ok(file) => {
                                *current_file.borrow_mut() = Some(file.clone());
                                *file_path.borrow_mut() = Some(path);
                                
                                Self::update_ui_fields(
                                    &file, name_entry, exec_entry, comment_entry, icon_entry, path_entry,
                                    keywords_entry, terminal_switch, hidden_switch,
                                    type_combo, url_entry, mime_type_entry, categories_visible_entry, &categories_checkboxes
                                );
                            },
                            Err(e) => {
                                eprintln!("Error loading desktop file: {}", e);
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    
    fn update_ui_fields(
        file: &DesktopFile,
        name_entry: &gtk::Entry,
        exec_entry: &gtk::Entry,
        comment_entry: &gtk::Entry,
        icon_entry: &gtk::Entry,
        path_entry: &gtk::Entry,
        keywords_entry: &gtk::Entry,
        terminal_switch: &gtk::Switch,
        hidden_switch: &gtk::Switch,
        type_combo: &gtk::ComboBoxText,
        url_entry: &gtk::Entry,
        mime_type_entry: &gtk::Entry,
        categories_visible_entry: &gtk::Entry,
        categories_checkboxes: &[gtk::CheckButton],
    ) {
        name_entry.set_text(&file.desktop_entry.name);
        
        if let Some(ref exec) = file.desktop_entry.exec {
            exec_entry.set_text(exec);
        } else {
            exec_entry.set_text("");
        }
        
        if let Some(ref comment) = file.desktop_entry.comment {
            comment_entry.set_text(comment);
        } else {
            comment_entry.set_text("");
        }
        
        if let Some(ref icon) = file.desktop_entry.icon {
            icon_entry.set_text(icon);
        } else {
            icon_entry.set_text("");
        }
        
        if let Some(ref path) = file.desktop_entry.path {
            path_entry.set_text(path);
        } else {
            path_entry.set_text("");
        }
        
        if let Some(ref keywords) = file.desktop_entry.keywords {
            keywords_entry.set_text(keywords);
        } else {
            keywords_entry.set_text("");
        }
        
        if let Some(terminal) = file.desktop_entry.terminal {
            terminal_switch.set_active(terminal);
        } else {
            terminal_switch.set_active(false);
        }
        
        if let Some(hidden) = file.desktop_entry.hidden {
            hidden_switch.set_active(hidden);
        } else {
            hidden_switch.set_active(false);
        }
        
        // Set type combo
        match file.desktop_entry.entry_type.as_str() {
            "Application" => type_combo.set_active(Some(0)),
            "Link" => type_combo.set_active(Some(1)),
            "Directory" => type_combo.set_active(Some(2)),
            _ => type_combo.set_active(Some(0)),
        }
        
        if let Some(ref url) = file.desktop_entry.url {
            url_entry.set_text(url);
        } else {
            url_entry.set_text("");
        }
        
        if let Some(ref mime_type) = file.desktop_entry.mime_type {
            mime_type_entry.set_text(mime_type);
        } else {
            mime_type_entry.set_text("");
        }
        
        if let Some(ref categories) = file.desktop_entry.categories {
            categories_visible_entry.set_text(categories);
            
            let category_list: Vec<&str> = categories.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            for (i, checkbox) in categories_checkboxes.iter().enumerate() {
                let is_selected = category_list.contains(&COMMON_CATEGORIES[i]);
                checkbox.set_active(is_selected);
            }
        } else {
            categories_visible_entry.set_text("");
            
            for checkbox in categories_checkboxes {
                checkbox.set_active(false);
            }
        }
    }
    
    fn filter_file_list(list: &gtk::ListBox, search_text: &str) {
        let mut row = list.first_child();
        while let Some(child) = row {
            let next_sibling = child.next_sibling();
            if let Some(list_row) = child.downcast::<gtk::ListBoxRow>().ok() {
                if list_row.has_css_class("file-item") {
                    // For file items, check if the filename matches
                    if let Some(child) = list_row.child() {
                        if let Some(box_widget) = child.downcast::<gtk::Box>().ok() {
                            if let Some(label) = box_widget.first_child() {
                                if let Some(label_widget) = label.downcast::<gtk::Label>().ok() {
                                    let file_name = label_widget.text().to_lowercase();
                                    let should_show = search_text.is_empty() || file_name.contains(search_text);
                                    list_row.set_visible(should_show);
                                }
                            }
                        }
                    }
                } else if list_row.has_css_class("category-header") {
                    // For category headers, always show them for now
                    // TODO: Implement proper category filtering
                    list_row.set_visible(true);
                }
            }
            row = next_sibling;
        }
    }
    
    pub fn show(&self) {
        self.window.show();
    }
} 