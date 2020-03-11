use winapi::shared::windef::{POINT, RECT};
use winapi::um::winuser::{MONITORINFO, TITLEBARINFO, WINDOWPLACEMENT};

pub fn monitor_info() -> MONITORINFO {
  MONITORINFO {
    cbSize: 0,
    rcMonitor: rect(),
    rcWork: rect(),
    dwFlags: 0,
  }
}

pub fn point() -> POINT {
  POINT { x: 0, y: 0 }
}

pub fn rect() -> RECT {
  RECT {
    top: 0,
    bottom: 0,
    left: 0,
    right: 0,
  }
}

pub fn window_placement() -> WINDOWPLACEMENT {
  WINDOWPLACEMENT {
    length: 0,
    flags: 0,
    showCmd: 0,
    ptMinPosition: point(),
    ptMaxPosition: point(),
    rcNormalPosition: rect(),
  }
}

pub fn clone_window_placement(p: WINDOWPLACEMENT) -> WINDOWPLACEMENT {
  WINDOWPLACEMENT {
    length: p.length,
    flags: p.flags,
    showCmd: p.showCmd,
    ptMinPosition: p.ptMinPosition,
    ptMaxPosition: p.ptMaxPosition,
    rcNormalPosition: p.rcNormalPosition,
  }
}

pub fn titlebar_info() -> TITLEBARINFO {
  TITLEBARINFO {
    cbSize: 0,
    rcTitleBar: rect(),
    rgstate: [0, 0, 0, 0, 0, 0],
  }
}
