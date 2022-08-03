use crate::print_at;

/// 坐标
pub struct Point {
    pub x: u16,
    pub y: u16,
}

/// 一个组件
pub struct Widget {
    pub p0: Point,           // 组件左上角的点的坐标
    pub uid: String,         // 组件 id
    pub children: Component, // 组件内部的子组件
}

/// 子组件
pub enum Component {
    Widgets(Vec<Widget>), // 子组件可以由多个组件构成
    Str(String),          // 子组件可以直接是一个子符串，此时的组件就是一个子符串
}

impl Widget {
    /// 在终端打印组件
    pub fn draw(&self) {
        match &self.children {
            Component::Widgets(vw) => {
                // 递归打印子组件
                for w in vw {
                    w.draw();
                }
            }
            Component::Str(s) => print_at(self.p0.x, self.p0.y, s),
        }
    }

    /// 向组件添加子组件 `widget`
    pub fn add(&mut self, widget: Widget) {
        match &mut self.children {
            Component::Widgets(vw) => {
                // TODO 添加对 uid 互斥的检测
                // 由于整个组件树中，各个组件的 uid 应该互不相同，
                // 因此添加组件前应该先检查 uid 是否出现过
                vw.push(widget);
            }
            Component::Str(_) => panic!("Cannot add sub widget to a pure `String` widget!"),
        }
    }

    /// 从组件中删除 id 号为 `uid` 的子组件
    pub fn delete(&mut self, uid: &String) {
        if let Component::Widgets(vw) = &mut self.children {
            if let Some(index) = vw.iter().position(|x| x.uid == *uid) {
                vw.remove(index);
            } else {
                println!("{} not exist", uid);
            }
        }
    }

    /// 将 id 号为 `uid` 的子组件更新为 `widget`
    pub fn update(&mut self, uid: &String, widget: Widget) {
        self.delete(uid);
        self.add(widget);
    }
}
