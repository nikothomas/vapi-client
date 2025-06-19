use std::env;
use vapi_client::apis::calls_api;
use vapi_client::apis::configuration::Configuration;
use vapi_client::models::chunk_plan::PunctuationBoundary;
use vapi_client::models::eleven_labs_transcriber::LanguageTrue;
use vapi_client::models::{
    self, eleven_labs_voice, open_ai_model, ChunkPlan, CreateAssistantDto, CreateAssistantDtoModel,
    CreateCallDto, CreateCustomerDto, FormatPlan,
};

#[tokio::test]
async fn test_create_outbound_call() {
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");
    let phone_number = env::var("VAPI_TEST_PHONE_NUMBER")
        .expect("VAPI_TEST_PHONE_NUMBER environment variable must be set");
    let vapi_outbound_number_id =
        env::var("VAPI_PHONE_ID").expect("VAPI_PHONE_ID environment variable must be set");
    let eleven_labs_voice_id = env::var("ELEVEN_LABS_VOICE_ID")
        .expect("ELEVEN_LABS_VOICE_ID environment variable must be set");
    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    let mut customer = CreateCustomerDto::new();
    customer.number = Some(phone_number);
    customer.name = Some("Test Customer".to_string());

    let mut assistant = CreateAssistantDto::new();
    assistant.name = Some("Test Assistant".to_string());
    assistant.model = Some(CreateAssistantDtoModel::OpenAiModel(
        models::OpenAiModel::new(
            open_ai_model::ProviderTrue::Openai,
            open_ai_model::ModelTrue::Gpt4Period120250414,
        ),
    ));
    assistant.first_message =
        Some("Hello! This is a test call from the VAPI client library.".to_string());
    assistant.voice = Some(models::CreateAssistantDtoVoice::ElevenLabsVoice(
        (models::ElevenLabsVoice {
            caching_enabled: Some(true),
            provider: eleven_labs_voice::ProviderTrue::Variant11labs,
            voice_id: models::ElevenLabsVoiceVoiceId::Model11LabsVoiceId(eleven_labs_voice_id),
            stability: Some(0.5),
            similarity_boost: Some(0.5),
            style: Some(0.5),
            use_speaker_boost: Some(true),
            speed: Some(1.0),
            optimize_streaming_latency: Some(3),
            enable_ssml_parsing: Some(true),
            auto_mode: Some(true),
            model: Some(eleven_labs_voice::ModelTrue::ElevenTurboV25),
            chunk_plan: Some(ChunkPlan {
                enabled: Some(true),
                min_characters: Some(80.0),
                punctuation_boundaries: None,
                format_plan: None,
            }),
            language: Some("en".to_string()),
            fallback_plan: None,
        }),
    ));
    let mut call_dto = CreateCallDto::new();
    call_dto.name = Some("Test Call".to_string());
    call_dto.customer = Some(customer);
    call_dto.assistant = Some(assistant);
    call_dto.phone_number_id = Some(vapi_outbound_number_id);

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
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    match calls_api::call_controller_find_all(
        &config, None, None, None, None, None, None, None, None, None, None, None, None,
    )
    .await
    {
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
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");
    let call_id = env::var("VAPI_TEST_CALL_ID").unwrap_or_else(|_| {
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
