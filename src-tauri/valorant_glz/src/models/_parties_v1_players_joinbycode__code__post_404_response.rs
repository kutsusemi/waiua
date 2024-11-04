/*
 * Valorant API
 *
 * Valorant API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartiesV1PlayersJoinbycodeCodePost404Response {
    #[serde(rename = "httpStatus")]
    pub http_status: HttpStatus,
    #[serde(rename = "errorCode")]
    pub error_code: ErrorCode,
    #[serde(rename = "message")]
    pub message: Message,
}

impl PartiesV1PlayersJoinbycodeCodePost404Response {
    pub fn new(http_status: HttpStatus, error_code: ErrorCode, message: Message) -> PartiesV1PlayersJoinbycodeCodePost404Response {
        PartiesV1PlayersJoinbycodeCodePost404Response {
            http_status,
            error_code,
            message,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpStatus {
    #[serde(rename = "404")]
    Variant404,
}

impl Default for HttpStatus {
    fn default() -> HttpStatus {
        Self::Variant404
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "ERR_MISSING_INVITE_CODE_MAPPING")]
    ErrMissingInviteCodeMapping,
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        Self::ErrMissingInviteCodeMapping
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Message {
    #[serde(rename = "No PartyID <--> InviteCode mapping found")]
    NoPartyIdLessThanGreaterThanInviteCodeMappingFound,
}

impl Default for Message {
    fn default() -> Message {
        Self::NoPartyIdLessThanGreaterThanInviteCodeMappingFound
    }
}

