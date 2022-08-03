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

/// 清空终端
pub fn clean_screen(w: u16, h: u16) {
    let mut empty_line = String::new();
    for _ in 0..w {
        empty_line.push(' ');
    }

    for y in 1..h + 1 {
        print_at(0, y, &empty_line);
    }
}

/// 禁用键盘输入时的终端回显
pub fn no_echo() {
    let mut term = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; libc::NCCS],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut term) } < 0 {
        panic!("tcgetattr");
    }

    term.c_lflag &= !libc::ECHO;
    if unsafe { libc::tcsetattr(libc::STDIN_FILENO, 0, &term) } < 0 {
        panic!("tcsetattr");
    }
}
