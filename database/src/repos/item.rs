use crate::{
    error::DatabaseError,
    models::item::{InventoryItem, ItemInfo, ItemInfoCreate, Price},
};

use sqlx::SqlitePool;

pub struct ItemRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> ItemRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub async fn get_info_by_id(&self, id: u32) -> Result<ItemInfo, DatabaseError> {
        sqlx::query_as(
            "SELECT id, app_id, class_id, instance_id, name, market_hash_name, icon_url, marketable
             FROM item_info WHERE id = ? LIMIT 1;",
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }

    pub async fn create_or_update_info(
        &self,
        item: ItemInfoCreate,
    ) -> Result<ItemInfo, DatabaseError> {
        sqlx::query_as(
            "
            INSERT INTO item_info (app_id, class_id, instance_id, name, market_hash_name, icon_url, marketable)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(class_id, instance_id) DO UPDATE SET
                market_hash_name = excluded.market_hash_name,
                icon_url = excluded.icon_url,
                marketable = excluded.marketable
            RETURNING id, app_id, class_id, instance_id, name, market_hash_name, icon_url, marketable;
            ",
        )
        .bind(item.app_id as i64)
        .bind(item.class_id as i64)
        .bind(item.instance_id as i64)
        .bind(&item.name)
        .bind(&item.market_hash_name)
        .bind(&item.icon_url)
        .bind(item.marketable)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }

    pub async fn create_or_update_asset(
        &self,
        asset_id: u64,
        info_id: u32,
        active: bool,
    ) -> Result<InventoryItem, DatabaseError> {
        sqlx::query_as(
            "
            INSERT INTO inventory_item (asset_id, info_id, active, acquired_at)
            VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            ON CONFLICT(asset_id) DO UPDATE SET
                active = excluded.active,
                info_id = excluded.info_id
            RETURNING asset_id, active, acquired_at, info_id;
            ",
        )
        .bind(asset_id as i64)
        .bind(info_id)
        .bind(active)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }

    pub async fn add_price(
        &self,
        info_id: u32,
        price_cents: u64,
        currency_code: u8,
    ) -> Result<Price, DatabaseError> {
        sqlx::query_as(
            "
            INSERT INTO price (info_id, price_cents, currency_code, time)
            VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            RETURNING id, info_id, price_cents, currency_code, time;
            ",
        )
        .bind(info_id)
        .bind(price_cents as i64)
        .bind(currency_code)
        .fetch_optional(self.db)
        .await?
        .ok_or(DatabaseError::NotFound)
    }
}
