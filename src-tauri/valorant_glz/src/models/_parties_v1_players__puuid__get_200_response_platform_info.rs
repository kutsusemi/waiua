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
pub struct PartiesV1PlayersPuuidGet200ResponsePlatformInfo {
    #[serde(rename = "platformType")]
    pub platform_type: PlatformType,
    #[serde(rename = "platformOS")]
    pub platform_os: PlatformOs,
    #[serde(rename = "platformOSVersion")]
    pub platform_os_version: String,
    #[serde(rename = "platformChipset")]
    pub platform_chipset: PlatformChipset,
}

impl PartiesV1PlayersPuuidGet200ResponsePlatformInfo {
    pub fn new(platform_type: PlatformType, platform_os: PlatformOs, platform_os_version: String, platform_chipset: PlatformChipset) -> PartiesV1PlayersPuuidGet200ResponsePlatformInfo {
        PartiesV1PlayersPuuidGet200ResponsePlatformInfo {
            platform_type,
            platform_os,
            platform_os_version,
            platform_chipset,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformType {
    #[serde(rename = "PC")]
    Pc,
}

impl Default for PlatformType {
    fn default() -> PlatformType {
        Self::Pc
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformOs {
    #[serde(rename = "Windows")]
    Windows,
}

impl Default for PlatformOs {
    fn default() -> PlatformOs {
        Self::Windows
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformChipset {
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for PlatformChipset {
    fn default() -> PlatformChipset {
        Self::Unknown
    }
}

