use anyhow::{Context, Result};
use spin_sdk::{http::{Request, Response}, http_component, llm};

/// A simple Spin HTTP component.
#[http_component]
fn handle_ai(req: Request) -> Result<Response> {

    let model = llm::InferencingModel::Llama2Chat;
    let inference = llm::infer(model, "What is serverless".into());

    if let Ok(result) = &inference {
        let text = &result.text;
        Ok(Response::builder()
            .status(200)
            .body(text.to_string()).build())
    } else {
        Ok(Response::builder()
            .status(500)
            .body("Error").build())
    }
}