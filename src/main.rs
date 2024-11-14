use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct ChatRequest {
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct LlamaRequest {
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/v1/chat/completions", post(chat_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn chat_handler(
    Json(request): Json<ChatRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = reqwest::Client::new();

    let llama_request = LlamaRequest {
        messages: request.messages,
        temperature: 0.7,
        max_tokens: 1000,
    };

    let response = match client
        .post("https://llama3b.gaia.domains/v1/chat/completions")
        .json(&llama_request)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("Error sending request: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response_json = match response.json::<serde_json::Value>().await {
        Ok(r) => r,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(response_json))
}
