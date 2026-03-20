"use client";

import { create } from "zustand";
import * as THREE from "three";
import { createInitialState, type AircraftState } from "@/game/physics/FlightModel";

// ---------------------------------------------------------------------------
// Game-state store (Zustand)
// ---------------------------------------------------------------------------

export interface GameState {
  // Aircraft state — the single source of truth for all flight data
  aircraft: AircraftState;
  isFlying: boolean;

  // Actions
  updateAircraft: (partial: Partial<AircraftState>) => void;
  setAircraft: (state: AircraftState) => void;
  setThrottle: (throttle: number) => void;
  reset: () => void;
}

const initialAircraft = createInitialState();

export const useGameStore = create<GameState>((set) => ({
  aircraft: initialAircraft,
  isFlying: true,

  updateAircraft: (partial) =>
    set((s) => ({
      aircraft: { ...s.aircraft, ...partial },
    })),

  setAircraft: (aircraft) => set({ aircraft }),

  setThrottle: (throttle) =>
    set((s) => ({
      aircraft: { ...s.aircraft, throttle: Math.max(0, Math.min(1, throttle)) },
    })),

  reset: () =>
    set({
      aircraft: createInitialState(),
      isFlying: true,
    }),
}));
