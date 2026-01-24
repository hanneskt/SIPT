use std::collections::HashMap;

use reqwest::Client;

use crate::inventory::models::{SteamAsset, SteamDescription, SteamInventory};

pub async fn get_items(user_id: &str) -> (HashMap<(u64, u64), SteamDescription>, Vec<SteamAsset>) {
    let client = Client::new();
    let url = format!(
        "https://steamcommunity.com/inventory/{user_id}/730/2?l=english&count=1000&preserve_bbcode=1&raw_asset_properties=1"
    );

    let resp = client.get(url).send().await.unwrap();

    let inv: SteamInventory = resp.json().await.unwrap();

    let mut desc_hm = HashMap::new();
    for desc in inv.descriptions {
        desc_hm.insert((desc.classid, desc.instanceid), desc);
    }

    (desc_hm, inv.assets)
}
