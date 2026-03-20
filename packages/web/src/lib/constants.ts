// ---------------------------------------------------------------------------
// Unit conversion helpers
// ---------------------------------------------------------------------------

/** Metres per second to knots */
export const MS_TO_KNOTS = 1.94384;

/** Knots to metres per second */
export const KNOTS_TO_MS = 1 / MS_TO_KNOTS;

/** Metres to feet */
export const M_TO_FT = 3.28084;

/** Feet to metres */
export const FT_TO_M = 1 / M_TO_FT;

/** Degrees to radians */
export const DEG_TO_RAD = Math.PI / 180;

/** Radians to degrees */
export const RAD_TO_DEG = 180 / Math.PI;

// ---------------------------------------------------------------------------
// Physics constants
// ---------------------------------------------------------------------------

/** Gravitational acceleration (m/s^2) */
export const GRAVITY = 9.80665;

/** Sea-level air density in the ISA model (kg/m^3) */
export const SEA_LEVEL_DENSITY = 1.225;

/** Sea-level temperature in ISA (K) */
export const SEA_LEVEL_TEMP = 288.15;

/** ISA temperature lapse rate (K/m) */
export const TEMP_LAPSE_RATE = 0.0065;

/** Specific gas constant for dry air (J/(kg*K)) */
export const GAS_CONSTANT_AIR = 287.05;

/** Exponent used in barometric formula: g / (L * R) */
export const BAROMETRIC_EXPONENT =
  GRAVITY / (TEMP_LAPSE_RATE * GAS_CONSTANT_AIR); // ~5.2559

// ---------------------------------------------------------------------------
// Prop plane specs (from shared/aircraft/prop.json)
// ---------------------------------------------------------------------------

export const PROP_PLANE = {
  name: "Cessna 172",
  wingArea: 16.2, // m^2
  mass: 1111, // kg
  maxThrust: 3500, // N
  cd0: 0.027,
  oswaldEfficiency: 0.8,
  aspectRatio: 7.32,
  pitchRate: 60 * DEG_TO_RAD, // rad/s
  rollRate: 90 * DEG_TO_RAD, // rad/s
  yawRate: 30 * DEG_TO_RAD, // rad/s
  maxSpeedKnots: 180,
  ceilingFt: 15_000,
  climbRateFtMin: 1_000,
} as const;

// ---------------------------------------------------------------------------
// Lift coefficient model
// ---------------------------------------------------------------------------

/** Critical angle of attack (radians) — beyond this the wing stalls */
export const STALL_ALPHA = 15 * DEG_TO_RAD;

/** Lift-curve slope (per radian) — typical for GA wing */
export const CL_ALPHA_SLOPE = 2 * Math.PI; // ~6.28 /rad

/** Maximum Cl just before stall */
export const CL_MAX = CL_ALPHA_SLOPE * STALL_ALPHA; // ~1.64

/** Lateral (sideslip) force coefficient (per radian of β).
 *  Models side force from the fuselage + vertical tail on the velocity.
 *  Pushes velocity to follow the nose — makes rudder effective. */
export const SIDE_FORCE_COEFF = 1.0;

/** Aerodynamic yaw rate coefficient.
 *  When there's sideslip (β ≠ 0), the vertical tail creates a yawing
 *  moment that rotates the aircraft nose toward the velocity vector.
 *  This is what makes banked turns change heading.
 *  Units: rad/s per radian of β at cruise dynamic pressure.
 *  Scaled by (q / q_cruise) so it's weak in stall. */
export const AERO_YAW_COEFF = 2.0;

/** Reference dynamic pressure at cruise (0.5 × ρ₀ × 60²). */
export const Q_CRUISE = 0.5 * SEA_LEVEL_DENSITY * 60 * 60;

// ---------------------------------------------------------------------------
// World geometry
// ---------------------------------------------------------------------------

/** Half-extent of the terrain plane (metres) — total size = 2x this */
export const TERRAIN_HALF_SIZE = 5_000; // 10 km x 10 km

/** Grid cell size on terrain (metres) */
export const TERRAIN_GRID_CELL = 100;
