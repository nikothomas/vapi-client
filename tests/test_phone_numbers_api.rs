use std::env;
use vapi_client::apis::configuration::Configuration;
use vapi_client::apis::phone_numbers_api;
use vapi_client::models::{
    self, CreateVapiPhoneNumberDto, PhoneNumberControllerCreateRequest,
    PhoneNumberControllerUpdateRequest,
};

#[tokio::test]
async fn test_create_vapi_phone_number() {
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    let mut vapi_phone_number = CreateVapiPhoneNumberDto::new(
        models::create_vapi_phone_number_dto::ProviderTrue::Vapi,
    );
    vapi_phone_number.name = Some("Test Phone Number".to_string());
    vapi_phone_number.number_desired_area_code = Some("415".to_string());

    // If you have an assistant ID, you can set it
    let assistant_id = env::var("VAPI_TEST_ASSISTANT_ID").ok();
    if let Some(id) = assistant_id {
        vapi_phone_number.assistant_id = Some(id);
    }

    let create_request = PhoneNumberControllerCreateRequest::Vapi(vapi_phone_number);

    match phone_numbers_api::phone_number_controller_create(&config, create_request).await {
        Ok(response) => {
            println!("Phone number created successfully!");
            println!("Response: {:?}", response);
            // Check that the response has an ID (all variants have an ID field)
            let id = match &response {
                models::PhoneNumber::Vapi(vapi) => &vapi.id,
                models::PhoneNumber::Twilio(twilio) => &twilio.id,
                models::PhoneNumber::Vonage(vonage) => &vonage.id,
                models::PhoneNumber::ByoPhoneNumber(byo) => &byo.id,
                models::PhoneNumber::Telnyx(telnyx) => &telnyx.id,
            };
            assert!(!id.is_empty());
        }
        Err(e) => {
            eprintln!("Error creating phone number: {:?}", e);
            panic!("Failed to create phone number");
        }
    }
}

#[tokio::test]
async fn test_list_phone_numbers() {
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    match phone_numbers_api::phone_number_controller_find_all(
        &config, None, None, None, None, None, None, None, None, None,
    )
    .await
    {
        Ok(phone_numbers) => {
            println!("Found {} phone numbers", phone_numbers.len());
            for phone_number in phone_numbers.iter().take(5) {
                // Extract common fields from the phone number variant
                match phone_number {
                    models::PhoneNumber::Vapi(vapi) => {
                        println!(
                            "Phone Number ID: {:?}, Name: {:?}, Number: {:?}",
                            vapi.id, vapi.name, vapi.number
                        );
                    }
                    models::PhoneNumber::Twilio(twilio) => {
                        println!(
                            "Phone Number ID: {:?}, Name: {:?}, Number: {:?}",
                            twilio.id, twilio.name, twilio.number
                        );
                    }
                    models::PhoneNumber::Vonage(vonage) => {
                        println!(
                            "Phone Number ID: {:?}, Name: {:?}, Number: {:?}",
                            vonage.id, vonage.name, vonage.number
                        );
                    }
                    models::PhoneNumber::ByoPhoneNumber(byo) => {
                        println!(
                            "Phone Number ID: {:?}, Name: {:?}, Number: {:?}",
                            byo.id, byo.name, byo.number
                        );
                    }
                    models::PhoneNumber::Telnyx(telnyx) => {
                        println!(
                            "Phone Number ID: {:?}, Name: {:?}, Number: {:?}",
                            telnyx.id, telnyx.name, telnyx.number
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing phone numbers: {:?}", e);
            panic!("Failed to list phone numbers");
        }
    }
}

#[tokio::test]
async fn test_get_phone_number_by_id() {
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");
    let phone_number_id = env::var("VAPI_TEST_PHONE_NUMBER_ID").unwrap_or_else(|_| {
        println!("VAPI_TEST_PHONE_NUMBER_ID not set, skipping test_get_phone_number_by_id");
        return String::new();
    });

    if phone_number_id.is_empty() {
        return;
    }

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    match phone_numbers_api::phone_number_controller_find_one(&config, &phone_number_id).await {
        Ok(phone_number) => {
            println!("Retrieved phone number successfully!");
            // Extract common fields from the phone number variant
            match &phone_number {
                models::PhoneNumber::Vapi(vapi) => {
                    println!("Phone Number ID: {:?}", vapi.id);
                    println!("Name: {:?}", vapi.name);
                    println!("Number: {:?}", vapi.number);
                    println!("Provider: vapi");
                    println!("Created at: {:?}", vapi.created_at);
                }
                models::PhoneNumber::Twilio(twilio) => {
                    println!("Phone Number ID: {:?}", twilio.id);
                    println!("Name: {:?}", twilio.name);
                    println!("Number: {:?}", twilio.number);
                    println!("Provider: twilio");
                    println!("Created at: {:?}", twilio.created_at);
                }
                models::PhoneNumber::Vonage(vonage) => {
                    println!("Phone Number ID: {:?}", vonage.id);
                    println!("Name: {:?}", vonage.name);
                    println!("Number: {:?}", vonage.number);
                    println!("Provider: vonage");
                    println!("Created at: {:?}", vonage.created_at);
                }
                models::PhoneNumber::ByoPhoneNumber(byo) => {
                    println!("Phone Number ID: {:?}", byo.id);
                    println!("Name: {:?}", byo.name);
                    println!("Number: {:?}", byo.number);
                    println!("Provider: byo-phone-number");
                    println!("Created at: {:?}", byo.created_at);
                }
                models::PhoneNumber::Telnyx(telnyx) => {
                    println!("Phone Number ID: {:?}", telnyx.id);
                    println!("Name: {:?}", telnyx.name);
                    println!("Number: {:?}", telnyx.number);
                    println!("Provider: telnyx");
                    println!("Created at: {:?}", telnyx.created_at);
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting phone number: {:?}", e);
            panic!("Failed to get phone number");
        }
    }
}

#[tokio::test]
async fn test_update_phone_number() {
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");
    let phone_number_id = env::var("VAPI_TEST_PHONE_NUMBER_ID").unwrap_or_else(|_| {
        println!("VAPI_TEST_PHONE_NUMBER_ID not set, skipping test_update_phone_number");
        return String::new();
    });

    if phone_number_id.is_empty() {
        return;
    }

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    // First, get the existing phone number to determine its type
    let phone_number = match phone_numbers_api::phone_number_controller_find_one(&config, &phone_number_id).await {
        Ok(pn) => pn,
        Err(e) => {
            eprintln!("Error getting phone number: {:?}", e);
            panic!("Failed to get phone number for update");
        }
    };

    // Create an update request based on the phone number type
    // For this example, we'll only update the name
    let update_request = match phone_number {
        models::PhoneNumber::Vapi(_) => {
            let mut update_dto = models::UpdateVapiPhoneNumberDto::default();
            update_dto.name = Some("Updated Test Phone Number".to_string());
            PhoneNumberControllerUpdateRequest::Vapi(update_dto)
        }
        models::PhoneNumber::Twilio(_) => {
            let mut update_dto = models::UpdateTwilioPhoneNumberDto::default();
            update_dto.name = Some("Updated Test Phone Number".to_string());
            PhoneNumberControllerUpdateRequest::Twilio(update_dto)
        }
        models::PhoneNumber::Vonage(_) => {
            let mut update_dto = models::UpdateVonagePhoneNumberDto::default();
            update_dto.name = Some("Updated Test Phone Number".to_string());
            PhoneNumberControllerUpdateRequest::Vonage(update_dto)
        }
        models::PhoneNumber::ByoPhoneNumber(_) => {
            let mut update_dto = models::UpdateByoPhoneNumberDto::default();
            update_dto.name = Some("Updated Test Phone Number".to_string());
            PhoneNumberControllerUpdateRequest::ByoPhoneNumber(update_dto)
        }
        models::PhoneNumber::Telnyx(_) => {
            let mut update_dto = models::UpdateTelnyxPhoneNumberDto::default();
            update_dto.name = Some("Updated Test Phone Number".to_string());
            PhoneNumberControllerUpdateRequest::Telnyx(update_dto)
        }
    };

    match phone_numbers_api::phone_number_controller_update(&config, &phone_number_id, update_request).await {
        Ok(updated_phone_number) => {
            println!("Phone number updated successfully!");
            // Extract name from the updated phone number variant
            let updated_name = match &updated_phone_number {
                models::PhoneNumber::Vapi(vapi) => &vapi.name,
                models::PhoneNumber::Twilio(twilio) => &twilio.name,
                models::PhoneNumber::Vonage(vonage) => &vonage.name,
                models::PhoneNumber::ByoPhoneNumber(byo) => &byo.name,
                models::PhoneNumber::Telnyx(telnyx) => &telnyx.name,
            };
            println!("Updated Name: {:?}", updated_name);
            assert_eq!(
                updated_name,
                &Some("Updated Test Phone Number".to_string())
            );
        }
        Err(e) => {
            eprintln!("Error updating phone number: {:?}", e);
            panic!("Failed to update phone number");
        }
    }
}

#[tokio::test]
async fn test_delete_phone_number() {
    let api_key = env::var("VAPI_API_KEY").expect("VAPI_API_KEY environment variable must be set");
    let phone_number_id = env::var("VAPI_TEST_DELETE_PHONE_NUMBER_ID").unwrap_or_else(|_| {
        println!("VAPI_TEST_DELETE_PHONE_NUMBER_ID not set, skipping test_delete_phone_number");
        println!("This test requires a phone number ID that is safe to delete");
        return String::new();
    });

    if phone_number_id.is_empty() {
        return;
    }

    let mut config = Configuration::new();
    config.bearer_access_token = Some(api_key);

    match phone_numbers_api::phone_number_controller_remove(&config, &phone_number_id).await {
        Ok(deleted_phone_number) => {
            println!("Phone number deleted successfully!");
            // Extract fields from the deleted phone number variant
            let (id, name) = match &deleted_phone_number {
                models::PhoneNumber::Vapi(vapi) => (&vapi.id, &vapi.name),
                models::PhoneNumber::Twilio(twilio) => (&twilio.id, &twilio.name),
                models::PhoneNumber::Vonage(vonage) => (&vonage.id, &vonage.name),
                models::PhoneNumber::ByoPhoneNumber(byo) => (&byo.id, &byo.name),
                models::PhoneNumber::Telnyx(telnyx) => (&telnyx.id, &telnyx.name),
            };
            println!("Deleted Phone Number ID: {:?}", id);
            println!("Deleted Phone Number Name: {:?}", name);
        }
        Err(e) => {
            eprintln!("Error deleting phone number: {:?}", e);
            panic!("Failed to delete phone number");
        }
    }
}