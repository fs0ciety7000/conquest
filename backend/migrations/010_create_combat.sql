CREATE TABLE combat_reports (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    attacker_id    UUID REFERENCES empires(id),
    defender_id    UUID REFERENCES empires(id),
    planet_id      UUID REFERENCES planets(id),
    outcome        TEXT NOT NULL,
    rounds         JSONB NOT NULL DEFAULT '[]',
    loot           JSONB,
    debris         JSONB,
    attacker_read  BOOLEAN DEFAULT FALSE,
    defender_read  BOOLEAN DEFAULT FALSE,
    created_at     TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE debris_fields (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    galaxy     SMALLINT NOT NULL,
    system     SMALLINT NOT NULL,
    position   SMALLINT NOT NULL,
    metal      NUMERIC(20,2) DEFAULT 0,
    crystal    NUMERIC(20,2) DEFAULT 0,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_combat_attacker ON combat_reports(attacker_id);
CREATE INDEX idx_combat_defender ON combat_reports(defender_id);
CREATE INDEX idx_debris_location ON debris_fields(galaxy, system, position);
CREATE INDEX idx_debris_expires  ON debris_fields(expires_at);
