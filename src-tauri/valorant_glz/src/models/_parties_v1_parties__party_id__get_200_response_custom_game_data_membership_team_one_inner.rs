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
pub struct PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembershipTeamOneInner {
    /// Player UUID
    #[serde(rename = "Subject")]
    pub subject: String,
}

impl PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembershipTeamOneInner {
    pub fn new(
        subject: String,
    ) -> PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembershipTeamOneInner {
        PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembershipTeamOneInner { subject }
    }
}
