use tauri::{utils::config::WindowEffectsConfig, AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use crate::constants::HOME_WINDOW;

pub fn get_home_window(app: &AppHandle) -> WebviewWindow {
    if let Some(window) = app.get_webview_window(HOME_WINDOW) {
        window
    } else {
        let window =
            WebviewWindowBuilder::new(app, HOME_WINDOW, WebviewUrl::App("/home".into()))
                .title("GLM")
                .decorations(false)
                .visible(false)
                .transparent(true)
                .skip_taskbar(true)
                .shadow(false)
                .resizable(false)
                .effects(WindowEffectsConfig {
                    effects: vec![],
                    radius: Some(80.0),
                    ..Default::default()
                })
                .inner_size(80.0, 80.0);

        let window = window.build().expect("Unable to build startup window");

        window
    }
}

pub fn show_home_window(app: &AppHandle) {
    let window = get_home_window(app);
    window.show();
}

pub fn hide_home_window(app: &AppHandle) {
    let window = get_home_window(app);
    window.hide();
}

pub fn close_home_window(handle: AppHandle) {
  let window = handle.get_webview_window(HOME_WINDOW).unwrap();
  window.close();
}
