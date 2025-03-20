/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SipTrunkGateway {
    /// This is the address of the gateway. It can be an IPv4 address like 1.1.1.1 or a fully qualified domain name like my-sip-trunk.pstn.twilio.com.
    #[serde(rename = "ip")]
    pub ip: String,
    /// This is the port number of the gateway. Default is 5060.  @default 5060
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,
    /// This is the netmask of the gateway. Defaults to 32.  @default 32
    #[serde(rename = "netmask", skip_serializing_if = "Option::is_none")]
    pub netmask: Option<f64>,
    /// This is whether inbound calls are allowed from this gateway. Default is true.  @default true
    #[serde(rename = "inboundEnabled", skip_serializing_if = "Option::is_none")]
    pub inbound_enabled: Option<bool>,
    /// This is whether outbound calls should be sent to this gateway. Default is true.  Note, if netmask is less than 32, it doesn't affect the outbound IPs that are tried. 1 attempt is made to `ip:port`.  @default true
    #[serde(rename = "outboundEnabled", skip_serializing_if = "Option::is_none")]
    pub outbound_enabled: Option<bool>,
    /// This is the protocol to use for SIP signaling outbound calls. Default is udp.  @default udp
    #[serde(rename = "outboundProtocol", skip_serializing_if = "Option::is_none")]
    pub outbound_protocol: Option<OutboundProtocol>,
    /// This is whether to send options ping to the gateway. This can be used to check if the gateway is reachable. Default is false.  This is useful for high availability setups where you want to check if the gateway is reachable before routing calls to it. Note, if no gateway for a trunk is reachable, outbound calls will be rejected.  @default false
    #[serde(rename = "optionsPingEnabled", skip_serializing_if = "Option::is_none")]
    pub options_ping_enabled: Option<bool>,
}

impl SipTrunkGateway {
    pub fn new(ip: String) -> SipTrunkGateway {
        SipTrunkGateway {
            ip,
            port: None,
            netmask: None,
            inbound_enabled: None,
            outbound_enabled: None,
            outbound_protocol: None,
            options_ping_enabled: None,
        }
    }
}
/// This is the protocol to use for SIP signaling outbound calls. Default is udp.  @default udp
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutboundProtocol {
    #[serde(rename = "tls/srtp")]
    TlsSlashSrtp,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "tls")]
    Tls,
    #[serde(rename = "udp")]
    Udp,
}

impl Default for OutboundProtocol {
    fn default() -> OutboundProtocol {
        Self::TlsSlashSrtp
    }
}
