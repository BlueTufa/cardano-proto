use blockfrost::Error;
use blockfrost::{load, AssetPolicy, BlockFrostApi, BlockFrostSettings};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub name: String,
    pub src: String,
}

pub fn build_api() -> blockfrost::Result<BlockFrostApi> {
    // this is supposed to load from an env var automatically,
    // but does not appear to be working.  Temporarily use env::var directly.
    let _configurations = load::configurations_from_env()?;
    let project_id = env::var("BLOCKFROST_PROJECT_ID").unwrap();
    let settings = BlockFrostSettings::new();
    let api = BlockFrostApi::new(project_id, settings);
    Ok(api)
}

pub async fn collect_cover_images(asset: &str) -> Result<File, String> {
    let api = build_api().expect("Api was not initialized");
    let asset_details = api.assets_by_id(&asset).await.expect("Unable to retrieve asset");
    if let Some(metadata) = asset_details.onchain_metadata {
        if let Some(cover) = metadata.get("files") {
            let files: Vec<File> = serde_json::from_value(cover.to_owned()).unwrap();
            if let Some(file) = files.iter().find(|c| c.name == "High-Res Cover Image") {
                return Ok(file.to_owned());
            }
        }
    }
    Err(format!("Unable to retrieve asset: {:?}", asset))
}

pub async fn get_assets_by_policy_id(policy_id: &str) -> Result<Vec<AssetPolicy>, Error> {
    let api = build_api()?;
    Ok(api.assets_policy_by_id(policy_id).await?)
}
