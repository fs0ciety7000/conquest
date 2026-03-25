import { create } from "zustand";
import { persist } from "zustand/middleware";
import { Empire, Planet } from "@/lib/types";

interface GameState {
  currentPlanet: Planet | null;
  empire: Empire | null;
  setPlanet: (planet: Planet) => void;
  setEmpire: (empire: Empire, planets: Planet[]) => void;
  clearGame: () => void;
}

export const useGameStore = create<GameState>()(
  persist(
    (set) => ({
      currentPlanet: null,
      empire: null,

      setPlanet: (planet) => {
        set({ currentPlanet: planet });
      },

      setEmpire: (empire, planets) => {
        set((state) => ({
          empire,
          // Keep current planet if it's still in the list, otherwise pick first
          currentPlanet:
            planets.find((p) => p.id === state.currentPlanet?.id) ??
            planets[0] ??
            null,
        }));
      },

      clearGame: () => {
        set({ currentPlanet: null, empire: null });
      },
    }),
    {
      name: "conquest-game",
      partialize: (state) => ({
        currentPlanet: state.currentPlanet,
        empire: state.empire,
      }),
    }
  )
);
