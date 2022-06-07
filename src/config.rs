use serde::{Deserialize};

#[derive(Debug, PartialEq, Deserialize)]
pub struct General {
    pub task_setting: TaskSetting
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct TaskSetting {
    pub max_work_time_for_not_cycle_task: u64,
    pub return_time_work: u64,
}


#[test]
fn test_config_general() {
    use std::{env, fs};

    let mut path = env::current_dir().unwrap();
    path.push("config");
    path.push("gпeneral.yaml");

    let s = fs::read_to_string(path).unwrap();
    let x: General = serde_yaml::from_str(&s).unwrap();

    let expect = General {
        task_setting : TaskSetting {
            max_work_time_for_not_cycle_task: 500,
            return_time_work: 1000,
        }
    };

    assert_eq!(x, expect);
}