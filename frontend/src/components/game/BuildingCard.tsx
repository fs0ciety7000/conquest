"use client";

import { useState } from "react";
import { BuildingStatus } from "@/lib/types";
import { api } from "@/lib/api";
import { Button } from "@/components/ui/Button";

interface BuildingCardProps {
  building: BuildingStatus;
  planetId: string;
  queueFull: boolean;
  onUpgradeSuccess: () => void;
}

export function BuildingCard({
  building,
  planetId,
  queueFull,
  onUpgradeSuccess,
}: BuildingCardProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const isUpgrading = building.is_upgrading;
  const canUpgrade = !isUpgrading && !queueFull;

  async function handleUpgrade() {
    setError(null);
    setLoading(true);
    try {
      await api.post(`/api/planets/${planetId}/buildings`, {
        building_id: building.building_id,
      });
      onUpgradeSuccess();
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to start upgrade.";
      setError(message);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="card p-5 flex flex-col gap-3 hover:border-primary/20 transition-colors">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <h3 className="text-text-primary font-semibold">{building.name}</h3>
          <p className="text-text-muted text-xs mt-0.5">{building.category}</p>
        </div>
        <div className="text-right">
          <span className="text-primary font-bold text-lg">Lv {building.level}</span>
          {isUpgrading && (
            <p className="text-warning text-xs mt-0.5 animate-pulse-slow">Building...</p>
          )}
        </div>
      </div>

      {/* Description */}
      {building.description && (
        <p className="text-text-secondary text-sm leading-relaxed">
          {building.description}
        </p>
      )}

      {/* Cost */}
      {building.upgrade_cost && (
        <div className="bg-background rounded-lg p-3 space-y-1.5">
          <p className="text-text-muted text-xs font-medium uppercase tracking-wide mb-2">
            Upgrade Cost
          </p>
          <div className="grid grid-cols-2 gap-x-4 gap-y-1 text-xs">
            {building.upgrade_cost.metal > 0 && (
              <div className="flex justify-between">
                <span className="text-metal">Metal</span>
                <span className="text-text-secondary">
                  {building.upgrade_cost.metal.toLocaleString()}
                </span>
              </div>
            )}
            {building.upgrade_cost.crystal > 0 && (
              <div className="flex justify-between">
                <span className="text-crystal">Crystal</span>
                <span className="text-text-secondary">
                  {building.upgrade_cost.crystal.toLocaleString()}
                </span>
              </div>
            )}
            {building.upgrade_cost.deuterium > 0 && (
              <div className="flex justify-between">
                <span className="text-deuterium">Deuterium</span>
                <span className="text-text-secondary">
                  {building.upgrade_cost.deuterium.toLocaleString()}
                </span>
              </div>
            )}
            {(building.upgrade_cost.energy ?? 0) > 0 && (
              <div className="flex justify-between">
                <span className="text-energy">Energy</span>
                <span className="text-text-secondary">
                  {(building.upgrade_cost.energy ?? 0).toLocaleString()}
                </span>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Error */}
      {error && (
        <p className="text-danger text-xs">{error}</p>
      )}

      {/* Upgrade button */}
      <Button
        variant={isUpgrading ? "ghost" : "secondary"}
        size="sm"
        className="w-full mt-auto"
        disabled={!canUpgrade || loading}
        onClick={handleUpgrade}
        title={
          isUpgrading
            ? "Already upgrading"
            : queueFull
            ? "Construction queue is full"
            : `Upgrade ${building.name} to level ${building.level + 1}`
        }
      >
        {isUpgrading
          ? "Upgrading..."
          : loading
          ? "Starting..."
          : queueFull
          ? "Queue Full"
          : `Upgrade to Lv ${building.level + 1}`}
      </Button>
    </div>
  );
}
