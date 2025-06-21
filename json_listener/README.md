# JSON Listener with Roark Forwarding

This service listens for JSON payloads and:
1. Saves them locally to `/workspaces/vapi-client/tests/server_message_assets`
2. Forwards them to Roark API at `https://api.roark.ai/v1/integrations/vapi`

## Setup

### Environment Variables

Set the Roark API key:
```bash
export ROARK_API_KEY="your-roark-api-key-here"
```

If the API key is not set, the service will still save JSON locally but won't forward to Roark.

## Building and Running

```bash
cd json_listener
cargo build --release
cargo run
```

The server will listen on `http://0.0.0.0:3000`

## VAPI Configuration

In your VAPI webhook settings:
1. Set the Server URL to your json_listener endpoint (e.g., `http://your-server:3000`)
2. The service will automatically forward to Roark with the required `x-roark-api-key` header

## Features

- Saves all incoming JSON payloads with timestamps
- Asynchronously forwards to Roark (non-blocking)
- Handles any path (both `/` and `/*path`)
- Pretty-prints JSON for local storage
- Logs success/failure for both local save and Roark forwarding 