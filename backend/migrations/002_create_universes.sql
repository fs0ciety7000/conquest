CREATE TABLE universes (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name              TEXT UNIQUE NOT NULL,
    speed_multiplier  SMALLINT NOT NULL DEFAULT 1,
    fleet_speed       SMALLINT NOT NULL DEFAULT 1,
    economy_speed     SMALLINT NOT NULL DEFAULT 1,
    research_speed    SMALLINT NOT NULL DEFAULT 1,
    max_players       INT NOT NULL DEFAULT 3000,
    is_active         BOOLEAN NOT NULL DEFAULT TRUE,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_universes_active ON universes(is_active);

INSERT INTO universes (id, name, speed_multiplier, fleet_speed, economy_speed, research_speed)
VALUES ('00000000-0000-0000-0000-000000000001', 'Dev Universe', 2, 2, 2, 2)
ON CONFLICT DO NOTHING;
