use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug)]
pub struct Item {
    pub appid: u32,
    pub classid: u64,
    pub instanceid: u64,
    pub amount: u32,
    pub icon_url: String,
    pub name: String,
    pub market_hash_name: String,
    pub marketable: u8,
}

#[derive(Debug, Deserialize)]
pub struct SteamInventory {
    pub assets: Vec<SteamAsset>,
    pub descriptions: Vec<SteamDescription>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SteamAsset {
    pub appid: u32,
    pub contextid: String,
    #[serde_as(as = "DisplayFromStr")]
    /// specific item, globally unique
    pub assetid: u64,
    #[serde_as(as = "DisplayFromStr")]
    /// kind of item
    pub classid: u64,
    #[serde_as(as = "DisplayFromStr")]
    /// variant of item
    pub instanceid: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: u32,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SteamDescription {
    pub appid: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub classid: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub instanceid: u64,
    pub currency: u32,
    pub background_color: String,
    pub icon_url: String,
    pub tradable: u8,
    pub name: String,
    pub name_color: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub market_name: String,
    pub market_hash_name: String,
    pub commodity: u8,
    pub marketable: u8,
    pub sealed: u8,
}
