mod device;
mod terminal;

use crate::terminal::utils::print_at;
use device::joystick;
use std::sync::mpsc;
use std::{thread, time};
use terminal::plot::{clean_screen, Component, Point, Widget};
use terminal::utils::terminal_size;

fn main() {
    if let Some((w, h)) = terminal_size() {
        let (input_tx, input_rx) = mpsc::channel();
        let (timer_tx, timer_rx) = mpsc::channel();

        // 获取手柄 fd
        let joystick_fd = joystick::get_fd();

        // 手柄输入线程
        thread::spawn(move || loop {
            let c = joystick::input(joystick_fd);
            input_tx.send(c).unwrap();
        });

        // 刷新率定时器线程
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_secs_f32(0.05));
            timer_tx.send(()).unwrap();
        });

        // 图像绘制线程
        run(w, h, input_rx, timer_rx);
    } else {
        // 终端异常
        println!("Terminal status error!");
    }
}

fn run(
    w: u16,
    h: u16,
    input_rx: mpsc::Receiver<Option<joystick::JoystickEvent>>,
    timer_rx: mpsc::Receiver<()>,
) {
    let mut x = 1;
    let mut y = 1;
    let mut bx = 1;
    let mut by = 1;
    let mut by_real = 1.0;
    let mut time = 0;

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
        // 接收手柄输入
        if let Ok(Some(recv)) = input_rx.try_recv() {
            // 根据 recv 移动组件
            // TODO 解决连续输入
            match recv.event {
                1 => {
                    match recv.index {
                        // A
                        0 => {
                            match recv.value {
                                // A pressed
                                1 => {
                                    bx = x;
                                    by = y - 1;
                                    by_real = by as f64;
                                    canvas.add(Widget {
                                        p0: Point { x: bx, y: by },
                                        uid: "bullet".to_string(),
                                        children: Component::Str("|".to_string()),
                                    });
                                }
                                0 => {} // A released
                                _ => {}
                            }
                        }
                        // B
                        1 => {}
                        // X
                        2 => {}
                        // Y
                        3 => {}
                        _ => {}
                    }
                }
                2 => {
                    match recv.index {
                        6 => {
                            match recv.value {
                                32767 => x += 1,  // Right
                                -32767 => x -= 1, // Left
                                _ => {}
                            }
                        }
                        7 => {
                            match recv.value {
                                32767 => y += 1,  // Down
                                -32767 => y -= 1, // Up
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            canvas.update(
                &"player".to_string(),
                Widget {
                    p0: Point { x, y },
                    uid: "player".to_string(),
                    children: Component::Str("#".to_string()),
                },
            );
        }

        // 接收刷新率定时器
        if let Ok(()) = timer_rx.try_recv() {
            clean_screen(w, h);

            time += 1;
            canvas.update(
                &"timer".to_string(),
                Widget {
                    p0: Point { x: 1, y: h },
                    uid: "timer".to_string(),
                    children: Component::Str(format!("{}", time)),
                },
            );

            if by <= 1 {
                println!("{} {}", by, by_real);
                canvas.delete(&"bullet".to_string());
            } else {
                by_real -= 0.5;
                by = by_real as u16;
                canvas.update(
                    &"bullet".to_string(),
                    Widget {
                        p0: Point { x: bx, y: by },
                        uid: "bullet".to_string(),
                        children: Component::Str("|".to_string()),
                    },
                );
            }

            canvas.draw();
            print_at(w, h, &"".to_string()); // 光标置于终端右下角
        }
    }
}
