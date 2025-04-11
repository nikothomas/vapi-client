/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvoicePlan {
    /// This is the name of the company.
    #[serde(rename = "companyName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<Option<String>>,
    /// This is the address of the company.
    #[serde(rename = "companyAddress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub company_address: Option<Option<String>>,
    /// This is the tax ID of the company.
    #[serde(rename = "companyTaxId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub company_tax_id: Option<Option<String>>,
    /// This is the preferred invoicing email of the company. If not specified, defaults to the subscription's email.
    #[serde(rename = "companyEmail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub company_email: Option<Option<String>>,
}

impl InvoicePlan {
    pub fn new() -> InvoicePlan {
        InvoicePlan {
            company_name: None,
            company_address: None,
            company_tax_id: None,
            company_email: None,
        }
    }
}

