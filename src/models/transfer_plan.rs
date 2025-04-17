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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferPlan {
    /// This configures how transfer is executed and the experience of the destination party receiving the call.  Usage: - `blind-transfer`: The assistant forwards the call to the destination without any message or summary. - `blind-transfer-add-summary-to-sip-header`: The assistant forwards the call to the destination and adds a SIP header X-Transfer-Summary to the call to include the summary. - `warm-transfer-say-message`: The assistant dials the destination, delivers the `message` to the destination party, connects the customer, and leaves the call. - `warm-transfer-say-summary`: The assistant dials the destination, provides a summary of the call to the destination party, connects the customer, and leaves the call. - `warm-transfer-wait-for-operator-to-speak-first-and-then-say-message`: The assistant dials the destination, waits for the operator to speak, delivers the `message` to the destination party, and then connects the customer. - `warm-transfer-wait-for-operator-to-speak-first-and-then-say-summary`: The assistant dials the destination, waits for the operator to speak, provides a summary of the call to the destination party, and then connects the customer. - `warm-transfer-twiml`: The assistant dials the destination, executes the twiml instructions on the destination call leg, connects the customer, and leaves the call.  @default 'blind-transfer'
    #[serde(rename = "mode")]
    pub mode: Mode,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<models::TransferPlanMessage>,
    /// This specifies the SIP verb to use while transferring the call. - 'refer': Uses SIP REFER to transfer the call (default) - 'bye': Ends current call with SIP BYE - 'dial': Uses SIP DIAL to transfer the call
    #[serde(rename = "sipVerb", skip_serializing_if = "Option::is_none")]
    pub sip_verb: Option<SipVerb>,
    /// This is the TwiML instructions to execute on the destination call leg before connecting the customer.  Usage: - Used only when `mode` is `warm-transfer-twiml`. - Supports only `Play`, `Say`, `Gather`, `Hangup` and `Pause` verbs. - Maximum length is 4096 characters.  Example: ``` <Say voice=\"alice\" language=\"en-US\">Hello, transferring a customer to you.</Say> <Pause length=\"2\"/> <Say>They called about billing questions.</Say> ```
    #[serde(rename = "twiml", skip_serializing_if = "Option::is_none")]
    pub twiml: Option<String>,
    /// This is the plan for generating a summary of the call to present to the destination party.  Usage: - Used only when `mode` is `blind-transfer-add-summary-to-sip-header` or `warm-transfer-say-summary` or `warm-transfer-wait-for-operator-to-speak-first-and-then-say-summary`.
    #[serde(rename = "summaryPlan", skip_serializing_if = "Option::is_none")]
    pub summary_plan: Option<models::SummaryPlan>,
}

impl TransferPlan {
    pub fn new(mode: Mode) -> TransferPlan {
        TransferPlan {
            mode,
            message: None,
            sip_verb: None,
            twiml: None,
            summary_plan: None,
        }
    }
}
/// This configures how transfer is executed and the experience of the destination party receiving the call.  Usage: - `blind-transfer`: The assistant forwards the call to the destination without any message or summary. - `blind-transfer-add-summary-to-sip-header`: The assistant forwards the call to the destination and adds a SIP header X-Transfer-Summary to the call to include the summary. - `warm-transfer-say-message`: The assistant dials the destination, delivers the `message` to the destination party, connects the customer, and leaves the call. - `warm-transfer-say-summary`: The assistant dials the destination, provides a summary of the call to the destination party, connects the customer, and leaves the call. - `warm-transfer-wait-for-operator-to-speak-first-and-then-say-message`: The assistant dials the destination, waits for the operator to speak, delivers the `message` to the destination party, and then connects the customer. - `warm-transfer-wait-for-operator-to-speak-first-and-then-say-summary`: The assistant dials the destination, waits for the operator to speak, provides a summary of the call to the destination party, and then connects the customer. - `warm-transfer-twiml`: The assistant dials the destination, executes the twiml instructions on the destination call leg, connects the customer, and leaves the call.  @default 'blind-transfer'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "blind-transfer")]
    BlindTransfer,
    #[serde(rename = "blind-transfer-add-summary-to-sip-header")]
    BlindTransferAddSummaryToSipHeader,
    #[serde(rename = "warm-transfer-say-message")]
    WarmTransferSayMessage,
    #[serde(rename = "warm-transfer-say-summary")]
    WarmTransferSaySummary,
    #[serde(rename = "warm-transfer-twiml")]
    WarmTransferTwiml,
    #[serde(rename = "warm-transfer-wait-for-operator-to-speak-first-and-then-say-message")]
    WarmTransferWaitForOperatorToSpeakFirstAndThenSayMessage,
    #[serde(rename = "warm-transfer-wait-for-operator-to-speak-first-and-then-say-summary")]
    WarmTransferWaitForOperatorToSpeakFirstAndThenSaySummary,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::BlindTransfer
    }
}
/// This specifies the SIP verb to use while transferring the call. - 'refer': Uses SIP REFER to transfer the call (default) - 'bye': Ends current call with SIP BYE - 'dial': Uses SIP DIAL to transfer the call
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SipVerb {
    #[serde(rename = "refer")]
    Refer,
    #[serde(rename = "bye")]
    Bye,
    #[serde(rename = "dial")]
    Dial,
}

impl Default for SipVerb {
    fn default() -> SipVerb {
        Self::Refer
    }
}

