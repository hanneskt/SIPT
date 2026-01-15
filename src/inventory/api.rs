use std::collections::HashMap;

use reqwest::Client;
use tracing::info;

use crate::inventory::models::{Item, SteamInventory};

pub async fn get_items(user_id: &str) -> Vec<Item> {
    let client = Client::new();
    let url = format!(
        "https://steamcommunity.com/inventory/{user_id}/730/2?l=english&count=1000&preserve_bbcode=1&raw_asset_properties=1"
    );

    let resp = client.get(url).send().await.unwrap();

    let inv: SteamInventory = resp.json().await.unwrap();

    let mut desc_hm = HashMap::new();
    for desc in &inv.descriptions {
        desc_hm.insert((desc.classid, desc.instanceid), desc);
    }

    let mut items = Vec::new();
    for asset in inv.assets {
        let desc = desc_hm.get(&(asset.classid, asset.instanceid)).unwrap();
        items.push(Item {
            appid: asset.appid,
            classid: asset.classid,
            instanceid: asset.instanceid,
            amount: asset.amount,
            icon_url: desc.icon_url.clone(),
            name: desc.name.clone(),
            market_hash_name: desc.market_hash_name.clone(),
            marketable: desc.marketable,
        });
    }

    info!("Found {} items", items.len());
    items.sort_by_key(|i| i.name.clone());
    items
}
