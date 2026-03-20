"use client";

import { useGameStore } from "@/stores/gameStore";
import { MS_TO_KNOTS, M_TO_FT, RAD_TO_DEG } from "@/lib/constants";
import VersionBadge from "./VersionBadge";

// ---------------------------------------------------------------------------
// HUD — single upper-right instrument panel + controls hint
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

  // Pitch angle: rotation.x in radians → degrees (positive = nose up)
  const pitchDeg = -(aircraft.rotation.x * RAD_TO_DEG);

  return (
    <div className="pointer-events-none absolute inset-0 z-10 select-none">
      {/* ── Upper-right: Instrument panel ──────────────────────────────── */}
      <div className="absolute right-4 top-4 rounded bg-black/60 px-4 py-3 backdrop-blur-sm">
        <table className="font-mono text-sm text-green-400">
          <tbody>
            <Row label="SPD" value={`${fmt(speedKnots)} kts`} />
            <Row label="ALT" value={`${fmt(altitudeFt)} ft`} />
            <Row label="HDG" value={`${fmt(heading)}\u00B0`} />
            <Row label="PIT" value={`${fmt(pitchDeg, 1)}\u00B0`} />
            <Row label="THR" value={`${fmt(throttlePct)}%`} />
          </tbody>
        </table>
      </div>

      {/* ── Center: Crosshair ──────────────────────────────────────────── */}
      <div className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
        <svg width="40" height="40" viewBox="0 0 40 40" className="opacity-50">
          <circle cx="20" cy="20" r="8" fill="none" stroke="#22c55e" strokeWidth="1" />
          <line x1="0" y1="20" x2="12" y2="20" stroke="#22c55e" strokeWidth="1" />
          <line x1="28" y1="20" x2="40" y2="20" stroke="#22c55e" strokeWidth="1" />
          <line x1="20" y1="0" x2="20" y2="12" stroke="#22c55e" strokeWidth="1" />
          <line x1="20" y1="28" x2="20" y2="40" stroke="#22c55e" strokeWidth="1" />
        </svg>
      </div>

      {/* ── Bottom-center: Controls hint ───────────────────────────────── */}
      <div className="absolute bottom-6 left-1/2 -translate-x-1/2">
        <p className="font-mono text-[10px] tracking-wide text-green-400/50">
          W/S Pitch &middot; A/D Roll &middot; Q/E Yaw &middot; Shift/Ctrl
          Throttle
        </p>
      </div>

      {/* ── Version badge ──────────────────────────────────────────────── */}
      <VersionBadge />
    </div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <tr>
      <td className="pr-3 text-[10px] uppercase tracking-widest text-green-400/60">
        {label}
      </td>
      <td className="text-right tabular-nums">{value}</td>
    </tr>
  );
}
