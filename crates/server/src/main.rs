use miniserve::http;
use miniserve::{Content, Request, Response};
use std::collections::HashMap;

fn chat(_req: Request) -> Response {
    match _req {
        Request::Post(data) => {
            let json_data = serde_json::from_str::<HashMap<String, Vec<String>>>(&data);
            match json_data {
                Ok(data) => {
                    let mut messages = data.get("messages").unwrap().clone();
                    messages.push("And how does that make you feel?".to_string());
                    Ok(Content::Json(
                        (serde_json::json!({"messages": messages})).to_string(),
                    ))
                }
                Err(_e) => Err(http::StatusCode::from_u16(400_u16).unwrap()),
            }
        }
        _ => Err(http::StatusCode::from_u16(400_u16).unwrap()),
    }
}

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
