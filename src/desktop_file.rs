use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum DesktopFileError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid desktop file format: {0}")]
    #[allow(dead_code)]
    ParseError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid value for field {0}: {1}")]
    #[allow(dead_code)]
    InvalidValue(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopFile {
    pub desktop_entry: DesktopEntry,
    pub icon_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopEntry {
    #[serde(rename = "Type")]
    pub entry_type: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "GenericName")]
    pub generic_name: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Icon")]
    pub icon: Option<String>,
    #[serde(rename = "Exec")]
    pub exec: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Terminal")]
    pub terminal: Option<bool>,
    #[serde(rename = "Categories")]
    pub categories: Option<String>,
    #[serde(rename = "Keywords")]
    pub keywords: Option<String>,
    #[serde(rename = "StartupWMClass")]
    pub startup_wm_class: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "MimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "Hidden")]
    pub hidden: Option<bool>,
    #[serde(rename = "OnlyShowIn")]
    pub only_show_in: Option<String>,
    #[serde(rename = "NotShowIn")]
    pub not_show_in: Option<String>,
    #[serde(rename = "DBusActivatable")]
    pub dbus_activatable: Option<bool>,
    #[serde(rename = "TryExec")]
    pub try_exec: Option<String>,
    #[serde(rename = "Actions")]
    pub actions: Option<String>,
}

impl DesktopFile {
    pub fn new(name: String, exec: String) -> Self {
        Self {
            desktop_entry: DesktopEntry {
                entry_type: "Application".to_string(),
                version: Some("1.0".to_string()),
                name,
                generic_name: None,
                comment: None,
                icon: None,
                exec: Some(exec),
                path: None,
                terminal: Some(false),
                categories: None,
                keywords: None,
                startup_wm_class: None,
                url: None,
                mime_type: None,
                hidden: Some(false),
                only_show_in: None,
                not_show_in: None,
                dbus_activatable: Some(false),
                try_exec: None,
                actions: None,
            },
            icon_data: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, DesktopFileError> {
        let content = fs::read_to_string(path)?;
        Self::from_string(&content)
    }

    pub fn from_string(content: &str) -> Result<Self, DesktopFileError> {
        let lines = content.lines();
        let mut current_section = None;
        let mut desktop_entry = HashMap::new();

        for line in lines {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = Some(line[1..line.len() - 1].to_string());
                continue;
            }

            if let Some(section) = &current_section {
                if let Some(pos) = line.find('=') {
                    let key = line[..pos].trim();
                    let value = line[pos + 1..].trim();

                    if section == "Desktop Entry" {
                        desktop_entry.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        let entry = DesktopEntry {
            entry_type: desktop_entry
                .get("Type")
                .cloned()
                .unwrap_or_else(|| "Application".to_string()),
            version: desktop_entry.get("Version").cloned(),
            name: desktop_entry
                .get("Name")
                .cloned()
                .ok_or_else(|| DesktopFileError::MissingField("Name".to_string()))?,
            generic_name: desktop_entry.get("GenericName").cloned(),
            comment: desktop_entry.get("Comment").cloned(),
            icon: desktop_entry.get("Icon").cloned(),
            exec: desktop_entry.get("Exec").cloned(),
            path: desktop_entry.get("Path").cloned(),
            terminal: desktop_entry.get("Terminal").map(|v| v == "true"),
            categories: desktop_entry.get("Categories").cloned(),
            keywords: desktop_entry.get("Keywords").cloned(),
            startup_wm_class: desktop_entry.get("StartupWMClass").cloned(),
            url: desktop_entry.get("URL").cloned(),
            mime_type: desktop_entry.get("MimeType").cloned(),
            hidden: desktop_entry.get("Hidden").map(|v| v == "true"),
            only_show_in: desktop_entry.get("OnlyShowIn").cloned(),
            not_show_in: desktop_entry.get("NotShowIn").cloned(),
            dbus_activatable: desktop_entry.get("DBusActivatable").map(|v| v == "true"),
            try_exec: desktop_entry.get("TryExec").cloned(),
            actions: desktop_entry.get("Actions").cloned(),
        };

        Ok(Self {
            desktop_entry: entry,
            icon_data: None,
        })
    }

    pub fn to_string(&self) -> String {
        let mut content = String::new();
        content.push_str("[Desktop Entry]\n");

        content.push_str(&format!("Type={}\n", self.desktop_entry.entry_type));
        content.push_str(&format!("Name={}\n", self.desktop_entry.name));

        if let Some(ref version) = self.desktop_entry.version {
            content.push_str(&format!("Version={}\n", version));
        }
        if let Some(ref generic_name) = self.desktop_entry.generic_name {
            content.push_str(&format!("GenericName={}\n", generic_name));
        }
        if let Some(ref comment) = self.desktop_entry.comment {
            content.push_str(&format!("Comment={}\n", comment));
        }
        if let Some(ref icon) = self.desktop_entry.icon {
            content.push_str(&format!("Icon={}\n", icon));
        }
        if let Some(ref exec) = self.desktop_entry.exec {
            content.push_str(&format!("Exec={}\n", exec));
        }
        if let Some(ref path) = self.desktop_entry.path {
            content.push_str(&format!("Path={}\n", path));
        }
        if let Some(terminal) = self.desktop_entry.terminal {
            content.push_str(&format!(
                "Terminal={}\n",
                if terminal { "true" } else { "false" }
            ));
        }
        if let Some(ref categories) = self.desktop_entry.categories {
            content.push_str(&format!("Categories={}\n", categories));
        }
        if let Some(ref keywords) = self.desktop_entry.keywords {
            content.push_str(&format!("Keywords={}\n", keywords));
        }
        if let Some(ref startup_wm_class) = self.desktop_entry.startup_wm_class {
            content.push_str(&format!("StartupWMClass={}\n", startup_wm_class));
        }
        if let Some(ref url) = self.desktop_entry.url {
            content.push_str(&format!("URL={}\n", url));
        }
        if let Some(ref mime_type) = self.desktop_entry.mime_type {
            content.push_str(&format!("MimeType={}\n", mime_type));
        }
        if let Some(hidden) = self.desktop_entry.hidden {
            content.push_str(&format!(
                "Hidden={}\n",
                if hidden { "true" } else { "false" }
            ));
        }
        if let Some(ref only_show_in) = self.desktop_entry.only_show_in {
            content.push_str(&format!("OnlyShowIn={}\n", only_show_in));
        }
        if let Some(ref not_show_in) = self.desktop_entry.not_show_in {
            content.push_str(&format!("NotShowIn={}\n", not_show_in));
        }
        if let Some(dbus_activatable) = self.desktop_entry.dbus_activatable {
            content.push_str(&format!(
                "DBusActivatable={}\n",
                if dbus_activatable { "true" } else { "false" }
            ));
        }
        if let Some(ref try_exec) = self.desktop_entry.try_exec {
            content.push_str(&format!("TryExec={}\n", try_exec));
        }
        if let Some(ref actions) = self.desktop_entry.actions {
            content.push_str(&format!("Actions={}\n", actions));
        }

        content
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), DesktopFileError> {
        let content = self.to_string();
        fs::write(path, content)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), DesktopFileError> {
        if self.desktop_entry.name.is_empty() {
            return Err(DesktopFileError::MissingField("Name".to_string()));
        }

        match self.desktop_entry.entry_type.as_str() {
            "Application" | "Link" | "Directory" => {}
            _ => {
                return Err(DesktopFileError::InvalidValue(
                    "Type".to_string(),
                    self.desktop_entry.entry_type.clone(),
                ))
            }
        }

        if self.desktop_entry.entry_type == "Application" && self.desktop_entry.exec.is_none() {
            return Err(DesktopFileError::MissingField("Exec".to_string()));
        }

        if self.desktop_entry.entry_type == "Link" && self.desktop_entry.url.is_none() {
            return Err(DesktopFileError::MissingField("URL".to_string()));
        }

        Ok(())
    }
}

pub fn get_desktop_file_paths() -> Vec<String> {
    let mut paths = Vec::new();

    if let Ok(entries) = fs::read_dir("/usr/share/applications") {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "desktop" {
                    paths.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        let user_apps = format!("{}/.local/share/applications", home);
        if let Ok(entries) = fs::read_dir(user_apps) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "desktop" {
                        paths.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    paths
}
