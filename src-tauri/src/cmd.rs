use tauri::AppHandle;
use tauri_nspanel::ManagerExt;

use crate::constants::{HOME_WINDOW, TASK_WINDOW};

#[tauri::command]
pub fn show_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(HOME_WINDOW).unwrap();

    panel.show();
}

#[tauri::command]
pub fn hide_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(HOME_WINDOW).unwrap();

    panel.order_out(None);
}

#[tauri::command]
pub fn close_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(HOME_WINDOW).unwrap();

    panel.set_released_when_closed(true);

    panel.close();
}

// task panel management

#[tauri::command]
pub fn show_task_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(TASK_WINDOW).unwrap();
    panel.show();
}

#[tauri::command]
pub fn hide_task_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(TASK_WINDOW).unwrap();
    panel.order_out(None);

}


#[tauri::command]
pub fn close_task_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(TASK_WINDOW).unwrap();

    panel.set_released_when_closed(true);

    panel.close();
}


