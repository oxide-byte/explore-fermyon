use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Serialize, Deserialize)]
struct HelloMessage {
    message: String
}

#[http_component]
fn aos_hello_world(_: Request) -> Result<Response> {

    let message = HelloMessage {message: "Hello, world!".to_string()};

    let response = Response::builder()
        .body(serde_json::to_string(&message).expect("Error converting to json"))
        .header("Content-Type", "application/json")
        .status(200)
        .build();

    Ok(response)
}