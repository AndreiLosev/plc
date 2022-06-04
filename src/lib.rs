pub mod task;

use task::Task;

pub struct Plc {
    tasks: Vec<Task>,
}
