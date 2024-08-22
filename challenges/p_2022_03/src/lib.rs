use anyhow::Result;
use spin_sdk::{http::{Request, Response}, http_component};
use spin_sdk::http::{IntoResponse, Params, Router};

#[http_component]
fn aos_route_me(req: Request) -> Response {

    let mut router = Router::new();
    router.get("/", home_page);
    router.handle(req)

}

pub fn home_page(_: Request, _: Params) -> Result<impl IntoResponse> {
    println!("*** Lower Page");

    let page = "\
    <html>
    <body>
    <h1>Welcome to the Advent of Spin!</h1>
    Santa's on his way
    </body>
    </html>
    ";

    let response = Response::builder()
        .body(page)
        .header("Content-Type", "text/html")
        .status(200)
        .build();

    Ok(response)
}