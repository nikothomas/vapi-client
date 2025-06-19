# VAPI Client Tests

This directory contains integration tests for the VAPI client library.

## Available Tests

### test_calls_api.rs
Tests for the Calls API endpoints:
- Creating outbound calls
- Listing calls
- Getting call details by ID

### test_phone_numbers_api.rs
Tests for the Phone Numbers API endpoints:
- Creating VAPI phone numbers
- Listing phone numbers
- Getting phone number details by ID
- Updating phone numbers
- Deleting phone numbers

## Running Tests

Set up the required environment variables before running tests:

```bash
# Required for all tests
export VAPI_API_KEY="your-api-key"

# For calls tests
export VAPI_TEST_PHONE_NUMBER="phone-number-to-call"
export VAPI_PHONE_ID="your-vapi-phone-id"
export ELEVEN_LABS_VOICE_ID="voice-id"
export VAPI_TEST_CALL_ID="optional-call-id-for-get-test"

# For phone numbers tests
export VAPI_TEST_ASSISTANT_ID="optional-assistant-id"
export VAPI_TEST_PHONE_NUMBER_ID="phone-number-id-for-get-update-tests"
export VAPI_TEST_DELETE_PHONE_NUMBER_ID="phone-number-id-safe-to-delete"
```

Run all tests:
```bash
cargo test
```

Run specific test file:
```bash
cargo test --test test_phone_numbers_api
```

Run specific test function:
```bash
cargo test --test test_phone_numbers_api test_list_phone_numbers
```