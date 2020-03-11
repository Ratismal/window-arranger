use std::ptr;
use winapi::shared::minwindef::{BOOL, LPARAM};
use winapi::shared::windef::{HDC, HMONITOR, HWND, LPRECT, RECT};
use winapi::um::winuser::{
  EnumDisplayMonitors, EnumWindows, GetAncestor, GetLastActivePopup, GetMonitorInfoW, GetParent,
  GetShellWindow, GetTitleBarInfo, GetWindowLongW, GetWindowPlacement, GetWindowRect,
  GetWindowTextW, IsIconic, IsWindow, IsWindowVisible, MonitorFromWindow, SetWindowPlacement,
  ShowWindowAsync, GA_ROOTOWNER, GWL_EXSTYLE, GWL_STYLE, MONITORENUMPROC, MONITORINFO,
  MONITOR_DEFAULTTONEAREST, STATE_SYSTEM_INVISIBLE, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED,
  SW_SHOWMINNOACTIVE, SW_SHOWNOACTIVATE, WINDOWPLACEMENT, WNDENUMPROC, WPF_ASYNCWINDOWPLACEMENT,
  WS_CHILD, WS_EX_TOOLWINDOW, WS_POPUP,
};

use crate::structs::Program;

pub mod def;

unsafe extern "system" fn monitor_enum_proc(
  _: HMONITOR,
  _: HDC,
  rect: LPRECT,
  l_param: LPARAM,
) -> BOOL {
  let vec = &mut *(l_param as *mut Vec<RECT>);
  vec.push(*rect);
  1
}

pub fn get_all_monitors() -> Vec<RECT> {
  let mut vec: Vec<RECT> = Vec::new();
  unsafe {
    // let cproc: CMONITORENUMPROC = monitor_enum_proc;
    let proc: MONITORENUMPROC = Some(monitor_enum_proc);
    EnumDisplayMonitors(
      ptr::null_mut(),
      ptr::null_mut(),
      proc,
      &mut vec as *mut Vec<RECT> as LPARAM,
    );
  }
  vec
}

unsafe extern "system" fn window_enum_proc(hwnd: HWND, l_param: LPARAM) -> BOOL {
  let vec = &mut *(l_param as *mut Vec<HWND>);
  if is_application_window(hwnd) {
    vec.push(hwnd);
  }
  1
}

pub fn get_all_windows() -> Vec<HWND> {
  let mut vec: Vec<HWND> = Vec::new();

  unsafe {
    let proc: WNDENUMPROC = Some(window_enum_proc);
    EnumWindows(proc, &mut vec as *mut Vec<HWND> as LPARAM);
  }

  vec
}

pub fn get_window_text(hwnd: HWND) -> String {
  let size = 255;
  let mut name = Vec::with_capacity(size as usize);
  unsafe {
    let read_len = GetWindowTextW(hwnd, name.as_mut_ptr(), size);
    name.set_len(read_len as usize);
    String::from_utf16_lossy(&name)
  }
  // String::from_utf16(&name).unwrap()
}

pub fn get_window_placement(hwnd: HWND, placement: &mut WINDOWPLACEMENT) -> bool {
  // let mut placement: WINDOWPLACEMENT = get_default_window_placement();

  unsafe {
    if GetWindowPlacement(hwnd, placement) == 1 {
      GetWindowRect(hwnd, &mut placement.rcNormalPosition);

      let monitor = get_window_monitor(hwnd);
      let monitor_info = get_monitor_info(monitor);
      let dx = monitor_info.rcMonitor.left - monitor_info.rcWork.left;
      let dy = monitor_info.rcMonitor.top - monitor_info.rcWork.top;

      placement.rcNormalPosition.left += dx;
      placement.rcNormalPosition.right += dx;
      placement.rcNormalPosition.top += dy;
      placement.rcNormalPosition.bottom += dy;
    }
  }

  true
}

pub fn restore_placement(hwnd: HWND, p: WINDOWPLACEMENT) {
  let mut current = def::window_placement();
  get_window_placement(hwnd, &mut current);

  let name = get_window_text(hwnd);
  println!("Restoring window {:?}", name);

  let mut placement = def::clone_window_placement(p);

  unsafe {
    let cmd = current.showCmd;
    if cmd == SW_SHOWMINIMIZED as u32 {
      placement.showCmd = SW_SHOWMINNOACTIVE as u32;
      placement.flags |= WPF_ASYNCWINDOWPLACEMENT;
      SetWindowPlacement(hwnd, &placement);
    } else if cmd == SW_SHOWMAXIMIZED as u32 {
      // Restore
      ShowWindowAsync(hwnd, SW_SHOWNOACTIVATE);

      // Move
      placement.showCmd = SW_SHOWNOACTIVATE as u32;
      placement.flags |= WPF_ASYNCWINDOWPLACEMENT;
      SetWindowPlacement(hwnd, &placement);

      // Maximize
      ShowWindowAsync(hwnd, SW_SHOWMAXIMIZED);
    } else {
      placement.showCmd = SW_SHOWNOACTIVATE as u32;
      placement.flags |= WPF_ASYNCWINDOWPLACEMENT;
      SetWindowPlacement(hwnd, &placement);
    }
  }
}

pub fn is_application_window(hwnd: HWND) -> bool {
  unsafe {
    if IsWindow(hwnd) == 0 {
      return false;
    }

    if hwnd == GetShellWindow() {
      return false;
    }

    if IsWindowVisible(hwnd) == 0 {
      return false;
    }

    if IsIconic(hwnd) == 1 {
      return false;
    }

    let mut hwnd_walk: HWND = GetAncestor(hwnd, GA_ROOTOWNER);
    let mut hwnd_try = GetLastActivePopup(hwnd);
    while hwnd_try != hwnd_walk {
      if IsWindowVisible(hwnd_try) == 1 {
        break;
      }

      hwnd_walk = hwnd_try;
      hwnd_try = GetLastActivePopup(hwnd_walk);
      // println!("{:?} {:?} {:?} {:?}", hwnd, hwnd_try, hwnd_walk, name);
    }
    if hwnd_try != hwnd {
      return false;
    }

    let mut ti = def::titlebar_info();
    GetTitleBarInfo(hwnd, &mut ti);
    if (ti.rgstate[0] & STATE_SYSTEM_INVISIBLE) > 0 {
      return false;
    }

    let exlong = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
    let long = GetWindowLongW(hwnd, GWL_STYLE) as u32;

    if (exlong & WS_EX_TOOLWINDOW) > 0 {
      return false;
    }
    if (long & WS_POPUP) > 0 {
      return false;
    }
    if (long & WS_CHILD) > 0 {
      return false;
    }

    let parent = GetParent(hwnd);
    if IsWindow(parent) == 1 {
      return false;
    }

    // println!("{:#032b} {:#032b} {:?}", exlong, long, name);

    // if (exlong & 0x200000) > 0 {
    //   return false;
    // }

    return true;
  }
}

pub fn get_window_monitor(hwdn: HWND) -> HMONITOR {
  unsafe { MonitorFromWindow(hwdn, MONITOR_DEFAULTTONEAREST) }
}

pub fn get_monitor_info(hmonitor: HMONITOR) -> MONITORINFO {
  let mut info = def::monitor_info();

  unsafe {
    GetMonitorInfoW(hmonitor, &mut info);
  }

  info
}

pub fn get_programs() -> Vec<Program> {
  let mut programs: Vec<Program> = Vec::new();

  let windows = get_all_windows();
  for window in windows {
    let name = get_window_text(window);

    let mut placement = def::window_placement();
    get_window_placement(window, &mut placement);

    let monitor = get_window_monitor(window);
    let monitor_info = get_monitor_info(monitor);

    let program = Program {
      name: name,
      window: window,
      placement: placement,
      monitor: monitor,
      monitor_info: monitor_info,
    };

    programs.push(program);
  }

  programs
}
