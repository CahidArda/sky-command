// ---------------------------------------------------------------------------
// ISA (International Standard Atmosphere) model
//
// Valid from sea level up to the tropopause (~11 km / ~36 000 ft).
// Above that we clamp to tropopause values — good enough for our ceiling of
// 15 000 ft.
// ---------------------------------------------------------------------------

import {
  SEA_LEVEL_DENSITY,
  SEA_LEVEL_TEMP,
  TEMP_LAPSE_RATE,
  BAROMETRIC_EXPONENT,
} from "@/lib/constants";

/**
 * Return ISA temperature (K) at a given geometric altitude (m).
 */
export function temperature(altitudeM: number): number {
  const alt = Math.max(0, altitudeM);
  // Clamp at tropopause (11 000 m)
  const h = Math.min(alt, 11_000);
  return SEA_LEVEL_TEMP - TEMP_LAPSE_RATE * h;
}

/**
 * Return ISA air density (kg/m^3) at a given geometric altitude (m).
 *
 * Uses the barometric formula:
 *   rho = rho0 * (T / T0) ^ (g/(L*R) - 1)
 */
export function density(altitudeM: number): number {
  const T = temperature(altitudeM);
  const ratio = T / SEA_LEVEL_TEMP;
  // exponent for density is (g/(L*R) - 1)
  return SEA_LEVEL_DENSITY * Math.pow(ratio, BAROMETRIC_EXPONENT - 1);
}

/**
 * Return ISA pressure (Pa) at a given geometric altitude (m).
 * Not strictly needed for v0.1 flight model but useful for display / future.
 */
export function pressure(altitudeM: number): number {
  const T = temperature(altitudeM);
  const ratio = T / SEA_LEVEL_TEMP;
  // Sea-level pressure 101325 Pa
  return 101_325 * Math.pow(ratio, BAROMETRIC_EXPONENT);
}
