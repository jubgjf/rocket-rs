mod device;
mod terminal;

use device::keyboard;
use std::{sync::mpsc, thread, time};
use terminal::utils::{clean_screen, print_at, terminal_size};
use terminal::widget::{Component, Point, Widget};

fn main() {
    if let Some((w, h)) = terminal_size() {
        let (input_tx, input_rx) = mpsc::channel();
        let (timer_tx, timer_rx) = mpsc::channel();

        // 键盘输入线程
        let fd = keyboard::get_fd();
        let mut keys: [bool; 256] = [false; 256];
        thread::spawn(move || loop {
            keyboard::keys(fd, &mut keys);
            input_tx.send(keys).unwrap();
        });

        // 刷新率定时器线程 -> 20 fps
        let fps = 20_f32;
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_secs_f32(1_f32 / fps));
            timer_tx.send(()).unwrap();
        });

        // 禁用终端回显
        terminal::utils::no_echo();

        // 游戏主线程
        run(w, h, input_rx, timer_rx);
    } else {
        // 终端异常
        println!("Terminal status error!");
    }
}

fn run(w: u16, h: u16, input_rx: mpsc::Receiver<[bool; 256]>, timer_rx: mpsc::Receiver<()>) {
    let mut x = 1;
    let mut y = 1;

    let mut time = 0;

    let mut keys: [bool; 256] = [false; 256];

    let mut canvas = Widget {
        p0: Point { x: 1, y: 1 },
        uid: "canvas".to_string(),
        children: Component::Widgets(Vec::new()),
    };

    canvas.add(Widget {
        p0: Point { x, y },
        uid: "player".to_string(),
        children: Component::Str("#".to_string()),
    });
    canvas.add(Widget {
        p0: Point { x: 1, y: h },
        uid: "timer".to_string(),
        children: Component::Str(format!("{}", time)),
    });

    loop {
        // 接收键盘输入，刷新 keys 状态
        if let Ok(recv) = input_rx.try_recv() {
            keys = recv;
        }

        // 接收刷新率定时器
        if let Ok(()) = timer_rx.try_recv() {
            clean_screen(w, h);

            // 移动组件
            if keys[keyboard::keycode::KEY_W] && y > 1 {
                y -= 1;
            }
            if keys[keyboard::keycode::KEY_A] && x > 1 {
                x -= 1;
            }
            if keys[keyboard::keycode::KEY_S] && y < h {
                y += 1;
            }
            if keys[keyboard::keycode::KEY_D] && x < w {
                x += 1;
            }

            // 更新定时器
            time += 1;

            // 更新组件显示
            canvas.update(
                &"player".to_string(),
                Widget {
                    p0: Point { x, y },
                    uid: "player".to_string(),
                    children: Component::Str("#".to_string()),
                },
            );
            canvas.update(
                &"timer".to_string(),
                Widget {
                    p0: Point { x: 1, y: h },
                    uid: "timer".to_string(),
                    children: Component::Str(format!("{}", time)),
                },
            );

            canvas.draw();
            print_at(w, h, &"".to_string()); // 光标置于终端右下角
        }
    }
}
