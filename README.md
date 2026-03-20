# SkyCommand

A 3D flight simulator with realistic aerodynamics, playable in the browser. Fly a Cessna 172 with lift, drag, thrust, and weight modeled from first principles.

Built as two independent implementations sharing the same flight model and game design:

| Package | Stack | Status |
|---------|-------|--------|
| `@skycommand/web` | Next.js 15 + React Three Fiber + Zustand | v0.1.0 |
| `@skycommand/wasm` | Bevy 0.15 + Rust → WebAssembly | v0.1.0 |

## Tech Stack

### Web (Three.js)

- **Next.js 15** — static export (`output: 'export'`)
- **React Three Fiber** + **@react-three/drei** — 3D rendering
- **Zustand** — game state
- **Tailwind CSS** — HUD styling
- **TypeScript**

### WASM (Bevy)

- **Bevy 0.15** — ECS game engine
- **Rust** compiled to `wasm32-unknown-unknown`
- **wasm-bindgen** — JS interop

### Shared

- **GitHub Actions** — CI + release workflows
- **Cloudflare Pages** — static hosting
- **Cloudflare R2** — asset CDN (planned)

## Flight Model

Both implementations share the same physics:

```
Lift   = 0.5 * rho * V^2 * S * Cl(alpha)
Drag   = 0.5 * rho * V^2 * S * (Cd0 + Cl^2 / (pi * e * AR))
Thrust = throttle * max_thrust * (rho / rho_sea_level)
Weight = mass * g
```

- ISA atmosphere (density decreases with altitude)
- Angle of attack computed relative to world up (roll-invariant)
- Lift direction is always toward the wing's top surface
- Aerodynamic yaw aligns the nose with the velocity during banked turns
- Stall modeled above 15 deg AoA

## Controls

| Key | Action |
|-----|--------|
| W / S | Pitch (nose down / up) |
| A / D | Roll (bank) |
| Q / E | Yaw (rudder) |
| Shift | Increase throttle |
| Ctrl | Decrease throttle |

Banking is the primary way to turn — it tilts the lift vector, curving the flight path.

## Local Development

### Web package

```bash
cd packages/web
pnpm install
pnpm dev          # http://localhost:3000
```

Other commands:

```bash
pnpm build        # static export → out/
pnpm type-check   # tsc --noEmit
pnpm test         # vitest
pnpm lint         # next lint
```

### WASM package

```bash
cd packages/wasm

# prerequisites
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# dev build + serve
wasm-pack build --dev --target web
ln -sf ../pkg web/pkg   # symlink build output into serve root
npx serve web/

# check / test
cargo check
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Deployment

Both packages deploy as static sites to Cloudflare Pages.

| Project | Domain | Build | Output |
|---------|--------|-------|--------|
| `skycommand-web` | `skycommand.dev` | `cd packages/web && pnpm build` | `packages/web/out/` |
| `skycommand-wasm` | `wasm.skycommand.dev` | `cd packages/wasm && wasm-pack build --release --target web` | `packages/wasm/web/` |

### Releasing

Releases are triggered via GitHub Actions workflow dispatch:

```bash
# Bump and release the web package
gh workflow run release-web.yml -f bump=patch

# Bump and release the WASM package
gh workflow run release-wasm.yml -f bump=minor
```

This bumps the version, tags, builds, creates a GitHub Release, and deploys.

## Project Structure

```
skycommand/
├── packages/
│   ├── web/              # Next.js + React Three Fiber
│   │   ├── src/
│   │   │   ├── app/      # Next.js pages (landing + game)
│   │   │   ├── game/     # 3D scene, aircraft, physics, HUD, camera
│   │   │   ├── stores/   # Zustand game state
│   │   │   └── lib/      # Constants, version
│   │   └── package.json
│   └── wasm/             # Bevy + Rust
│       ├── src/
│       │   ├── aircraft/  # Aircraft components + mesh
│       │   ├── physics/   # Flight model + atmosphere
│       │   ├── world/     # Terrain + sky
│       │   ├── ui/        # HUD + version display
│       │   ├── input/     # Keyboard handling
│       │   └── camera/    # Chase camera
│       ├── web/           # HTML host for WASM
│       └── Cargo.toml
├── shared/
│   ├── aircraft/          # Aircraft spec JSONs
│   └── missions/          # Mission definition JSONs
├── scripts/               # Version bump, asset sync
└── .github/workflows/     # CI + release pipelines
```

## License

See [LICENSE](LICENSE).
