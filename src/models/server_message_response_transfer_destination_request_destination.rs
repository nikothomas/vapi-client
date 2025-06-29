/*
 * Vapi API
 *
 * Voice AI for developers.
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ServerMessageResponseTransferDestinationRequestDestination : This is the destination you'd like the call to be transferred to.
/// This is the destination you'd like the call to be transferred to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessageResponseTransferDestinationRequestDestination {
    TransferDestinationAssistant(models::TransferDestinationAssistant),
    TransferDestinationNumber(models::TransferDestinationNumber),
    TransferDestinationSip(models::TransferDestinationSip),
}

impl Default for ServerMessageResponseTransferDestinationRequestDestination {
    fn default() -> Self {
        Self::TransferDestinationAssistant(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "sip")]
    Sip,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Assistant
    }
}
/// This is the mode to use for the transfer. Defaults to `rolling-history`.  - `rolling-history`: This is the default mode. It keeps the entire conversation history and appends the new assistant's system message on transfer.    Example:    Pre-transfer:     system: assistant1 system message     assistant: assistant1 first message     user: hey, good morning     assistant: how can i help?     user: i need help with my account     assistant: (destination.message)    Post-transfer:     system: assistant1 system message     assistant: assistant1 first message     user: hey, good morning     assistant: how can i help?     user: i need help with my account     assistant: (destination.message)     system: assistant2 system message     assistant: assistant2 first message (or model generated if firstMessageMode is set to `assistant-speaks-first-with-model-generated-message`)  - `swap-system-message-in-history`: This replaces the original system message with the new assistant's system message on transfer.    Example:    Pre-transfer:     system: assistant1 system message     assistant: assistant1 first message     user: hey, good morning     assistant: how can i help?     user: i need help with my account     assistant: (destination.message)    Post-transfer:     system: assistant2 system message     assistant: assistant1 first message     user: hey, good morning     assistant: how can i help?     user: i need help with my account     assistant: (destination.message)     assistant: assistant2 first message (or model generated if firstMessageMode is set to `assistant-speaks-first-with-model-generated-message`)  - `delete-history`: This deletes the entire conversation history on transfer.    Example:    Pre-transfer:     system: assistant1 system message     assistant: assistant1 first message     user: hey, good morning     assistant: how can i help?     user: i need help with my account     assistant: (destination.message)    Post-transfer:     system: assistant2 system message     assistant: assistant2 first message     user: Yes, please     assistant: how can i help?     user: i need help with my account  - `swap-system-message-in-history-and-remove-transfer-tool-messages`: This replaces the original system message with the new assistant's system message on transfer and removes transfer tool messages from conversation history sent to the LLM.    Example:    Pre-transfer:     system: assistant1 system message     assistant: assistant1 first message     user: hey, good morning     assistant: how can i help?     user: i need help with my account     transfer-tool     transfer-tool-result     assistant: (destination.message)    Post-transfer:     system: assistant2 system message     assistant: assistant1 first message     user: hey, good morning     assistant: how can i help?     user: i need help with my account     assistant: (destination.message)     assistant: assistant2 first message (or model generated if firstMessageMode is set to `assistant-speaks-first-with-model-generated-message`)  @default 'rolling-history'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferModeTrue {
    #[serde(rename = "rolling-history")]
    RollingHistory,
    #[serde(rename = "swap-system-message-in-history")]
    SwapSystemMessageInHistory,
    #[serde(rename = "swap-system-message-in-history-and-remove-transfer-tool-messages")]
    SwapSystemMessageInHistoryAndRemoveTransferToolMessages,
    #[serde(rename = "delete-history")]
    DeleteHistory,
}

impl Default for TransferModeTrue {
    fn default() -> TransferModeTrue {
        Self::RollingHistory
    }
}
