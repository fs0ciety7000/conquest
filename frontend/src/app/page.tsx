import Link from "next/link";

export default function Home() {
  return (
    <main className="min-h-screen bg-space flex flex-col items-center justify-center px-4">
      {/* Star decorations */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute top-1/4 left-1/4 w-1 h-1 bg-primary rounded-full opacity-60 animate-pulse-slow" />
        <div className="absolute top-1/3 right-1/3 w-1 h-1 bg-crystal rounded-full opacity-40 animate-pulse-slow" style={{ animationDelay: "1s" }} />
        <div className="absolute bottom-1/3 left-1/3 w-1 h-1 bg-primary rounded-full opacity-50 animate-pulse-slow" style={{ animationDelay: "2s" }} />
        <div className="absolute top-2/3 right-1/4 w-1 h-1 bg-deuterium rounded-full opacity-40 animate-pulse-slow" style={{ animationDelay: "0.5s" }} />
      </div>

      <div className="relative z-10 text-center max-w-2xl mx-auto">
        {/* Logo / Title */}
        <div className="mb-4">
          <span className="text-6xl">🌌</span>
        </div>
        <h1 className="text-5xl md:text-7xl font-bold mb-4 tracking-tight">
          <span className="text-text-primary">Space</span>{" "}
          <span className="text-primary glow-primary">Conquest</span>
        </h1>
        <p className="text-text-secondary text-lg md:text-xl mb-12 leading-relaxed">
          Build your empire across the stars. Mine resources, construct fleets,
          and dominate the galaxy.
        </p>

        {/* CTA Buttons */}
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Link
            href="/login"
            className="px-8 py-3 bg-primary hover:bg-primary-hover text-background font-semibold rounded-lg transition-colors duration-200 text-lg glow-primary"
          >
            Login
          </Link>
          <Link
            href="/register"
            className="px-8 py-3 bg-surface hover:bg-surface-hover text-text-primary font-semibold rounded-lg border border-border hover:border-primary transition-colors duration-200 text-lg"
          >
            Register
          </Link>
        </div>

        {/* Feature highlights */}
        <div className="mt-20 grid grid-cols-1 sm:grid-cols-3 gap-6 text-left">
          {[
            {
              icon: "⚒️",
              title: "Mine Resources",
              desc: "Harvest metal, crystal, and deuterium from your planets.",
            },
            {
              icon: "🏗️",
              title: "Build & Upgrade",
              desc: "Construct mines, factories, shipyards, and research labs.",
            },
            {
              icon: "🚀",
              title: "Conquer",
              desc: "Send fleets to explore, colonize, and conquer the galaxy.",
            },
          ].map((f) => (
            <div
              key={f.title}
              className="card p-5 hover:border-primary/40 transition-colors duration-200"
            >
              <div className="text-2xl mb-2">{f.icon}</div>
              <h3 className="text-text-primary font-semibold mb-1">{f.title}</h3>
              <p className="text-text-muted text-sm">{f.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </main>
  );
}
