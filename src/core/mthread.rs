use std::thread;

pub struct MThread;

impl MThread {
    pub fn new() -> Self {
        MThread {}
    }

    pub fn start(self) {
        thread::spawn(move || self.run());
    }

    pub fn run(&self) {}

    pub fn finished(&self) {}
}
