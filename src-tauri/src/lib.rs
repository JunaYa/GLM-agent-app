use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_nspanel::{cocoa::appkit::NSWindowCollectionBehavior, panel_delegate, WebviewWindowExt};

mod cmd;
mod menu;
mod window;
mod panel;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_nspanel::init())
        .invoke_handler(tauri::generate_handler![
            cmd::show_panel,
            cmd::hide_panel,
            cmd::close_panel,
            cmd::start_task,
        ])
        .setup(|app| {
            // Set activation poicy to Accessory to prevent the app icon from showing on the dock
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            menu::create_tray(app)?;

            Ok(())
        })
        .menu(menu::get_app_menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
