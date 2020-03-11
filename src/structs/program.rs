use winapi::shared::windef::{HMONITOR, HWND};
use winapi::um::winuser::{MONITORINFO, WINDOWPLACEMENT};

pub struct Program {
  pub name: String,
  pub window: HWND,
  pub placement: WINDOWPLACEMENT,
  pub monitor: HMONITOR,
  pub monitor_info: MONITORINFO,
}
