use std::{thread::sleep, time::Duration};

use tauri::{AppHandle, Manager, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};
use crate::constants::TASK_WINDOW;

use super::find_monitor;

pub fn get_task_window(app: &AppHandle) -> WebviewWindow {
    if let Some(window) = app.get_webview_window(TASK_WINDOW) {
        window
    } else {
        let window =
            WebviewWindowBuilder::new(app, TASK_WINDOW, WebviewUrl::App("/task".into()))
                .title("GLM")
                .decorations(false)
                .visible(false)
                .transparent(true)
                .skip_taskbar(true)
                .shadow(false)
                .resizable(false)
                .inner_size(800.0, 800.0);

        let window = window.build().expect("Unable to build startup window");

        if let Some(monitor) = find_monitor(&window) {
            let screen_size = monitor.size();
            let size = PhysicalSize {
                width: screen_size.width,
                height: screen_size.height,
            };
            println!("size: {:?}", size);
            let _ = window.set_size(tauri::Size::Physical(size));
            // sleep 0.3
            let window = window.clone();
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_millis(100));
                let _ = window.move_window(Position::Center);
            });
        }

        window
    }
}

pub fn show_task_window(app: &AppHandle) {
    let window = get_task_window(app);
    window.show();
}

pub fn hide_task_window(app: &AppHandle) {
    let window = get_task_window(app);
    window.hide();
}

pub fn close_task_window(handle: AppHandle) {
  let window = handle.get_webview_window(TASK_WINDOW).unwrap();
  window.close();
}