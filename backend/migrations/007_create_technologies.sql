CREATE TABLE technologies (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id     UUID REFERENCES empires(id) ON DELETE CASCADE,
    technology_id TEXT NOT NULL,
    level         INT NOT NULL DEFAULT 0,
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(empire_id, technology_id)
);

CREATE TABLE research_queues (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id     UUID REFERENCES empires(id) ON DELETE CASCADE,
    planet_id     UUID REFERENCES planets(id) ON DELETE CASCADE,
    technology_id TEXT NOT NULL,
    level         INT NOT NULL,
    started_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completes_at  TIMESTAMPTZ NOT NULL,
    UNIQUE(empire_id)
);

CREATE INDEX idx_technologies_empire    ON technologies(empire_id);
CREATE INDEX idx_research_queues_empire ON research_queues(empire_id);
CREATE INDEX idx_research_queues_time   ON research_queues(completes_at);
