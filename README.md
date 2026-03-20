# SkyCommand

A 3D flight simulator with realistic aerodynamics, running in the browser via WebAssembly. Built with Bevy 0.15 and Rust.

Choose from three aircraft — Cessna 172, Boeing 737, or F-15 Eagle — each with unique flight characteristics, then fly with full lift/drag/thrust physics, control surface animations, and engine sounds.

## Controls

| Key | Action |
|-----|--------|
| W / S | Pitch (nose down / up) |
| A / D | Roll (bank left / right) |
| Q / E | Yaw (rudder) |
| Shift | Increase throttle |
| Ctrl | Decrease throttle |
| C | Toggle chase / cockpit camera |

## Local Development

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Build and run

```bash
wasm-pack build --dev --target web
ln -sf ../pkg web/pkg
ln -sf ../assets web/assets
npx serve web/
```

Open http://localhost:3000.

### Check / test / lint

```bash
cargo check
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Deploy to Cloudflare Pages

### Automatic (CI)

Every push to `main`/`master` triggers the GitHub Actions workflow which:
1. Runs `cargo fmt`, `clippy`, and tests
2. Builds the WASM release binary with `wasm-pack`
3. Deploys `web/` to Cloudflare Pages

Required GitHub secrets:
- `CLOUDFLARE_API_TOKEN` — API token with Pages edit permissions
- `CLOUDFLARE_ACCOUNT_ID` — your Cloudflare account ID

### Manual

```bash
wasm-pack build --release --target web
mkdir -p web/pkg web/assets
cp pkg/* web/pkg/
cp -r assets/* web/assets/
npx wrangler pages deploy web --project-name=skycommand
```

## Project Structure

```
skycommand/
├── src/
│   ├── main.rs              # Native entry point
│   ├── lib.rs               # WASM entry point
│   ├── state.rs             # Game states (Menu / Flying)
│   ├── aircraft/            # Aircraft specs, meshes, components
│   │   ├── mod.rs           # Aircraft enum, selection, plugin
│   │   ├── prop.rs          # Cessna 172
│   │   ├── airliner.rs      # Boeing 737
│   │   └── fighter.rs       # F-15 Eagle
│   ├── physics/             # Flight model
│   │   ├── flight_model.rs  # Lift, drag, thrust, weight, AoA
│   │   ├── atmosphere.rs    # ISA atmosphere model
│   │   └── mod.rs           # Physics systems, aero yaw, animations
│   ├── world/               # Terrain, sky, environment
│   ├── camera/              # Chase + cockpit camera
│   ├── input/               # Keyboard controls
│   ├── audio/               # Engine sounds per aircraft
│   └── ui/                  # HUD, menu, version display
├── web/                     # HTML host for WASM
│   ├── index.html
│   └── styles.css
├── assets/                  # Audio files
├── shared/                  # Aircraft/mission JSON specs
├── Cargo.toml
└── .github/workflows/ci.yml # CI + Cloudflare Pages deploy
```

## Flight Model

```
Lift   = 0.5 * rho * V^2 * S * Cl(alpha)
Drag   = 0.5 * rho * V^2 * S * (Cd0 + Cl^2/(pi*e*AR) + Cd_separation)
Thrust = throttle * max_thrust * (rho / rho_sea_level)
Weight = mass * g
```

- ISA atmosphere (density decreases with altitude)
- Angle of attack computed in the aircraft's pitch plane (roll-invariant)
- Post-stall drag ramps up quadratically (parachute effect)
- Aerodynamic yaw aligns nose with velocity during banked turns
- Per-aircraft sideslip force (rudder effectiveness)

## License

See [LICENSE](LICENSE).
