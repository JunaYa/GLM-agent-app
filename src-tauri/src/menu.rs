use std::str::FromStr;

use strum_macros::{Display, EnumString};
use tauri::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle,
};
use tracing::info;

use crate::panel;

#[derive(Debug, Display, EnumString)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
enum MenuID {
    START_TASK,
    STOP_TASK,
    EXIT,
}

pub fn create_tray(app: &mut tauri::App) -> Result<(), tauri::Error> {
    let _ = TrayIconBuilder::with_id("main-tray")
        .menu(&get_tray_menu(app.handle())?)
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .show_menu_on_left_click(true)
        .on_menu_event(handle_tray_menu_events)
        .on_tray_icon_event(handle_tray_icon_events)
        .build(app)?;
    Ok(())
}

pub fn get_tray_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let tray = Menu::with_id(app, "tray_menu")?;

    tray.append_items(&[
        &MenuItem::with_id(
            app,
            MenuID::START_TASK.to_string(),
            "Start Task",
            true,
            None::<&str>,
        )?,
        &MenuItem::with_id(
            app,
            MenuID::STOP_TASK.to_string(),
            "Stop Task",
            true,
            None::<&str>,
        )?,
        &MenuItem::with_id(app, MenuID::EXIT.to_string(), "Exit", true, None::<&str>)?,
    ])?;

    Ok(tray)
}

pub fn get_app_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let app_menu = Menu::new(app)?;
    let menu = Submenu::with_items(
        app,
        "GLM",
        true,
        &[
            &PredefinedMenuItem::about(app, Some("about"), Default::default())?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::show_all(app, None)?,
            &PredefinedMenuItem::quit(app, None)?,
        ],
    )?;
    app_menu.append(&menu)?;

    // edit menu
    let edit_menu = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::select_all(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
        ],
    )?;
    app_menu.append(&edit_menu)?;

    Ok(app_menu)
}

fn handle_tray_icon_events(_tray: &TrayIcon, event: TrayIconEvent) {
    tauri_plugin_positioner::on_tray_event(_tray.app_handle(), &event);
    if let TrayIconEvent::DoubleClick { .. } = event {
        info!("Double click");
    }
}

fn handle_tray_menu_events(app: &AppHandle, event: MenuEvent) {
    let menu_id = if let Ok(menu_id) = MenuID::from_str(event.id.as_ref()) {
        menu_id
    } else {
        return;
    };

    match menu_id {
        MenuID::START_TASK => {
            info!("Start Task");
            panel::show_task_panel(&app);
        }
        MenuID::STOP_TASK => {
            info!("Stop Task");
            panel::hide_task_panel(&app);
        }
        MenuID::EXIT => {
            info!("Exit");
            app.exit(0)
        }
    }
}
