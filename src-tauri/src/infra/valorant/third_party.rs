use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponse {
    pub status: i64,
    pub data: VersionResponseData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponseData {
    pub manifest_id: String,
    pub branch: String,
    pub version: String,
    pub build_version: String,
    pub engine_version: String,
    pub riot_client_version: String,
    pub riot_client_build: String,
    pub build_date: String,
}

pub async fn version() -> Result<VersionResponse, reqwest::Error> {
    let url = "https://valorant-api.com/v1/version";
    let resp = reqwest::get(url).await?.json().await?;
    Ok(resp)
}
