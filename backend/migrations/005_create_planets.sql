CREATE TABLE planets (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id            UUID NOT NULL REFERENCES empires(id) ON DELETE CASCADE,
    universe_id          UUID NOT NULL REFERENCES universes(id) ON DELETE CASCADE,
    name                 TEXT NOT NULL,
    galaxy               SMALLINT NOT NULL CHECK (galaxy BETWEEN 1 AND 9),
    system               SMALLINT NOT NULL CHECK (system BETWEEN 1 AND 499),
    position             SMALLINT NOT NULL CHECK (position BETWEEN 1 AND 15),
    diameter             INT NOT NULL DEFAULT 10000,
    temperature_min      SMALLINT NOT NULL DEFAULT 0,
    temperature_max      SMALLINT NOT NULL DEFAULT 40,
    image_id             SMALLINT NOT NULL DEFAULT 1,
    metal                DOUBLE PRECISION NOT NULL DEFAULT 500,
    crystal              DOUBLE PRECISION NOT NULL DEFAULT 300,
    deuterium            DOUBLE PRECISION NOT NULL DEFAULT 200,
    energy               INT NOT NULL DEFAULT 0,
    last_resource_update TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_homeworld         BOOLEAN NOT NULL DEFAULT FALSE,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(universe_id, galaxy, system, position)
);

CREATE INDEX idx_planets_empire   ON planets(empire_id);
CREATE INDEX idx_planets_location ON planets(universe_id, galaxy, system, position);
