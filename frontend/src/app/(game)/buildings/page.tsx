"use client";

import { useGameStore } from "@/stores/gameStore";
import { useBuildings } from "@/lib/queries";
import { BuildingCard } from "@/components/game/BuildingCard";

export default function BuildingsPage() {
  const currentPlanet = useGameStore((s) => s.currentPlanet);
  const { data: buildings, isLoading, error, refetch } = useBuildings(currentPlanet?.id ?? null);

  if (!currentPlanet) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-text-secondary">No planet selected.</p>
      </div>
    );
  }

  const hasQueue = buildings?.some((b) => b.is_upgrading) ?? false;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-text-primary">Buildings</h1>
        <p className="text-text-secondary text-sm mt-1">
          Manage structures on {currentPlanet.name}
        </p>
      </div>

      {isLoading ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <div key={i} className="card p-5 animate-pulse space-y-3">
              <div className="h-5 bg-surface-hover rounded w-3/4" />
              <div className="h-4 bg-surface-hover rounded w-1/2" />
              <div className="h-16 bg-surface-hover rounded" />
              <div className="h-9 bg-surface-hover rounded" />
            </div>
          ))}
        </div>
      ) : error ? (
        <div className="card p-6 text-center">
          <p className="text-danger mb-3">Failed to load buildings.</p>
          <button
            onClick={() => refetch()}
            className="text-primary text-sm hover:underline"
          >
            Try again
          </button>
        </div>
      ) : buildings && buildings.length > 0 ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4">
          {buildings.map((building) => (
            <BuildingCard
              key={building.building_id}
              building={building}
              planetId={currentPlanet.id}
              queueFull={hasQueue}
              onUpgradeSuccess={() => refetch()}
            />
          ))}
        </div>
      ) : (
        <div className="card p-8 text-center">
          <p className="text-text-muted">No buildings found on this planet.</p>
        </div>
      )}
    </div>
  );
}
