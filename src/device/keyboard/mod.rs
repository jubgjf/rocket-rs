pub mod keycode;

/// 打开 /dev/input/event20 作为键盘，并获得对应的 fd
///
/// 打开失败时将会 panic
/// 若失败，可以尝试将用户添加到 input 组
pub fn get_fd() -> i32 {
    let device = std::ffi::CString::new("/dev/input/event20").unwrap();
    let fd = unsafe { libc::open(device.as_ptr(), libc::O_RDONLY) };
    if fd == -1 {
        panic!("open");
    }

    fd
}

/// 从 fd 读取键盘事件并返回
///
/// 若没有输入，则一直阻塞
pub fn input(fd: i32) -> Option<libc::input_event> {
    let mut buf = [libc::input_event {
        time: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        type_: 0,
        code: 0,
        value: 0,
    }];

    let kbde_size = std::mem::size_of::<libc::input_event>();
    if unsafe {
        libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, kbde_size)
            == kbde_size as libc::ssize_t
    } {
        Some(buf[0])
    } else {
        None
    }
}

/// 从 fd 读取键盘事件
/// 参数 `keys` 的第 i 项保存 keycode == i 的按键的状态，true 为按下状态，false 为松开状态
///
/// 若没有输入，则一直阻塞
///
/// TODO: refactor 与 fn input(fd: i32) 合并
pub fn keys(fd: i32, keys: &mut [bool]) {
    let mut buf = [libc::input_event {
        time: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        type_: 0,
        code: 0,
        value: 0,
    }];

    let kbde_size = std::mem::size_of::<libc::input_event>();
    if unsafe {
        libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, kbde_size)
            == kbde_size as libc::ssize_t
    } {
        let kbde = buf[0];
        if kbde.type_ == 1 {
            if kbde.value == 0 {
                // key released
                keys[kbde.code as usize] = false;
            } else if kbde.value == 1 || kbde.value == 2 {
                //key pressed || key repeated
                keys[kbde.code as usize] = true;
            }
        }
    }
}

pub fn test() {
    let fd = get_fd();
    let kbde_states = vec!["RELEASED", "PRESSED", "REPEATED"];
    println!("see /usr/include/linux/input-event-codes.h for the meaning of `code`");
    loop {
        if let Some(kbde) = input(fd) {
            if kbde.type_ == 1 && kbde.value >= 0 && kbde.value <= 2 {
                println!(
                    "state = {}, code = {}",
                    kbde_states[kbde.value as usize], kbde.code
                );
            }
        }
    }
}
