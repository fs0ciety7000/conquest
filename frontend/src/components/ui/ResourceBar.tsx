"use client";

import { useResources } from "@/lib/queries";

interface ResourceBarProps {
  planetId: string;
}

function ResourceItem({
  icon,
  value,
  color,
  title,
}: {
  icon: string;
  value: number;
  color: string;
  title: string;
}) {
  return (
    <div className="flex items-center gap-1.5" title={title}>
      <span className="text-sm">{icon}</span>
      <span className={`text-sm font-medium tabular-nums ${color}`}>
        {Math.floor(value).toLocaleString()}
      </span>
    </div>
  );
}

export function ResourceBar({ planetId }: ResourceBarProps) {
  const { data: resources } = useResources(planetId ?? null);

  if (!resources) {
    return (
      <div className="flex gap-4 animate-pulse">
        {[1, 2, 3, 4].map((i) => (
          <div key={i} className="h-5 w-20 bg-surface-hover rounded" />
        ))}
      </div>
    );
  }

  return (
    <div className="flex items-center gap-4 flex-wrap">
      <ResourceItem
        icon="⚙️"
        value={resources.metal}
        color="text-metal"
        title={`Metal: ${Math.floor(resources.metal).toLocaleString()} (+${resources.metal_production}/h)`}
      />
      <ResourceItem
        icon="💎"
        value={resources.crystal}
        color="text-crystal"
        title={`Crystal: ${Math.floor(resources.crystal).toLocaleString()} (+${resources.crystal_production}/h)`}
      />
      <ResourceItem
        icon="⚗️"
        value={resources.deuterium}
        color="text-deuterium"
        title={`Deuterium: ${Math.floor(resources.deuterium).toLocaleString()} (+${resources.deuterium_production}/h)`}
      />
      <ResourceItem
        icon="⚡"
        value={resources.energy}
        color="text-energy"
        title={`Energy: ${Math.floor(resources.energy).toLocaleString()} (+${resources.energy_production}/h)`}
      />
    </div>
  );
}
