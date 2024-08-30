use anyhow::Result;
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use spin_sdk::{http::{Request, Response}, http_component};
use spin_sdk::http::{IntoResponse, Params, Router};
use spin_sdk::sqlite::{Connection, Value};
use uuid::{Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>
}

#[http_component]
fn aos_route_me(req: Request) -> Response {
    println!("Request {:?} - {:?}", req.method(), req.path());

    let mut router = Router::new();
    router.get("/api/", get_todo_list);
    router.post("/api/", insert_todo);
    router.put("/api/", update_todo);
    router.delete("/api/:id", delete_todo);
    router.handle(req)
}

pub fn get_todo_list(_: Request, _: Params) -> Result<impl IntoResponse> {
    println!("*** get_todo_list");

    let connection = Connection::open_default()?;

    let rowset = connection.execute(
        "SELECT id, title, description, created FROM todos",
        &[]
    )?;

    let todos: Vec<_> = rowset.rows().map(|row| {

        Todo {
            id: row.get::<&str>("id").unwrap().to_owned(),
            title: row.get::<&str>("title").unwrap().to_owned(),
            description: row.get::<&str>("description").unwrap().to_owned(),
            created: map_to_utc(row.get::<&str>("created").unwrap().to_owned()),
            //created: Utc::now(),
        }
    }
    ).collect();

    let response = Response::builder()
        .status(200)
        .body(serde_json::to_string(&todos).expect("Error converting to json"))
        .header("Content-Type", "application/json")
        .build();

    Ok(response)
}

pub fn insert_todo(req: Request, _: Params) -> Result<impl IntoResponse> {
    println!("*** insert_todo");

    let todo:Todo = serde_json::from_slice(req.body()).expect("Parsing error");

    let connection = Connection::open_default()?;

    let now = {
        let now = Utc::now();
        format!("{}-{}-{} {}:{}:{}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        )
    };

    let execute_params = [
        Value::Text(Uuid::new_v4().to_string()),
        Value::Text(todo.title.clone()),
        Value::Text(todo.description.clone()),
        Value::Text(now.clone()),
    ];

    connection.execute(
        "INSERT INTO todos (id, title, description, created) VALUES (?, ?, ?, ?)",
        execute_params.as_slice(),
    )?;

    let response = Response::builder()
        .status(200)
        .build();

    Ok(response)
}

pub fn update_todo(req: Request, _: Params) -> Result<impl IntoResponse> {
    println!("*** update_todo");

    let todo:Todo = serde_json::from_slice(req.body()).expect("Parsing error");

    let connection = Connection::open_default()?;

    let execute_params = [
        Value::Text(todo.title.clone()),
        Value::Text(todo.description.clone()),
        Value::Text(todo.id.clone()),
    ];

    connection.execute(
        "update todos set title = ?, description = ? where id = ?",
        execute_params.as_slice(),
    )?;

    let response = Response::builder()
        .status(200)
        .build();

    Ok(response)
}

pub fn delete_todo(_: Request, params: Params) -> Result<impl IntoResponse> {
    println!("*** delete_todo");

    let id = params.get("id").expect("No ID provided");

    let connection = Connection::open_default()?;

    let execute_params = [
        Value::Text(id.to_string()),
    ];

    connection.execute(
        "DELETE FROM todos where id = ?",
        &execute_params
    )?;

    let response = Response::builder()
        .status(200)
        .build();

    Ok(response)
}

fn map_to_utc(p0: String) -> DateTime<Utc> {
    // "2024-08-30 15:07:26"
    let dt = NaiveDateTime::parse_from_str(p0.as_str(), "%F %T").unwrap();
    dt.and_utc()
}