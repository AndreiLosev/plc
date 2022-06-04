pub mod task;

use task::Task;

pub struct Plc {
    tasks: [Task; 32],
}
