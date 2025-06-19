use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use chrono::Local;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let output_dir = Path::new("/workspaces/vapi-client/tests/server_message_assets");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    }

    let app = Router::new()
        .route("/", post(handle_json))
        .route("/*path", post(handle_json))
        .layer(ServiceBuilder::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server listening on http://0.0.0.0:3000");
    println!("JSON bodies will be saved to: {}", output_dir.display());
    
    axum::serve(listener, app).await.unwrap();
}

async fn handle_json(Json(payload): Json<Value>) -> impl IntoResponse {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");
    let millis = now.timestamp_subsec_millis();
    let filename = format!("json_{}_{:03}.json", timestamp, millis);
    let filepath = Path::new("/workspaces/vapi-client/tests/server_message_assets").join(&filename);
    
    let json_string = serde_json::to_string_pretty(&payload).unwrap_or_else(|_| {
        serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string())
    });
    
    match fs::write(&filepath, &json_string) {
        Ok(_) => {
            println!("Saved JSON to: {}", filepath.display());
            (StatusCode::OK, format!("JSON saved to {}", filename))
        }
        Err(e) => {
            eprintln!("Failed to save JSON: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save JSON: {}", e))
        }
    }
}