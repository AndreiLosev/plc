use std::time;

pub struct TimeautHeandler {
    write_timeout: time::Duration,
}

impl TimeautHeandler {
    pub fn get_timeout(&self) -> time::Duration {
        self.write_timeout
    }
}