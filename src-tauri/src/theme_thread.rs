use std::{
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tauri::{AppHandle, Emitter};

fn get_system_theme() -> String {
    let color_scheme = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("color-scheme")
        .output();

    if let Ok(out) = color_scheme {
        let color_scheme_setting = String::from_utf8_lossy(&out.stdout).to_lowercase();
        if color_scheme_setting.contains("dark") {
            return "dark".to_string();
        }
    }

    return "light".to_string();
}

pub fn update_theme(app_handle: AppHandle) {
    app_handle
        .emit("theme_changed", get_system_theme())
        .unwrap();
}

pub fn run(app_handle: AppHandle) {
    thread::spawn(move || {
        let last_theme = Arc::new(Mutex::new(String::new()));
        loop {
            let current_theme = get_system_theme();
            let mut last = last_theme.lock().unwrap();
            if *last != current_theme {
                *last = current_theme.clone();
                app_handle
                    .emit("theme_changed", current_theme.clone())
                    .unwrap();
            }
            thread::sleep(Duration::from_secs(1)); // poll interval
        }
    });
}
