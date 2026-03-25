CREATE TABLE buildings (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    planet_id   UUID NOT NULL REFERENCES planets(id) ON DELETE CASCADE,
    building_id TEXT NOT NULL,
    level       INT NOT NULL DEFAULT 0,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(planet_id, building_id)
);

CREATE TABLE building_queues (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    planet_id    UUID NOT NULL REFERENCES planets(id) ON DELETE CASCADE,
    building_id  TEXT NOT NULL,
    level        INT NOT NULL,
    started_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completes_at TIMESTAMPTZ NOT NULL,
    UNIQUE(planet_id)
);

CREATE INDEX idx_buildings_planet       ON buildings(planet_id);
CREATE INDEX idx_building_queues_planet ON building_queues(planet_id);
CREATE INDEX idx_building_queues_time   ON building_queues(completes_at);
