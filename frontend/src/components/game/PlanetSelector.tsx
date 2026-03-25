"use client";

import { useGameStore } from "@/stores/gameStore";
import { useEmpire } from "@/lib/queries";
import { useAuthStore } from "@/stores/authStore";

export function PlanetSelector() {
  const { currentPlanet, setPlanet } = useGameStore();
  const user = useAuthStore((s) => s.user);

  // Use universe_id 1 as default; adjust if your backend exposes it on user
  const universeId = user?.universe_id ?? "1";
  const { data: empireData } = useEmpire(universeId);

  const planets = empireData?.planets ?? [];

  if (planets.length === 0) {
    return (
      <span className="text-text-muted text-sm">
        {currentPlanet?.name ?? "No planet"}
      </span>
    );
  }

  return (
    <select
      value={currentPlanet?.id ?? ""}
      onChange={(e) => {
        const planet = planets.find((p) => p.id === e.target.value);
        if (planet) setPlanet(planet);
      }}
      className="bg-background border border-border rounded-lg px-3 py-1.5 text-sm text-text-primary focus:outline-none focus:border-primary transition-colors cursor-pointer"
      aria-label="Select planet"
    >
      {planets.map((planet) => (
        <option key={planet.id} value={planet.id}>
          {planet.name}
        </option>
      ))}
    </select>
  );
}
