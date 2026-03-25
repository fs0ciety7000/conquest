CREATE TABLE fleets (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id        UUID REFERENCES empires(id),
    mission          TEXT NOT NULL,
    status           TEXT DEFAULT 'IDLE',
    origin_planet_id UUID REFERENCES planets(id),
    dest_galaxy      SMALLINT,
    dest_system      SMALLINT,
    dest_position    SMALLINT,
    departure_time   TIMESTAMPTZ,
    arrival_time     TIMESTAMPTZ,
    return_time      TIMESTAMPTZ,
    cargo_metal      NUMERIC(20,2) DEFAULT 0,
    cargo_crystal    NUMERIC(20,2) DEFAULT 0,
    cargo_deuterium  NUMERIC(20,2) DEFAULT 0,
    speed_percent    SMALLINT DEFAULT 100,
    is_acs           BOOLEAN DEFAULT FALSE,
    acs_group_id     UUID,
    created_at       TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE fleet_units (
    fleet_id  UUID REFERENCES fleets(id) ON DELETE CASCADE,
    ship_id   TEXT NOT NULL,
    quantity  INT  NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (fleet_id, ship_id)
);

CREATE INDEX idx_fleets_empire  ON fleets(empire_id);
CREATE INDEX idx_fleets_status  ON fleets(status);
CREATE INDEX idx_fleets_arrival ON fleets(arrival_time) WHERE status = 'OUTBOUND';
