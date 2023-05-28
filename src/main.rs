pub mod filters;
pub mod manager;
pub mod task;

use manager::{TaskHandler, TaskManager};
use rocket::serde::json::Json;
use rocket::{launch, post, routes};
use task::Task;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct TaskQuery {
    query: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct TaskData {
    description: String,
    tags: Option<Vec<String>>,
    sub_tasks: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
struct StatusResponse {
    status: String,
}

#[post("/add_task", data = "<data>")]
fn add_task(data: Json<TaskData>) -> Json<StatusResponse> {
    let data_file = "data.json";
    let mut manager = TaskManager::default();
    manager.load_task_data(data_file);

    let mut sub_tasks_uuid: Vec<Uuid> = Default::default();
    if let Some(sub_tasks) = data.sub_tasks.clone() {
        for i in sub_tasks {
            if let Ok(uuid) = Uuid::parse_str(&i) {
                sub_tasks_uuid.push(uuid);
            }
        }
    }

    let mut tags_vec: Vec<String> = Default::default();
    if let Some(tags) = data.tags.clone() {
        tags_vec = tags;
    }

    manager.add_task(&data.description, tags_vec, sub_tasks_uuid);
    manager.write_task_data(data_file);

    return Json(StatusResponse {
        status: "OK".to_string(),
    });
}

#[post("/complete_task", data = "<data>")]
fn complete_task(data: Json<TaskQuery>) -> Json<StatusResponse> {
    let data_file = "data.json";
    let mut manager = TaskManager::default();
    manager.load_task_data(data_file);

    let tasks_uuid: Vec<Uuid> = manager
        .filter_tasks_from_string(&data.query)
        .iter()
        .map(|t| t.uuid)
        .collect();
    for uuid in tasks_uuid {
        manager.complete_task(&uuid);
    }

    manager.write_task_data(data_file);
    return Json(StatusResponse {
        status: String::from("OK"),
    });
}

#[post("/get_tasks", data = "<data>")]
fn get_tasks(data: Json<TaskQuery>) -> Json<Vec<Task>> {
    let data_file = "data.json";
    let mut manager = TaskManager::default();

    manager.load_task_data(data_file);
    let filtered_tasks = manager.filter_tasks_from_string(&data.query);
    let owned_tasks: Vec<Task> = filtered_tasks.iter().map(|&t| t.to_owned()).collect();

    return Json(owned_tasks);
}

#[post("/delete_task", data = "<data>")]
fn delete_task(data: Json<TaskQuery>) -> Json<StatusResponse> {
    let data_file = "data.json";
    let mut manager = TaskManager::default();
    manager.load_task_data(data_file);

    let tasks_uuid: Vec<Uuid> = manager
        .filter_tasks_from_string(&data.query)
        .iter()
        .map(|t| t.uuid)
        .collect();
    for uuid in tasks_uuid {
        manager.delete_task(&uuid);
    }

    manager.write_task_data(data_file);
    return Json(StatusResponse {
        status: String::from("OK"),
    });
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/v1", routes![add_task])
        .mount("/v1", routes![delete_task])
        .mount("/v1", routes![complete_task])
        .mount("/v1", routes![get_tasks])
}
