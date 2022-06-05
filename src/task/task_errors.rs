use std::fmt;
use std::time::Duration;
use std::error;

#[derive(Debug)]
pub struct TaskTimeOutError {
    time_left: Duration,
    name: &'static str,
    set_time: Duration,
}

impl TaskTimeOutError {
    pub fn new(
        time_left: Duration,
        name: &'static str,
        set_time: Duration
    ) -> Self { Self { time_left, name, set_time } }
}

impl fmt::Display for TaskTimeOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let set_time = self.set_time.as_millis();
        let time_left = self.time_left.as_millis();

        write!(f, "task: {}, TaskTimeOutError, time set: {},  time left: {}", self.name, set_time, time_left)
    }
}

impl error::Error for TaskTimeOutError {}

#[derive(Debug)]
pub struct TaskOtherError(&'static str);

impl TaskOtherError {
    pub fn new(mess: &'static str) -> Self { Self(mess) }
}

impl fmt::Display for TaskOtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TaskOtherError, message: {}", self.0)
    }
}

impl error::Error for TaskOtherError {}