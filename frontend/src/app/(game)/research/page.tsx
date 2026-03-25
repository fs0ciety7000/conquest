"use client";

import { useGameData } from "@/lib/queries";
import { Card } from "@/components/ui/Card";
import { useGameStore } from "@/stores/gameStore";

export default function ResearchPage() {
  const { data: gameData, isLoading, error } = useGameData();
  const empire = useGameStore((s) => s.empire);

  const technologies = gameData?.technologies ?? [];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-text-primary">Research</h1>
        <p className="text-text-secondary text-sm mt-1">
          Advance your technology tree
        </p>
      </div>

      {isLoading ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4">
          {Array.from({ length: 9 }).map((_, i) => (
            <div key={i} className="card p-5 animate-pulse space-y-3">
              <div className="h-5 bg-surface-hover rounded w-3/4" />
              <div className="h-4 bg-surface-hover rounded w-full" />
              <div className="h-4 bg-surface-hover rounded w-2/3" />
            </div>
          ))}
        </div>
      ) : error ? (
        <div className="card p-6 text-center">
          <p className="text-danger">Failed to load technology data.</p>
        </div>
      ) : technologies.length > 0 ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4">
          {technologies.map((tech) => {
            const currentLevel =
              empire?.technologies?.[tech.id] ?? 0;

            return (
              <Card key={tech.id} className="hover:border-primary/30 transition-colors">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <h3 className="text-text-primary font-semibold">{tech.name}</h3>
                    <p className="text-text-muted text-xs mt-0.5">{tech.category}</p>
                  </div>
                  <span className="text-primary font-bold text-lg">
                    Lv {currentLevel}
                  </span>
                </div>

                <p className="text-text-secondary text-sm mb-4 leading-relaxed">
                  {tech.description}
                </p>

                {/* Cost preview */}
                {tech.base_cost && (
                  <div className="text-xs text-text-muted space-y-1 border-t border-border pt-3">
                    <p className="text-text-secondary font-medium mb-1">
                      Next level cost:
                    </p>
                    {tech.base_cost.metal > 0 && (
                      <div className="flex justify-between">
                        <span className="text-metal">Metal</span>
                        <span>{tech.base_cost.metal.toLocaleString()}</span>
                      </div>
                    )}
                    {tech.base_cost.crystal > 0 && (
                      <div className="flex justify-between">
                        <span className="text-crystal">Crystal</span>
                        <span>{tech.base_cost.crystal.toLocaleString()}</span>
                      </div>
                    )}
                    {tech.base_cost.deuterium > 0 && (
                      <div className="flex justify-between">
                        <span className="text-deuterium">Deuterium</span>
                        <span>{tech.base_cost.deuterium.toLocaleString()}</span>
                      </div>
                    )}
                  </div>
                )}

                <button
                  className="mt-4 w-full py-2 rounded-lg text-sm font-medium bg-surface-hover hover:bg-primary/10 hover:text-primary border border-border hover:border-primary/40 text-text-secondary transition-colors"
                  disabled
                  title="Research coming soon"
                >
                  Research (coming soon)
                </button>
              </Card>
            );
          })}
        </div>
      ) : (
        <div className="card p-8 text-center">
          <p className="text-text-muted">No technologies available.</p>
        </div>
      )}
    </div>
  );
}
