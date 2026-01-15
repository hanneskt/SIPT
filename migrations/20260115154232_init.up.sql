CREATE TABLE item_info (
    id INTEGER PRIMARY KEY,

    app_id INTEGER NOT NULL,
    class_id INTEGER NOT NULL,
    instance_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    market_hash_name TEXT NOT NULL,
    icon_url TEXT NOT NULL,
    marketable BOOLEAN DEFAULT TRUE,

    UNIQUE (class_id, instance_id)
);

CREATE TABLE inventory_item (
    asset_id INTEGER PRIMARY KEY,

    active BOOLEAN DEFAULT TRUE, -- currently in inventory
    acquired_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    info_id INTEGER NOT NULL,

    FOREIGN KEY (info_id) REFERENCES item_info (id),
);

CREATE TABLE price (
    id INTEGER PRIMARY KEY,
    info_id INTEGER NOT NULL,

    price_cents INTEGER NOT NULL,
    currency_code INTEGER DEFAULT 3, -- 3 = EUR, 1 = USD

    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP

    FOREIGN KEY (info_id) REFERENCES item_info (id),
);

CREATE INDEX idx_price_history_def_time ON price_history(info_id, time DESC);
