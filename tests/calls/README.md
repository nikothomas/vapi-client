# VAPI Calls API Tests

This directory contains tests for the VAPI Calls API.

## Required Environment Variables

Before running the tests, you need to set the following environment variables:

- `VAPI_API_KEY`: Your VAPI API key (required for all tests)
- `VAPI_TEST_PHONE_NUMBER`: A phone number to use for test calls (required for `test_create_outbound_call`)
- `VAPI_TEST_CALL_ID`: An existing call ID to test retrieval (optional, for `test_get_call_by_id`)

## Running the Tests

```bash
# Set environment variables
export VAPI_API_KEY="your-api-key-here"
export VAPI_TEST_PHONE_NUMBER="+1234567890"  # E.164 format
export VAPI_TEST_CALL_ID="existing-call-id"  # Optional

# Run all tests
cargo test --test test_calls_api

# Run a specific test
cargo test test_create_outbound_call
cargo test test_list_calls
cargo test test_get_call_by_id
```

## Test Descriptions

- **test_create_outbound_call**: Creates a new outbound call with a test assistant
- **test_list_calls**: Lists all calls in your account (shows first 5)
- **test_get_call_by_id**: Retrieves details of a specific call by ID

## Notes

- The `test_create_outbound_call` will create a real call to the specified phone number
- Make sure to use a valid phone number in E.164 format (e.g., +1234567890)
- The assistant is configured with a simple greeting message