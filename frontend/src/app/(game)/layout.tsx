"use client";

import { useEffect } from "react";
import { useRouter, usePathname } from "next/navigation";
import Link from "next/link";
import { useAuthStore } from "@/stores/authStore";
import { ResourceBar } from "@/components/ui/ResourceBar";
import { PlanetSelector } from "@/components/game/PlanetSelector";
import { useGameStore } from "@/stores/gameStore";

const navItems = [
  { href: "/overview", label: "Overview", icon: "🏠" },
  { href: "/galaxy", label: "Galaxy Map", icon: "🌌" },
  { href: "/buildings", label: "Buildings", icon: "🏗️" },
  { href: "/fleet", label: "Fleet", icon: "🚀" },
  { href: "/research", label: "Research", icon: "🔬" },
];

export default function GameLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const router = useRouter();
  const pathname = usePathname();
  const { token, user, logout } = useAuthStore();
  const currentPlanet = useGameStore((s) => s.currentPlanet);

  useEffect(() => {
    if (!token) {
      router.push("/login");
    }
  }, [token, router]);

  if (!token) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-text-secondary">Redirecting...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-space flex flex-col">
      {/* Top bar */}
      <header className="bg-surface border-b border-border sticky top-0 z-40">
        <div className="flex items-center justify-between px-4 h-14">
          {/* Logo */}
          <Link href="/overview" className="text-primary font-bold text-lg shrink-0">
            Space Conquest
          </Link>

          {/* Resource bar */}
          {currentPlanet && (
            <div className="flex-1 px-4 hidden md:block">
              <ResourceBar planetId={currentPlanet.id} />
            </div>
          )}

          {/* Planet selector + user */}
          <div className="flex items-center gap-3 shrink-0">
            <PlanetSelector />
            <span className="text-text-secondary text-sm hidden sm:block">
              {user?.username}
            </span>
            <button
              onClick={() => {
                logout();
                router.push("/");
              }}
              className="text-text-muted hover:text-danger text-sm transition-colors"
            >
              Logout
            </button>
          </div>
        </div>

        {/* Mobile resource bar */}
        {currentPlanet && (
          <div className="px-4 pb-2 md:hidden">
            <ResourceBar planetId={currentPlanet.id} />
          </div>
        )}
      </header>

      <div className="flex flex-1">
        {/* Sidebar */}
        <nav className="w-48 bg-surface border-r border-border hidden lg:flex flex-col py-4 shrink-0">
          {navItems.map((item) => {
            const active = pathname === item.href || pathname.startsWith(item.href + "/");
            return (
              <Link
                key={item.href}
                href={item.href}
                className={`flex items-center gap-3 px-4 py-3 text-sm font-medium transition-colors ${
                  active
                    ? "text-primary bg-primary/10 border-r-2 border-primary"
                    : "text-text-secondary hover:text-text-primary hover:bg-surface-hover"
                }`}
              >
                <span>{item.icon}</span>
                <span>{item.label}</span>
              </Link>
            );
          })}
        </nav>

        {/* Mobile bottom nav */}
        <nav className="fixed bottom-0 left-0 right-0 bg-surface border-t border-border flex lg:hidden z-40">
          {navItems.map((item) => {
            const active = pathname === item.href || pathname.startsWith(item.href + "/");
            return (
              <Link
                key={item.href}
                href={item.href}
                className={`flex-1 flex flex-col items-center py-2 text-xs transition-colors ${
                  active ? "text-primary" : "text-text-muted"
                }`}
              >
                <span className="text-lg">{item.icon}</span>
                <span className="mt-0.5">{item.label}</span>
              </Link>
            );
          })}
        </nav>

        {/* Main content */}
        <main className="flex-1 p-4 lg:p-6 pb-20 lg:pb-6 overflow-auto">
          {children}
        </main>
      </div>
    </div>
  );
}
