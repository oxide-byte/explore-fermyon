use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{http::{Request, Response}, http_component};
use spin_sdk::http::{IntoResponse, Params, Router};

#[derive(Serialize, Deserialize)]
struct Message {
    message: String
}

#[derive(Serialize, Deserialize)]
struct ToLower {
    value: String
}


#[http_component]
fn aos_route_me(req: Request) -> Response {

    let mut router = Router::new();
    router.post("/lowercase", lower_response);
    router.get("/hello", hello_response);
    router.get("/hello/:name", hello_response);
    // router.any("/*", error_response);
    router.handle(req)

}

pub fn hello_response(_: Request, params: Params) -> Result<impl IntoResponse> {
    println!("*** Hello Page");

    let name = params.get("name").unwrap_or("world").to_lowercase();

    let message = Message {message: format!("Hello, {}!", name).to_string()};

    let response = Response::builder()
        .body(serde_json::to_string(&message).expect("Error converting to json"))
        .header("Content-Type", "application/json")
        .status(200)
        .build();

    Ok(response)
}

pub fn error_response(_: Request, _: Params) -> Result<impl IntoResponse> {
    println!("*** Error Page");

    let response = Response::builder()
        .header("Content-Type", "text/html")
        .status(404)
        .build();

    Ok(response)
}

pub fn lower_response(req: Request, _: Params) -> Result<impl IntoResponse> {
    println!("*** Lower Page");

    let message_in:ToLower = serde_json::from_slice(req.body()).expect("Parsing error");

    let message_out = Message {message: message_in.value.to_lowercase()};

    let response = Response::builder()
        .body(serde_json::to_string(&message_out).expect("Error converting to json"))
        .header("Content-Type", "application/json")
        .status(200)
        .build();

    Ok(response)
}