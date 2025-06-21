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
use reqwest::Client;
use std::sync::Arc;

// Roark API configuration
const ROARK_API_URL: &str = "https://api.roark.ai/v1/integrations/vapi";
const ROARK_API_KEY_ENV: &str = "ROARK_API_KEY";

#[tokio::main]
async fn main() {
    let output_dir = Path::new("/workspaces/vapi-client/tests/server_message_assets");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    }

    // Create HTTP client for forwarding to Roark
    let client = Arc::new(Client::new());
    
    // Get Roark API key from environment variable
    let roark_api_key = std::env::var(ROARK_API_KEY_ENV).ok();
    if roark_api_key.is_none() {
        eprintln!("Warning: {} environment variable not set. Roark forwarding will be disabled.", ROARK_API_KEY_ENV);
    }

    let app = Router::new()
        .route("/", post({
            let client = client.clone();
            let roark_api_key = roark_api_key.clone();
            move |json| handle_json(json, client.clone(), roark_api_key.clone())
        }))
        .route("/{*path}", post({
            let client = client.clone();
            let roark_api_key = roark_api_key.clone();
            move |json| handle_json(json, client.clone(), roark_api_key.clone())
        }))
        .layer(ServiceBuilder::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server listening on http://0.0.0.0:3000");
    println!("JSON bodies will be saved to: {}", output_dir.display());
    if roark_api_key.is_some() {
        println!("Roark forwarding enabled to: {}", ROARK_API_URL);
    }
    
    axum::serve(listener, app).await.unwrap();
}

async fn handle_json(
    Json(payload): Json<Value>,
    client: Arc<Client>,
    roark_api_key: Option<String>
) -> impl IntoResponse {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");
    let millis = now.timestamp_subsec_millis();
    let filename = format!("json_{}_{:03}.json", timestamp, millis);
    let filepath = Path::new("/workspaces/vapi-client/tests/server_message_assets").join(&filename);
    
    let json_string = serde_json::to_string_pretty(&payload).unwrap_or_else(|_| {
        serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string())
    });
    
    // Save to file
    let save_result = match fs::write(&filepath, &json_string) {
        Ok(_) => {
            println!("Saved JSON to: {}", filepath.display());
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to save JSON: {}", e);
            Err(format!("Failed to save JSON: {}", e))
        }
    };
    
    // Forward to Roark if API key is available
    if let Some(api_key) = roark_api_key {
        let payload_clone = payload.clone();
        let client_clone = client.clone();
        
        // Spawn a task to forward to Roark asynchronously
        tokio::spawn(async move {
            match client_clone
                .post(ROARK_API_URL)
                .header("x-roark-api-key", api_key)
                .json(&payload_clone)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Successfully forwarded to Roark (status: {})", response.status());
                    } else {
                        eprintln!("Roark API returned error status: {} - {}", 
                            response.status(), 
                            response.text().await.unwrap_or_else(|_| "No response body".to_string())
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Failed to forward to Roark: {}", e);
                }
            }
        });
    }
    
    // Return response based on save result
    match save_result {
        Ok(_) => (StatusCode::OK, format!("JSON saved to {}", filename)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
    }
}