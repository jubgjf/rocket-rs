/// 从 fd 读取一个字符，然后立即返回这个 char
///
/// 若没有输入，则一直阻塞
pub fn input(fd: i32) -> char {
    // TODO 处理 key delay
    // 一个可能的方案是将 fd 的 stdin 改为 /dev/...
    // 在 /proc/bus/device/devices 中看各个设备，其中 H: 就是 /dev/device/eventX

    let mut buf = ['\0'];
    let mut old = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; libc::NCCS],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    if unsafe { libc::tcgetattr(fd, &mut old) } < 0 {
        panic!("tcgetattr");
    }

    old.c_lflag &= !libc::ICANON;
    old.c_lflag &= !libc::ECHO;
    old.c_cc[libc::VMIN] = 1;
    old.c_cc[libc::VTIME] = 0;
    if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &old) } < 0 {
        panic!("tcsetattr TCSANOW");
    }
    if unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, 1) } < 0 {
        panic!("read()");
    }

    old.c_lflag |= libc::ICANON;
    old.c_lflag |= libc::ECHO;
    if unsafe { libc::tcsetattr(fd, libc::TCSADRAIN, &old) < 0 } {
        panic!("tcsetattr TCSADRAIN");
    }

    buf[0]
}
