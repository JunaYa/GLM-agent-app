mod window_home;
mod window_task;

use tauri::{Monitor, WebviewWindow};
pub use window_home::*;
pub use window_task::*;

pub fn find_monitor(window: &WebviewWindow) -> Option<Monitor> {
  if let Ok(Some(mon)) = window.primary_monitor() {
      Some(mon)
  } else if let Ok(Some(mon)) = window.current_monitor() {
      Some(mon)
  } else if let Ok(mut monitors) = window.available_monitors() {
      if monitors.is_empty() {
          None
      } else {
          monitors.pop()
      }
  } else {
      None
  }
}