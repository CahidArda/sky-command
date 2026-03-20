// ---------------------------------------------------------------------------
// Flight-model physics — runs once per frame
//
// All units are SI internally:
//   position: metres, velocity: m/s, angles: radians, forces: newtons
//
// Forces on the aircraft (affect velocity / flight path):
//   Lift   = 0.5 × ρ × V² × S × Cl(α)   — perpendicular to velocity, toward aircraft top
//   Drag   = 0.5 × ρ × V² × S × Cd       — opposite to velocity
//   Thrust = throttle × maxThrust × ρ/ρ₀  — along aircraft forward
//   Weight = m × g                         — always down
//
// Rotation (affects heading / nose direction):
//   Control input:  pitch/roll/yaw from player
//   Aerodynamic yaw: nose rotates toward velocity to reduce sideslip
//                    (proportional to dynamic pressure × bank angle)
// ---------------------------------------------------------------------------

import * as THREE from "three";
import { density } from "./Atmosphere";
import {
  GRAVITY,
  SEA_LEVEL_DENSITY,
  CL_ALPHA_SLOPE,
  STALL_ALPHA,
  CL_MAX,
  SIDE_FORCE_COEFF,
  AERO_YAW_COEFF,
  Q_CRUISE,
  PROP_PLANE,
} from "@/lib/constants";

// ── helpers ────────────────────────────────────────────────────────────────

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
  const sign = Math.sign(alpha);
  const postStallFraction =
    1 - (absAlpha - STALL_ALPHA) / (Math.PI / 2 - STALL_ALPHA);
  return sign * CL_MAX * clamp(postStallFraction, 0, 1);
}

// ── state type ─────────────────────────────────────────────────────────────

export interface AircraftState {
  position: THREE.Vector3;
  rotation: THREE.Euler;
  velocity: THREE.Vector3;
  airspeed: number;
  altitude: number;
  heading: number;
  throttle: number;
  angularVelocity: THREE.Vector3;
}

// ── control input ──────────────────────────────────────────────────────────

export interface ControlInput {
  pitch: number; // -1 (nose down) to +1 (nose up)
  roll: number; // -1 (left) to +1 (right)
  yaw: number; // -1 (left) to +1 (right)
  throttle: number; // 0 – 1 (absolute)
}

// ── integrator ─────────────────────────────────────────────────────────────

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

  const safeDt = Math.min(dt, 0.05);

  // ── 1. Control-surface rotation ─────────────────────────────────────

  const quat = new THREE.Quaternion().setFromEuler(state.rotation);

  // Local axes in world space (before control input)
  const forward = new THREE.Vector3(0, 0, -1).applyQuaternion(quat);
  const up = new THREE.Vector3(0, 1, 0).applyQuaternion(quat);
  const right = new THREE.Vector3(1, 0, 0).applyQuaternion(quat);

  const pitchDelta = input.pitch * maxPitch * safeDt;
  const yawDelta = input.yaw * maxYaw * safeDt;
  const rollDelta = input.roll * maxRoll * safeDt;

  const dq = new THREE.Quaternion();
  dq.setFromAxisAngle(right, pitchDelta);
  quat.premultiply(dq);
  dq.setFromAxisAngle(up, -yawDelta);
  quat.premultiply(dq);
  dq.setFromAxisAngle(forward, -rollDelta);
  quat.premultiply(dq);

  quat.normalize();
  state.rotation.setFromQuaternion(quat, "YXZ");

  // Recompute all local axes after control input
  forward.set(0, 0, -1).applyQuaternion(quat);
  up.set(0, 1, 0).applyQuaternion(quat);
  right.set(1, 0, 0).applyQuaternion(quat);

  // ── 2. Aerodynamic forces (on velocity) ─────────────────────────────

  const altitude = Math.max(state.position.y, 0);
  const rho = density(altitude);
  const V = state.velocity.length();
  const dynamicPressure = 0.5 * rho * V * V;

  // Angle of attack: angle between velocity and forward in the pitch plane
  let alpha = 0;
  if (V > 1) {
    const velDir = state.velocity.clone().normalize();
    const dotFwd = velDir.dot(forward);
    const dotUp = velDir.dot(up);
    alpha = Math.atan2(-dotUp, dotFwd);
  }

  const Cl = liftCoefficient(alpha);
  const Cd_induced = (Cl * Cl) / (Math.PI * e * AR);
  const Cd = Cd0 + Cd_induced;

  const liftMag = dynamicPressure * S * Cl;
  const dragMag = dynamicPressure * S * Cd;
  const thrust = input.throttle * maxThrust * (rho / SEA_LEVEL_DENSITY);

  // ── Build force vector (only real aerodynamic forces) ───────────────

  const force = new THREE.Vector3();

  // Thrust — along aircraft forward
  force.addScaledVector(forward, thrust);

  if (V > 1) {
    const velDir = state.velocity.clone().normalize();

    // LIFT — perpendicular to velocity, toward aircraft top.
    // This is the key force for banked turns: when the aircraft banks,
    // the lift vector tilts with it. The horizontal component of the
    // tilted lift is what curves the flight path.
    // At 1g level flight: liftMag ≈ m×g ≈ 10,900 N.
    // At 45° bank: horizontal component ≈ 7,600 N → turn rate ≈ 6.6°/s.
    const liftDir = up.clone().addScaledVector(velDir, -up.dot(velDir));
    const liftDirLen = liftDir.length();
    if (liftDirLen > 0.001) {
      liftDir.divideScalar(liftDirLen);
      force.addScaledVector(liftDir, liftMag);
    }

    // DRAG — opposite to velocity
    force.addScaledVector(velDir, -dragMag);

    // SIDESLIP FORCE — pushes velocity toward nose direction.
    // Makes rudder actually change the flight path, not just heading.
    // Moderate coefficient so it only weakly opposes banked turns.
    const dotRight = velDir.dot(right);
    const beta = Math.asin(clamp(dotRight, -1, 1));
    const sideForceMag = dynamicPressure * S * SIDE_FORCE_COEFF * beta;
    const sideDir = right.clone().addScaledVector(velDir, -dotRight);
    const sideDirLen = sideDir.length();
    if (sideDirLen > 0.001) {
      sideDir.divideScalar(sideDirLen);
      force.addScaledVector(sideDir, -sideForceMag);
    }
  }

  // WEIGHT — always down
  force.y -= mass * GRAVITY;

  // ── 3. Integrate velocity and position ──────────────────────────────

  const accel = force.clone().divideScalar(mass);
  state.velocity.addScaledVector(accel, safeDt);
  state.position.addScaledVector(state.velocity, safeDt);

  // Ground collision
  if (state.position.y < 0) {
    state.position.y = 0;
    if (state.velocity.y < 0) state.velocity.y = 0;
  }

  // ── 4. Aerodynamic yaw (nose follows velocity) ──────────────────────
  //
  // The vertical tail + fuselage create a yawing moment that aligns the
  // nose with the velocity direction (reduces sideslip β).
  //
  // Scaled by bank angle: when level, rudder freely changes heading;
  // when banked, the nose tracks the velocity so heading follows the turn.
  //
  // Computed in the HORIZONTAL PLANE to avoid sign confusion from
  // tilted aircraft axes.

  const Vnew = state.velocity.length();
  if (Vnew > 1) {
    // Heading error: signed angle from forward to velocity in the XZ plane.
    // Positive = velocity is clockwise from forward (to the right).
    const fwdX = forward.x, fwdZ = forward.z;
    const velDirNew = state.velocity.clone().normalize();
    const velX = velDirNew.x, velZ = velDirNew.z;

    // 2D cross product (fwd × vel projected onto XZ)
    const cross = fwdZ * velX - fwdX * velZ;
    const dot2d = fwdX * velX + fwdZ * velZ;
    const headingError = Math.atan2(cross, dot2d);

    // Bank factor: mostly active when banked, weak when level.
    // The 0.1 minimum gives a slow heading return after releasing rudder.
    const bankFactor = Math.max(0.1, Math.sqrt(Math.max(0, 1 - up.y * up.y)));

    const qScale = dynamicPressure / Q_CRUISE;
    const yawRate = headingError * AERO_YAW_COEFF * qScale * bankFactor;

    // Rotate around WORLD Y — avoids tilted-axis sign issues
    const yawDq = new THREE.Quaternion();
    yawDq.setFromAxisAngle(new THREE.Vector3(0, 1, 0), yawRate * safeDt);
    quat.premultiply(yawDq);
    quat.normalize();
    state.rotation.setFromQuaternion(quat, "YXZ");

    forward.set(0, 0, -1).applyQuaternion(quat);
  }

  // ── 5. Derived values ───────────────────────────────────────────────

  state.airspeed = Vnew;
  state.altitude = state.position.y;

  const fwd2d = new THREE.Vector2(forward.x, forward.z);
  if (fwd2d.length() > 0.001) {
    let hdg = Math.atan2(forward.x, forward.z) * (180 / Math.PI);
    if (hdg < 0) hdg += 360;
    state.heading = hdg;
  }

  state.throttle = input.throttle;
  return state;
}

// ── Factory ────────────────────────────────────────────────────────────────

export function createInitialState(): AircraftState {
  // Trim α ≈ 3° so Lift ≈ Weight at cruise (60 m/s, 1000 m).
  const TRIM_ALPHA = 0.053;
  const rotation = new THREE.Euler(TRIM_ALPHA, Math.PI, 0, "YXZ");
  const cruiseSpeed = 60;
  const velocity = new THREE.Vector3(0, 0, cruiseSpeed);

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
