use std::time;

#[derive(Clone, Copy)]
pub struct TimeautHeandler {
    write_timeout: time::Duration,
}

impl TimeautHeandler {
    pub fn new(write_timeout: time::Duration) -> Self {
        TimeautHeandler { write_timeout }
    }

    pub fn get_timeout(&self) -> time::Duration {
        self.write_timeout
    }
}