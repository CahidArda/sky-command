"use client";

import { useGameStore } from "@/stores/gameStore";
import { MS_TO_KNOTS, M_TO_FT } from "@/lib/constants";
import VersionBadge from "./VersionBadge";

// ---------------------------------------------------------------------------
// HUD — HTML overlay with flight instruments
//
// Rendered OUTSIDE the R3F Canvas as absolutely-positioned HTML.
// Military/flight-sim style: monospace green text on translucent dark bg.
// ---------------------------------------------------------------------------

function fmt(n: number, decimals = 0): string {
  return n.toFixed(decimals);
}

export default function HUD() {
  const aircraft = useGameStore((s) => s.aircraft);

  const speedKnots = aircraft.airspeed * MS_TO_KNOTS;
  const altitudeFt = aircraft.altitude * M_TO_FT;
  const heading = aircraft.heading;
  const throttlePct = aircraft.throttle * 100;

  return (
    <div className="pointer-events-none absolute inset-0 z-10 select-none">
      {/* ── Top-left: Speed ──────────────────────────────────────────── */}
      <div className="absolute left-6 top-6">
        <HudBlock label="SPD" value={`${fmt(speedKnots)} kts`} />
      </div>

      {/* ── Top-right: Altitude ──────────────────────────────────────── */}
      <div className="absolute right-6 top-6">
        <HudBlock label="ALT" value={`${fmt(altitudeFt)} ft`} />
      </div>

      {/* ── Bottom-left: Heading ─────────────────────────────────────── */}
      <div className="absolute bottom-6 left-6">
        <HudBlock label="HDG" value={`${fmt(heading)}\u00B0`} />
      </div>

      {/* ── Left-center: Throttle bar ────────────────────────────────── */}
      <div className="absolute left-6 top-1/2 -translate-y-1/2">
        <div className="flex flex-col items-center gap-1">
          <span className="font-mono text-xs uppercase tracking-widest text-green-400/80">
            THR
          </span>
          <div className="relative h-40 w-4 overflow-hidden rounded-sm border border-green-400/40 bg-black/40">
            <div
              className="absolute bottom-0 w-full bg-green-400/70 transition-all duration-75"
              style={{ height: `${throttlePct}%` }}
            />
          </div>
          <span className="font-mono text-xs text-green-400">
            {fmt(throttlePct)}%
          </span>
        </div>
      </div>

      {/* ── Center: Crosshair ────────────────────────────────────────── */}
      <div className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
        <svg width="40" height="40" viewBox="0 0 40 40" className="opacity-50">
          <circle
            cx="20"
            cy="20"
            r="8"
            fill="none"
            stroke="#22c55e"
            strokeWidth="1"
          />
          <line x1="0" y1="20" x2="12" y2="20" stroke="#22c55e" strokeWidth="1" />
          <line x1="28" y1="20" x2="40" y2="20" stroke="#22c55e" strokeWidth="1" />
          <line x1="20" y1="0" x2="20" y2="12" stroke="#22c55e" strokeWidth="1" />
          <line x1="20" y1="28" x2="20" y2="40" stroke="#22c55e" strokeWidth="1" />
        </svg>
      </div>

      {/* ── Bottom-center: Controls hint ─────────────────────────────── */}
      <div className="absolute bottom-6 left-1/2 -translate-x-1/2">
        <p className="font-mono text-[10px] tracking-wide text-green-400/50">
          W/S Pitch &middot; A/D Roll &middot; Q/E Yaw &middot; Shift/Ctrl
          Throttle
        </p>
      </div>

      {/* ── Bottom-right: Version badge ──────────────────────────────── */}
      <VersionBadge />
    </div>
  );
}

// ---------------------------------------------------------------------------
// Small reusable block for a labeled value
// ---------------------------------------------------------------------------

function HudBlock({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-sm bg-black/40 px-3 py-2 backdrop-blur-sm">
      <span className="block font-mono text-[10px] uppercase tracking-widest text-green-400/60">
        {label}
      </span>
      <span className="block font-mono text-lg font-bold tabular-nums text-green-400">
        {value}
      </span>
    </div>
  );
}
