#[cfg(test)]
use super::*;

#[test]
fn test_task_data_serialize() {
    let mut tasks = HashMap::new();
    let task1 = Task {
        uuid: Uuid::new_v4(),
        status: TaskStatus::PENDING,
        ..Default::default()
    };
    let task2 = Task {
        uuid: Uuid::new_v4(),
        status: TaskStatus::COMPLETED,
        ..Default::default()
    };
    tasks.insert(task1.uuid, task1.clone());
    tasks.insert(task2.uuid, task2.clone());

    let task_data = TaskData {
        tasks,
        id_to_uuid: HashMap::new(),
        operations: Vec::default(),
    };

    let serialized = serde_json::to_string(&task_data).unwrap();
    let expected = format!(
        r#"{{"completed":[{}],"pending":[{}],"deleted":[],"operations":[]}}"#,
        serde_json::to_string(&task2).unwrap(),
        serde_json::to_string(&task1).unwrap(),
    );
    assert_eq!(serialized, expected);
}

#[test]
fn test_task_data_deserialize() {
    let json = r#"{
            "completed": [
                {
                    "uuid": "00000000-0000-0000-0000-000000000001",
                    "date_created": "2023-05-25T21:25:24.899710+02:00",
                    "status": "COMPLETED",
                    "description": "task1",
                    "sub": [],
                    "tags": []
                }
            ],
            "pending": [
                {
                    "uuid": "00000000-0000-0000-0000-000000000002",
                    "date_created": "2023-05-25T21:25:24.899710+02:00",
                    "status": "COMPLETED",
                    "description": "task2",
                    "sub": [],
                    "tags": []
                },
                {
                    "uuid": "00000000-0000-0000-0000-000000000003",
                    "date_created": "2023-05-25T21:25:24.899710+02:00",
                    "status": "COMPLETED",
                    "description": "task3",
                    "sub": [],
                    "tags": []
                }
            ],
            "deleted": [],
            "operations": []
        }"#;

    let task_data: TaskData = serde_json::from_str(json).unwrap();

    assert_eq!(task_data.tasks.len(), 3);
    assert_eq!(
        task_data
            .tasks
            .contains_key(&Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()),
        true
    );
    assert_eq!(
        task_data
            .tasks
            .contains_key(&Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()),
        true
    );
    assert_eq!(
        task_data
            .tasks
            .contains_key(&Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap()),
        true
    );
}

#[test]
fn test_task_data_get_pending_count() {
    let mut tasks = HashMap::new();
    tasks.insert(
        Uuid::new_v4(),
        Task {
            status: TaskStatus::PENDING,
            ..Default::default()
        },
    );
    tasks.insert(
        Uuid::new_v4(),
        Task {
            status: TaskStatus::COMPLETED,
            ..Default::default()
        },
    );
    tasks.insert(
        Uuid::new_v4(),
        Task {
            status: TaskStatus::PENDING,
            ..Default::default()
        },
    );

    let task_data = TaskData {
        tasks,
        id_to_uuid: HashMap::new(),
        operations: Vec::default(),
    };

    assert_eq!(task_data.get_pending_count(), 2);
}
