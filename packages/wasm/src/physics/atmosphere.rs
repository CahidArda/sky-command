/// ISA (International Standard Atmosphere) model.
///
/// Reference values:
/// - Sea level density: 1.225 kg/m^3
/// - Sea level temperature: 288.15 K
/// - Temperature lapse rate: 0.0065 K/m
/// - Gravitational acceleration: 9.80665 m/s^2

/// Sea level air density in kg/m^3.
pub const RHO_SEA_LEVEL: f32 = 1.225;

/// Sea level temperature in Kelvin.
pub const T_SEA_LEVEL: f32 = 288.15;

/// Temperature lapse rate in K/m.
pub const LAPSE_RATE: f32 = 0.0065;

/// Gravitational acceleration in m/s^2.
pub const G: f32 = 9.80665;

/// Gas constant for dry air (J/(kg*K)).
pub const R_AIR: f32 = 287.05;

/// Compute air temperature at a given altitude (meters).
/// Uses the tropospheric lapse rate model up to 11,000m.
pub fn temperature(altitude: f32) -> f32 {
    let alt = altitude.max(0.0).min(11000.0);
    T_SEA_LEVEL - LAPSE_RATE * alt
}

/// Compute air density at a given altitude (meters).
/// Uses the barometric formula for the troposphere.
///
/// rho = rho_0 * (T / T_0) ^ (g / (L * R) - 1)
pub fn density(altitude: f32) -> f32 {
    let alt = altitude.max(0.0).min(11000.0);
    let temp = temperature(alt);
    let temp_ratio = temp / T_SEA_LEVEL;
    let exponent = (G / (LAPSE_RATE * R_AIR)) - 1.0;
    RHO_SEA_LEVEL * temp_ratio.powf(exponent)
}

/// Compute air pressure at a given altitude (meters).
/// P = P_0 * (T / T_0) ^ (g / (L * R))
pub fn pressure(altitude: f32) -> f32 {
    let alt = altitude.max(0.0).min(11000.0);
    let temp = temperature(alt);
    let temp_ratio = temp / T_SEA_LEVEL;
    let exponent = G / (LAPSE_RATE * R_AIR);
    let p_sea_level: f32 = 101325.0; // Pa
    p_sea_level * temp_ratio.powf(exponent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sea_level_values() {
        let t = temperature(0.0);
        assert!((t - 288.15).abs() < 0.01);

        let rho = density(0.0);
        assert!((rho - 1.225).abs() < 0.001);
    }

    #[test]
    fn density_decreases_with_altitude() {
        let rho_0 = density(0.0);
        let rho_1000 = density(1000.0);
        let rho_5000 = density(5000.0);

        assert!(rho_1000 < rho_0);
        assert!(rho_5000 < rho_1000);
    }

    #[test]
    fn temperature_at_1000m() {
        let t = temperature(1000.0);
        let expected = 288.15 - 0.0065 * 1000.0;
        assert!((t - expected).abs() < 0.01);
    }
}
