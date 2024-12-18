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

/// RiotGeoPasSiRiotgamesComPasV1ProductValorantPut200ResponseAffinities : The region IDs for PBE and live servers
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RiotGeoPasSiRiotgamesComPasV1ProductValorantPut200ResponseAffinities {
    #[serde(rename = "pbe")]
    pub pbe: String,
    #[serde(rename = "live")]
    pub live: String,
}

impl RiotGeoPasSiRiotgamesComPasV1ProductValorantPut200ResponseAffinities {
    /// The region IDs for PBE and live servers
    pub fn new(
        pbe: String,
        live: String,
    ) -> RiotGeoPasSiRiotgamesComPasV1ProductValorantPut200ResponseAffinities {
        RiotGeoPasSiRiotgamesComPasV1ProductValorantPut200ResponseAffinities { pbe, live }
    }
}
