use all_asserts::assert_true;
use chrono::{Duration, Local};

use super::*;
use crate::task::TaskStatus;

#[test]
fn test_task_data_serialize() {
    let mut tasks = HashMap::new();
    let task1 = Task {
        uuid: Uuid::new_v4(),
        status: TaskStatus::Pending,
        date_created: Local::now() - Duration::try_seconds(2).unwrap(),
        ..Default::default()
    };
    let task2 = Task {
        uuid: Uuid::new_v4(),
        status: TaskStatus::Completed,
        date_created: Local::now(),
        ..Default::default()
    };
    tasks.insert(task1.uuid, task1.clone());
    tasks.insert(task2.uuid, task2.clone());

    let task_data = TaskData {
        tasks,
        undos: HashMap::new(),
        max_id: 0,
    };

    let serialized = serde_json::to_string(&task_data).unwrap();
    let expected = format!(
        r#"[{},{}]"#,
        serde_json::to_string(&task1).unwrap(),
        serde_json::to_string(&task2).unwrap(),
    );
    assert_eq!(serialized, expected);
}

#[test]
fn test_task_data_deserialize() {
    let json = r#"[
                {
                    "uuid": "00000000-0000-0000-0000-000000000001",
                    "date_created": "2023-05-25T21:25:24.899710+02:00",
                    "status": "Completed",
                    "summary": "task1",
                    "sub": [],
                    "tags": []
                },
                {
                    "uuid": "00000000-0000-0000-0000-000000000002",
                    "date_created": "2023-05-25T21:25:24.899710+02:00",
                    "status": "Completed",
                    "summary": "task3",
                    "sub": [],
                    "tags": []
                },
                {
                    "uuid": "00000000-0000-0000-0000-000000000003",
                    "date_created": "2023-05-25T21:25:24.899710+02:00",
                    "status": "Completed",
                    "summary": "task3",
                    "sub": [],
                    "tags": []
                }
        ]"#;

    let task_data: TaskData = serde_json::from_str(json).unwrap();

    assert_eq!(task_data.tasks.len(), 3);
    assert_true!(task_data
        .tasks
        .contains_key(&Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()));
    assert_true!(task_data
        .tasks
        .contains_key(&Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()));
    assert_true!(task_data
        .tasks
        .contains_key(&Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap()));
}
