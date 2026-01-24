use chrono::{DateTime, Local};
use sqlx::FromRow;

#[derive(Debug, FromRow, PartialEq)]
pub struct ItemInfo {
    pub id: u32,

    pub app_id: u64,
    pub class_id: u64,
    pub instance_id: u64,

    pub name: String,
    pub market_hash_name: String,
    pub icon_url: String,
    pub marketable: bool,
}

#[derive(Debug, FromRow, PartialEq)]
pub struct ItemInfoCreate {
    pub app_id: u64,
    pub class_id: u64,
    pub instance_id: u64,

    pub name: String,
    pub market_hash_name: String,
    pub icon_url: String,
    pub marketable: bool,
}

#[derive(Debug, FromRow, PartialEq)]
pub struct InventoryItem {
    pub asset_id: u64,

    pub active: bool,
    pub acquired_at: DateTime<Local>,

    pub info_id: u64,
}

#[derive(Debug, FromRow, PartialEq)]
pub struct Price {
    pub id: u64,
    pub info_id: u64,

    pub price_cents: u64,
    pub currency_code: u8,

    pub time: DateTime<Local>,
}
