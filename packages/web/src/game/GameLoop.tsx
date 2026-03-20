"use client";

import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import { useKeyboardControls } from "@react-three/drei";
import { Controls } from "./input/KeyboardControls";
import { stepFlightModel, type ControlInput } from "./physics/FlightModel";
import { useGameStore } from "@/stores/gameStore";

// ---------------------------------------------------------------------------
// GameLoop — runs the flight-model physics every frame via useFrame
// ---------------------------------------------------------------------------

const THROTTLE_RATE = 0.5; // throttle change per second when key held

export default function GameLoop() {
  const throttleRef = useRef(useGameStore.getState().aircraft.throttle);
  const [, getKeys] = useKeyboardControls<Controls>();

  useFrame((_, delta) => {
    const keys = getKeys();
    const store = useGameStore.getState();
    const aircraft = store.aircraft;

    // ── Throttle ────────────────────────────────────────────────────────
    if (keys.throttleUp) {
      throttleRef.current = Math.min(1, throttleRef.current + THROTTLE_RATE * delta);
    }
    if (keys.throttleDown) {
      throttleRef.current = Math.max(0, throttleRef.current - THROTTLE_RATE * delta);
    }

    // ── Control input ───────────────────────────────────────────────────
    const input: ControlInput = {
      // W = pitch down (nose down), S = pitch up (nose up)
      pitch: (keys.forward ? -1 : 0) + (keys.back ? 1 : 0),
      // A = roll left, D = roll right
      roll: (keys.left ? 1 : 0) + (keys.right ? -1 : 0),
      // Q = yaw left, E = yaw right
      yaw: (keys.yawLeft ? -1 : 0) + (keys.yawRight ? 1 : 0),
      throttle: throttleRef.current,
    };

    // ── Step physics ────────────────────────────────────────────────────
    stepFlightModel(aircraft, input, delta);

    // Write back to store (trigger React re-renders for HUD)
    store.setAircraft({ ...aircraft });
  });

  return null;
}
