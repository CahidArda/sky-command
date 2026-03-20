import Link from "next/link";

// ---------------------------------------------------------------------------
// Landing page — simple hero with a "Start Flying" button
// ---------------------------------------------------------------------------

export default function Home() {
  return (
    <main className="flex h-screen flex-col items-center justify-center bg-gradient-to-b from-gray-950 via-gray-900 to-gray-950">
      {/* Title */}
      <h1 className="mb-2 text-5xl font-extrabold tracking-tight text-white sm:text-6xl">
        Sky<span className="text-sky-400">Command</span>
      </h1>

      <p className="mb-10 max-w-md text-center text-sm text-gray-400">
        A 3D flight simulator prototype. Fly a Cessna 172 with realistic
        aerodynamics, right in your browser.
      </p>

      {/* CTA */}
      <Link
        href="/game"
        className="rounded-lg bg-sky-500 px-8 py-3 text-lg font-semibold text-white shadow-lg shadow-sky-500/30 transition-all hover:bg-sky-400 hover:shadow-sky-400/40 active:scale-95"
      >
        Start Flying
      </Link>

      {/* Controls hint */}
      <div className="mt-16 text-center text-xs text-gray-600">
        <p className="mb-1 font-semibold uppercase tracking-widest text-gray-500">
          Controls
        </p>
        <p>W / S &mdash; Pitch &nbsp;&middot;&nbsp; A / D &mdash; Roll</p>
        <p>
          Q / E &mdash; Yaw &nbsp;&middot;&nbsp; Shift / Ctrl &mdash; Throttle
        </p>
      </div>

      {/* Version */}
      <span className="absolute bottom-4 right-4 text-[10px] text-gray-700">
        v0.1.0
      </span>
    </main>
  );
}
