use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{http::{Request, Response}, http_component};
use spin_sdk::http::{IntoResponse, Params, Router};
use spin_sdk::key_value::Store;

#[derive(Serialize, Deserialize)]
struct Data {
    value: String
}

#[http_component]
fn aos_home_page(req: Request) -> Response {

    let mut router = Router::new();
    router.get("/:key", get_key);
    router.put("/:key", add_key);
    router.handle(req)

}

pub fn get_key(_: Request, params: Params) -> Result<impl IntoResponse> {

    let key = params.get("key").expect("Invalid Key");
    let mut data = Data {value:"".to_string()};

    println!("Retrieve key [{}]", key);

    let store = Store::open_default()?;

    if let Ok(value) = store.get(key) {
        let str = String::from_utf8(value.expect("No value")).expect("Found invalid UTF-8");
        println!("Reply with [{}]", str);
        data = Data { value: str};
    }

    let response = Response::builder()
        .body(serde_json::to_string(&data).expect("Error converting to json"))
        .header("Content-Type", "application/json")
        .status(200)
        .build();

    Ok(response)
}

pub fn add_key(req: Request, params: Params) -> Result<impl IntoResponse> {

    let key = params.get("key").expect("Invalid Key");
    let data:Data = serde_json::from_slice(req.body()).expect("Data Parsing error");

    println!("Store key/value [{}/{}]", key, data.value);

    let store = Store::open_default()?;
    store.set(key, data.value.as_bytes());

    let response = Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .build();

    Ok(response)
}