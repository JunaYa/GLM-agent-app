use tauri::AppHandle;
use tauri_nspanel::ManagerExt;

const HOME_PANEL: &str = "home";
#[tauri::command]
pub fn show_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(HOME_PANEL).unwrap();

    panel.show();
}

#[tauri::command]
pub fn hide_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(HOME_PANEL).unwrap();

    panel.order_out(None);
}

#[tauri::command]
pub fn close_panel(handle: AppHandle) {
    let panel = handle.get_webview_panel(HOME_PANEL).unwrap();

    panel.set_released_when_closed(true);

    panel.close();
}

#[tauri::command]
pub fn start_task(handle: AppHandle) {
    let panel = handle.get_webview_panel(HOME_PANEL).unwrap();

    panel.show();
}
