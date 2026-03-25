CREATE TABLE market_offers (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id    UUID REFERENCES empires(id),
    offer_type   TEXT NOT NULL,
    resource     TEXT NOT NULL,
    quantity     NUMERIC(20,2) NOT NULL,
    price        NUMERIC(20,2) NOT NULL,
    status       TEXT DEFAULT 'ACTIVE',
    expires_at   TIMESTAMPTZ NOT NULL,
    created_at   TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_market_resource ON market_offers(resource, offer_type) WHERE status = 'ACTIVE';
CREATE INDEX idx_market_empire   ON market_offers(empire_id);
