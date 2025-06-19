use vapi_client::apis::configuration::Configuration;
use vapi_client::apis::calls_api;
use vapi_client::models::{CreateCallDto, CreateCustomerDto, CreateAssistantDto};
use std::env;

#[tokio::test]
async fn test_create_outbound_call() {
    let api_key = env::var("VAPI_API_KEY")
        .expect("VAPI_API_KEY environment variable must be set");
    let phone_number = env::var("VAPI_TEST_PHONE_NUMBER")
        .expect("VAPI_TEST_PHONE_NUMBER environment variable must be set");

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    let mut customer = CreateCustomerDto::new();
    customer.number = Some(phone_number);
    customer.name = Some("Test Customer".to_string());

    let mut assistant = CreateAssistantDto::new();
    assistant.name = Some("Test Assistant".to_string());
    assistant.model = Some(serde_json::json!({
        "provider": "openai",
        "model": "gpt-3.5-turbo"
    }));
    assistant.voice = Some(serde_json::json!({
        "provider": "playht",
        "voiceId": "larry"
    }));
    assistant.first_message = Some("Hello! This is a test call from the VAPI client library.".to_string());

    let mut call_dto = CreateCallDto::new();
    call_dto.name = Some("Test Call".to_string());
    call_dto.customer = Some(customer);
    call_dto.assistant = Some(assistant);

    match calls_api::call_controller_create(&config, call_dto).await {
        Ok(response) => {
            println!("Call created successfully!");
            println!("Response: {:?}", response);
        }
        Err(e) => {
            eprintln!("Error creating call: {:?}", e);
            panic!("Failed to create call");
        }
    }
}

#[tokio::test]
async fn test_list_calls() {
    let api_key = env::var("VAPI_API_KEY")
        .expect("VAPI_API_KEY environment variable must be set");

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    match calls_api::call_controller_find_all(&config, None, None, None, None, None, None, None, None, None, None, None).await {
        Ok(calls) => {
            println!("Found {} calls", calls.len());
            for call in calls.iter().take(5) {
                println!("Call ID: {:?}, Status: {:?}", call.id, call.status);
            }
        }
        Err(e) => {
            eprintln!("Error listing calls: {:?}", e);
            panic!("Failed to list calls");
        }
    }
}

#[tokio::test]
async fn test_get_call_by_id() {
    let api_key = env::var("VAPI_API_KEY")
        .expect("VAPI_API_KEY environment variable must be set");
    let call_id = env::var("VAPI_TEST_CALL_ID")
        .unwrap_or_else(|_| {
            println!("VAPI_TEST_CALL_ID not set, skipping test_get_call_by_id");
            return String::new();
        });

    if call_id.is_empty() {
        return;
    }

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    match calls_api::call_controller_find_one(&config, &call_id).await {
        Ok(call) => {
            println!("Retrieved call successfully!");
            println!("Call ID: {:?}", call.id);
            println!("Status: {:?}", call.status);
            println!("Type: {:?}", call.r#type);
            println!("Created at: {:?}", call.created_at);
        }
        Err(e) => {
            eprintln!("Error getting call: {:?}", e);
            panic!("Failed to get call");
        }
    }
}