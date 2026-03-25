"use client";

import { useGameStore } from "@/stores/gameStore";
import { useResources, useBuildings } from "@/lib/queries";
import { Card } from "@/components/ui/Card";

function ResourceItem({
  label,
  value,
  production,
  color,
  icon,
}: {
  label: string;
  value: number;
  production: number;
  color: string;
  icon: string;
}) {
  return (
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-2">
        <span>{icon}</span>
        <span className={`font-medium ${color}`}>{label}</span>
      </div>
      <div className="text-right">
        <div className="text-text-primary font-semibold">
          {Math.floor(value).toLocaleString()}
        </div>
        <div className="text-text-muted text-xs">+{production.toLocaleString()}/h</div>
      </div>
    </div>
  );
}

export default function OverviewPage() {
  const currentPlanet = useGameStore((s) => s.currentPlanet);
  const empire = useGameStore((s) => s.empire);

  const { data: resources, isLoading: resourcesLoading } = useResources(currentPlanet?.id ?? null);
  const { data: buildings, isLoading: buildingsLoading } = useBuildings(currentPlanet?.id ?? null);

  const buildingQueue = buildings?.filter((b) => b.is_upgrading) ?? [];

  if (!currentPlanet) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-text-secondary">No planet selected. Loading empire...</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Page header */}
      <div>
        <h1 className="text-2xl font-bold text-text-primary">{currentPlanet.name}</h1>
        <p className="text-text-secondary text-sm mt-1">
          Galaxy {currentPlanet.galaxy} · System {currentPlanet.solar_system} ·
          Position {currentPlanet.position}
          {empire && (
            <span className="ml-2 text-text-muted">— Empire: {empire.name}</span>
          )}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
        {/* Resources card */}
        <Card title="Resources" className="md:col-span-1">
          {resourcesLoading ? (
            <div className="space-y-3 animate-pulse">
              {[1, 2, 3, 4].map((i) => (
                <div key={i} className="h-8 bg-surface-hover rounded" />
              ))}
            </div>
          ) : resources ? (
            <div className="space-y-4">
              <ResourceItem
                label="Metal"
                value={resources.metal}
                production={resources.metal_production}
                color="text-metal"
                icon="⚙️"
              />
              <ResourceItem
                label="Crystal"
                value={resources.crystal}
                production={resources.crystal_production}
                color="text-crystal"
                icon="💎"
              />
              <ResourceItem
                label="Deuterium"
                value={resources.deuterium}
                production={resources.deuterium_production}
                color="text-deuterium"
                icon="⚗️"
              />
              <div className="border-t border-border pt-3">
                <ResourceItem
                  label="Energy"
                  value={resources.energy}
                  production={resources.energy_production}
                  color="text-energy"
                  icon="⚡"
                />
              </div>
            </div>
          ) : (
            <p className="text-text-muted text-sm">Failed to load resources.</p>
          )}
        </Card>

        {/* Building queue */}
        <Card title="Construction Queue" className="md:col-span-1">
          {buildingsLoading ? (
            <div className="space-y-3 animate-pulse">
              {[1, 2].map((i) => (
                <div key={i} className="h-12 bg-surface-hover rounded" />
              ))}
            </div>
          ) : buildingQueue.length > 0 ? (
            <div className="space-y-3">
              {buildingQueue.map((b) => (
                <div
                  key={b.building_id}
                  className="flex items-center justify-between p-3 bg-background rounded-lg border border-primary/20"
                >
                  <div>
                    <p className="text-text-primary text-sm font-medium">
                      {b.name}
                    </p>
                    <p className="text-text-muted text-xs">
                      Upgrading to level {b.level + 1}
                    </p>
                  </div>
                  <div className="w-2 h-2 rounded-full bg-primary animate-pulse-slow" />
                </div>
              ))}
            </div>
          ) : (
            <p className="text-text-muted text-sm">No buildings in queue.</p>
          )}
        </Card>

        {/* Planet info */}
        <Card title="Planet Info">
          <div className="space-y-3 text-sm">
            {[
              { label: "Diameter", value: currentPlanet.diameter ? `${currentPlanet.diameter.toLocaleString()} km` : "—" },
              { label: "Temperature", value: currentPlanet.temperature ? `${currentPlanet.temperature}°C` : "—" },
              { label: "Fields Used", value: currentPlanet.fields_used !== undefined ? `${currentPlanet.fields_used} / ${currentPlanet.fields_total ?? "?"}` : "—" },
            ].map(({ label, value }) => (
              <div key={label} className="flex justify-between">
                <span className="text-text-secondary">{label}</span>
                <span className="text-text-primary font-medium">{value}</span>
              </div>
            ))}
          </div>
        </Card>
      </div>
    </div>
  );
}
