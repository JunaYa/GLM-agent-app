use tauri::{utils::config::WindowEffectsConfig, AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use crate::constants::TASK_WINDOW;

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