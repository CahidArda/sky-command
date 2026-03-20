// ---------------------------------------------------------------------------
// Flight-model physics — runs once per frame
//
// All units are SI internally:
//   position: metres, velocity: m/s, angles: radians, forces: newtons
// ---------------------------------------------------------------------------

import * as THREE from "three";
import { density } from "./Atmosphere";
import {
  GRAVITY,
  SEA_LEVEL_DENSITY,
  CL_ALPHA_SLOPE,
  STALL_ALPHA,
  CL_MAX,
  PROP_PLANE,
} from "@/lib/constants";

// ── helpers ────────────────────────────────────────────────────────────────

/** Clamp a value between min and max. */
function clamp(v: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, v));
}

/**
 * Simplified lift-coefficient model.
 * Linear up to the stall angle, then drops off.
 */
export function liftCoefficient(alpha: number): number {
  const absAlpha = Math.abs(alpha);
  if (absAlpha <= STALL_ALPHA) {
    return CL_ALPHA_SLOPE * alpha;
  }
  // Post-stall: Cl drops linearly back toward zero
  const sign = Math.sign(alpha);
  const postStallFraction = 1 - (absAlpha - STALL_ALPHA) / (Math.PI / 2 - STALL_ALPHA);
  return sign * CL_MAX * clamp(postStallFraction, 0, 1);
}

// ── state type ─────────────────────────────────────────────────────────────

export interface AircraftState {
  position: THREE.Vector3;
  /** Euler rotation (pitch, yaw, roll) in intrinsic order "YXZ" */
  rotation: THREE.Euler;
  velocity: THREE.Vector3;
  airspeed: number;
  altitude: number;
  heading: number;
  throttle: number;
  angularVelocity: THREE.Vector3; // (pitchRate, yawRate, rollRate)
}

// ── control input ──────────────────────────────────────────────────────────

export interface ControlInput {
  pitch: number; // -1 (nose down) to +1 (nose up)
  roll: number; // -1 (left) to +1 (right)
  yaw: number; // -1 (left) to +1 (right)
  throttle: number; // 0 – 1 (absolute)
}

// ── integrator ─────────────────────────────────────────────────────────────

/**
 * Advance the flight model by `dt` seconds.
 *
 * Mutates `state` in place and returns it for convenience.
 */
export function stepFlightModel(
  state: AircraftState,
  input: ControlInput,
  dt: number,
): AircraftState {
  const {
    wingArea: S,
    mass,
    maxThrust,
    cd0: Cd0,
    oswaldEfficiency: e,
    aspectRatio: AR,
    pitchRate: maxPitch,
    rollRate: maxRoll,
    yawRate: maxYaw,
  } = PROP_PLANE;

  // Clamp dt to avoid physics explosions after tab-away
  const safeDt = Math.min(dt, 0.05);

  // ── 1. Rotation (control surfaces) ────────────────────────────────────

  // Build a quaternion from the current Euler so we can rotate vectors
  const quat = new THREE.Quaternion().setFromEuler(state.rotation);

  // Local axes in world space
  const forward = new THREE.Vector3(0, 0, -1).applyQuaternion(quat);
  const up = new THREE.Vector3(0, 1, 0).applyQuaternion(quat);
  const right = new THREE.Vector3(1, 0, 0).applyQuaternion(quat);

  // Apply angular velocities (exponential map — small angle approx is fine
  // for the per-frame deltas we deal with)
  const pitchDelta = input.pitch * maxPitch * safeDt;
  const yawDelta = input.yaw * maxYaw * safeDt;
  const rollDelta = input.roll * maxRoll * safeDt;

  const dq = new THREE.Quaternion();
  // Pitch around local right axis
  dq.setFromAxisAngle(right, pitchDelta);
  quat.premultiply(dq);
  // Yaw around local up axis
  dq.setFromAxisAngle(up, -yawDelta);
  quat.premultiply(dq);
  // Roll around local forward axis
  dq.setFromAxisAngle(forward, -rollDelta);
  quat.premultiply(dq);

  quat.normalize();
  state.rotation.setFromQuaternion(quat, "YXZ");

  // Recompute local axes after rotation update
  forward.set(0, 0, -1).applyQuaternion(quat);
  up.set(0, 1, 0).applyQuaternion(quat);

  // ── 2. Aerodynamic forces ─────────────────────────────────────────────

  const altitude = Math.max(state.position.y, 0);
  const rho = density(altitude);

  // Airspeed = magnitude of velocity
  const V = state.velocity.length();
  const dynamicPressure = 0.5 * rho * V * V; // q

  // Angle of attack: angle between forward vector and velocity
  let alpha = 0;
  if (V > 1) {
    const velDir = state.velocity.clone().normalize();
    // AoA is the angle in the plane of symmetry: use dot with forward and up
    const dotFwd = velDir.dot(forward);
    const dotUp = velDir.dot(up);
    alpha = Math.atan2(-dotUp, dotFwd);
  }

  const Cl = liftCoefficient(alpha);
  const Cd_induced = (Cl * Cl) / (Math.PI * e * AR);
  const Cd = Cd0 + Cd_induced;

  // Lift acts perpendicular to velocity in the plane of symmetry (local up
  // projected away from velocity direction).
  const liftMag = dynamicPressure * S * Cl;
  // Drag acts opposite to velocity.
  const dragMag = dynamicPressure * S * Cd;

  // Thrust along forward axis, scaled by density ratio
  const thrust = input.throttle * maxThrust * (rho / SEA_LEVEL_DENSITY);

  // ── Build force vector ────────────────────────────────────────────────

  const force = new THREE.Vector3();

  // Thrust — along forward
  force.addScaledVector(forward, thrust);

  // Lift — perpendicular to velocity, in the symmetry plane
  if (V > 1) {
    const velDir = state.velocity.clone().normalize();
    // Lift direction: component of local-up that is perpendicular to velDir
    const liftDir = up.clone().addScaledVector(velDir, -up.dot(velDir));
    const liftDirLen = liftDir.length();
    if (liftDirLen > 0.001) {
      liftDir.divideScalar(liftDirLen);
      force.addScaledVector(liftDir, liftMag);
    }
  }

  // Drag — opposite to velocity
  if (V > 1) {
    const dragDir = state.velocity.clone().normalize().negate();
    force.addScaledVector(dragDir, dragMag);
  }

  // Weight — always down
  force.y -= mass * GRAVITY;

  // ── 3. Integration (semi-implicit Euler) ──────────────────────────────

  const accel = force.divideScalar(mass);
  state.velocity.addScaledVector(accel, safeDt);
  state.position.addScaledVector(state.velocity, safeDt);

  // Ground collision — simple clamp
  if (state.position.y < 0) {
    state.position.y = 0;
    // Kill downward velocity
    if (state.velocity.y < 0) {
      state.velocity.y = 0;
    }
  }

  // ── 4. Derived values ─────────────────────────────────────────────────

  state.airspeed = V;
  state.altitude = state.position.y;

  // Heading: angle of the forward vector projected onto the XZ plane,
  // measured clockwise from north (+Z → 0, +X → 90).
  const fwd2d = new THREE.Vector2(forward.x, forward.z);
  if (fwd2d.length() > 0.001) {
    // atan2 gives angle from +X axis; we want angle from +Z (north)
    let hdg = Math.atan2(forward.x, forward.z) * (180 / Math.PI);
    if (hdg < 0) hdg += 360;
    state.heading = hdg;
  }

  state.throttle = input.throttle;

  return state;
}

// ── Factory ────────────────────────────────────────────────────────────────

/**
 * Create an initial aircraft state — airborne, heading north at cruise speed.
 */
export function createInitialState(): AircraftState {
  // Start at 1000 m altitude, heading north (negative Z in Three.js is into screen,
  // but we define north = +Z so forward = (0,0,-1) after default rotation means
  // heading south. We'll set rotation so forward = (0,0,1) i.e. heading north.
  // Euler "YXZ": yaw=PI rotates to face +Z.
  const rotation = new THREE.Euler(0, Math.PI, 0, "YXZ");

  // Cruise speed ~60 m/s (~116 knots — roughly 65 % of max)
  const cruiseSpeed = 60;
  const forward = new THREE.Vector3(0, 0, -1).applyEuler(rotation);
  const velocity = forward.multiplyScalar(cruiseSpeed);

  return {
    position: new THREE.Vector3(0, 1000, 0),
    rotation,
    velocity,
    airspeed: cruiseSpeed,
    altitude: 1000,
    heading: 0,
    throttle: 0.65,
    angularVelocity: new THREE.Vector3(),
  };
}
