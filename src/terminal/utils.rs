use std::io::Write;

/// 获取终端大小。返回宽和高，单位为字符数
pub fn terminal_size() -> Option<(u16, u16)> {
    let fd = libc::STDOUT_FILENO;

    let is_tty: bool = unsafe { libc::isatty(fd) == 1 };
    if !is_tty {
        return None;
    }

    let mut ws = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) } == -1 {
        return None;
    }

    let rows = ws.ws_row;
    let cols = ws.ws_col;

    if rows > 0 && cols > 0 {
        Some((cols, rows))
    } else {
        None
    }
}

/// 在终端打印字符串，打印后强行刷新终端
fn print(s: &String) {
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    stdout_lock
        .write_all(s.as_ref())
        .expect("stdout_lock write_all panic");

    stdout_lock.flush().expect("stdout_lock flush panic");
}

/// 将光标置于终端的 `(x, y)` 坐标位置
///
/// 坐标从屏幕左上角开始，原点为 `(1, 1)`
fn set_cursor(x: u16, y: u16) {
    print(&format!("\x1B[{};{}H", y, x));

    // TODO 坐标越界处理
}

/// 在终端的 `(x, y)` 坐标位置打印字符串
///
/// 坐标从屏幕左上角开始，原点为 `(1, 1)`
pub fn print_at(x: u16, y: u16, s: &String) {
    set_cursor(x, y);
    print(s);

    // TODO 打印字符串越界处理
}
