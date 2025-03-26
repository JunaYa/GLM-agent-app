use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_nspanel::ManagerExt;

use crate::panel;

const HOME_WINDOW: &str = "home";
pub fn get_home_window(app: &AppHandle) -> WebviewWindow {
    if let Some(window) = app.get_webview_window(HOME_WINDOW) {
        window
    } else {
        let window =
            WebviewWindowBuilder::new(app, HOME_WINDOW, WebviewUrl::App("/home".into()))
                .title("GLM")
                .decorations(false)
                .visible(true)
                .skip_taskbar(true)
                .shadow(false)
                .resizable(false)
                .inner_size(80.0, 80.0);

        let window = window.build().expect("Unable to build startup window");
        #[cfg(target_os = "macos")]
        {
            use cocoa::appkit::{NSColor, NSWindow};
            use cocoa::base::{id, nil};

            let ns_window = window.ns_window().unwrap() as id;
            unsafe {
                // macOS: Handle multiple spaces correctly
                ns_window.setCollectionBehavior_(cocoa::appkit::NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);

                let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                    nil,
                    33.0 / 255.0,
                    54.0 / 255.0,
                    201.0 / 255.0,
                    0.0,
                );
                ns_window.setBackgroundColor_(bg_color);
            }
        }

        panel::init(app, window.clone());

        window
    }
}

pub fn show_home_window(app: &AppHandle) {
    let window = get_home_window(app);
    let panel = app.get_webview_panel(HOME_WINDOW).unwrap();
    panel.show();
}

pub fn hide_home_window(app: &AppHandle) {
    let panel = app.get_webview_panel(HOME_WINDOW).unwrap();
    panel.order_out(None);
}

fn close_home_window(handle: AppHandle) {
  let panel = handle.get_webview_panel(HOME_WINDOW).unwrap();

  panel.set_released_when_closed(true);

  panel.close();
}