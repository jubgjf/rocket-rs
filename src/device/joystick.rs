/// 手柄事件
///
/// 仅使用 Microsoft Xbox Series S|X Controller 手柄做过测试
///
/// 手柄功能组件包括：
///     左摇杆(LS, `index` = 0/1)，右摇杆(RS, `index` = 3/4)，
///     左扳机(LZ, `index` = 2)，  右扳机(RZ, `index` = 5)，
///     左肩键(TL, `index` = 4)，  右肩键(TR, `index` = 5)，
///     方向键(Up/Down, `index` = 7 / Left/Right, `index` = 6)，
///     地图键(SELECT, `index` = 6)，
///     功能键(START,  `index` = 7)，
///     开关键(MODE,   `index` = 8)，
///     A/B/X/Y 键(A, `index` = 0 / B, `index` = 1 / X, `index` = 2 / Y, `index` = 3)
///
/// 其中 TR, TL, LS(按下), RS(按下), A, B, X, Y, SELECT, START, MODE 被视为按键类，
/// 其中 LZ, RZ, LS(摇动), RS(摇动), Up, Down, Left, Right 被视为摇杆类，
///
/// 对于按键类，被触发时 `event` == 1；
/// 按下的 `value` == 1，松开的 `value` == 0
///
/// 对于摇杆类，被触发时 `event` == 2；
/// LS 和 RS TODO 摇杆旋转的 value
/// LZ 和 RZ 全部按下的 `value` == 32767，全部松开的 `value` == -32767，其余线性变化
/// Up   按下的 `value` == -32767，Down  按下的 `value` == 32767，松开的 `value` == 0
/// Left 按下的 `value` == -32767，Right 按下的 `value` == 32767，松开的 `value` == 0
#[derive(Copy)]
pub struct JoystickEvent {
    pub timestamp: u32, // 事件时间戳
    pub value: i16,     // 摇杆偏移量或按钮状态
    pub event: u8,      // 事件类别
    pub index: u8,      // 摇杆或按钮的序号
}

impl Clone for JoystickEvent {
    fn clone(&self) -> Self {
        JoystickEvent {
            timestamp: self.timestamp,
            value: self.value,
            event: self.event,
            index: self.index,
        }
    }
}

/// 打开 /dev/input/js0 最为手柄，并获得对应的 fd
///
/// 打开失败时将会 panic
pub fn get_fd() -> i32 {
    let device = std::ffi::CString::new("/dev/input/js0").unwrap();
    let fd = unsafe { libc::open(device.as_ptr(), libc::O_RDONLY) };
    if fd == -1 {
        panic!("open");
    }

    fd
}

/// 从 fd 读取手柄事件并返回
///
/// 若没有输入，则一直阻塞
pub fn input(fd: i32) -> Option<JoystickEvent> {
    let mut buf = [JoystickEvent {
        timestamp: 0,
        value: 0,
        event: 0,
        index: 0,
    }];

    let jse_size = std::mem::size_of::<JoystickEvent>();
    if unsafe {
        libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, jse_size) == jse_size as libc::ssize_t
    } {
        Some(buf[0])
    } else {
        None
    }
}

pub fn test() {
    let fd = get_fd();
    loop {
        if let Some(jse) = input(fd) {
            println!(
                "value {:>6} | event {} | index {}",
                jse.value, jse.event, jse.index
            );
        }
    }
}
