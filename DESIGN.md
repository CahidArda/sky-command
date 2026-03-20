# SkyCommand — Flight Simulator Project Specification

## Overview

SkyCommand is a 3D flight simulator with mission-based gameplay, built as two independent implementations sharing common assets:

- **`@skycommand/wasm`** — Rust compiled to WebAssembly (Bevy + wgpu → WebGPU/WebGL2)
- **`@skycommand/web`** — TypeScript with React Three Fiber (Three.js + Next.js shell)

Both deploy as static sites to **Cloudflare Pages**. The repo is a monorepo with independent versioning per package, GitHub Actions–driven releases, and shared game assets.

---

## Repository Structure

```
skycommand/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                  # lint + test + build on every PR
│   │   ├── release-wasm.yml        # bump version, tag, deploy @skycommand/wasm
│   │   └── release-web.yml         # bump version, tag, deploy @skycommand/web
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── pull_request_template.md
│
├── packages/
│   ├── wasm/                       # Rust / Bevy / WASM implementation
│   │   ├── Cargo.toml              # version lives here (e.g. 0.1.0)
│   │   ├── Cargo.lock
│   │   ├── rust-toolchain.toml     # pin nightly/stable + wasm target
│   │   ├── src/
│   │   │   ├── main.rs             # entry point, app bootstrap
│   │   │   ├── lib.rs              # wasm_bindgen entry when targeting web
│   │   │   ├── aircraft/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── prop.rs         # Cessna-style prop plane
│   │   │   │   ├── airliner.rs     # Boeing 737-style
│   │   │   │   ├── fighter.rs      # F-15 Eagle
│   │   │   │   └── bomber.rs       # B-2 Spirit
│   │   │   ├── missions/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── ferry.rs        # fly A → B
│   │   │   │   ├── intercept.rs    # take out drones
│   │   │   │   ├── naval_strike.rs # bomb patrol boats
│   │   │   │   └── facility.rs     # bomb enemy facilities
│   │   │   ├── physics/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── flight_model.rs # lift, drag, thrust, weight
│   │   │   │   ├── atmosphere.rs   # air density vs altitude
│   │   │   │   └── collision.rs
│   │   │   ├── world/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── terrain.rs      # heightmap-based terrain
│   │   │   │   ├── sky.rs          # skybox, sun, fog
│   │   │   │   └── objects.rs      # buildings, runways, targets
│   │   │   ├── ui/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── hud.rs          # speed, altitude, heading, weapons
│   │   │   │   ├── menu.rs         # mission select, aircraft select
│   │   │   │   └── version.rs      # version display component
│   │   │   ├── input/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── keyboard.rs
│   │   │   │   └── gamepad.rs
│   │   │   ├── audio/
│   │   │   │   └── mod.rs          # engine sounds, explosions, wind
│   │   │   └── camera/
│   │   │       └── mod.rs          # chase cam, cockpit cam, free cam
│   │   ├── web/                    # wasm host page
│   │   │   ├── index.html
│   │   │   └── styles.css
│   │   ├── tests/
│   │   │   ├── flight_model_test.rs
│   │   │   └── mission_test.rs
│   │   └── README.md
│   │
│   └── web/                        # Three.js / R3F / Next.js implementation
│       ├── package.json            # version lives here (e.g. 0.1.0)
│       ├── tsconfig.json
│       ├── next.config.ts
│       ├── tailwind.config.ts
│       ├── src/
│       │   ├── app/
│       │   │   ├── layout.tsx
│       │   │   ├── page.tsx        # landing / mission select
│       │   │   └── game/
│       │   │       └── page.tsx    # game canvas page
│       │   ├── game/
│       │   │   ├── Game.tsx        # R3F Canvas wrapper
│       │   │   ├── GameLoop.tsx    # useFrame-based game loop
│       │   │   ├── aircraft/
│       │   │   │   ├── index.ts
│       │   │   │   ├── PropPlane.tsx
│       │   │   │   ├── Airliner.tsx
│       │   │   │   ├── Fighter.tsx
│       │   │   │   └── Bomber.tsx
│       │   │   ├── missions/
│       │   │   │   ├── index.ts
│       │   │   │   ├── FerryMission.ts
│       │   │   │   ├── InterceptMission.ts
│       │   │   │   ├── NavalStrikeMission.ts
│       │   │   │   └── FacilityMission.ts
│       │   │   ├── physics/
│       │   │   │   ├── FlightModel.ts
│       │   │   │   ├── Atmosphere.ts
│       │   │   │   └── Collision.ts
│       │   │   ├── world/
│       │   │   │   ├── Terrain.tsx
│       │   │   │   ├── Sky.tsx
│       │   │   │   ├── Ocean.tsx
│       │   │   │   └── Targets.tsx
│       │   │   ├── hud/
│       │   │   │   ├── HUD.tsx
│       │   │   │   ├── Altimeter.tsx
│       │   │   │   ├── SpeedIndicator.tsx
│       │   │   │   ├── WeaponSelect.tsx
│       │   │   │   └── VersionBadge.tsx
│       │   │   ├── camera/
│       │   │   │   └── FlightCamera.tsx
│       │   │   └── input/
│       │   │       ├── KeyboardControls.ts
│       │   │       └── GamepadControls.ts
│       │   ├── stores/
│       │   │   ├── gameStore.ts    # zustand — game state
│       │   │   └── settingsStore.ts
│       │   └── lib/
│       │       ├── constants.ts
│       │       └── version.ts      # reads version from package.json
│       ├── public/
│       │   └── .gitkeep            # static assets built from shared/
│       ├── tests/
│       │   ├── FlightModel.test.ts
│       │   └── missions.test.ts
│       └── README.md
│
├── shared/                         # shared game design & raw assets
│   ├── assets/
│   │   ├── models/                 # .glb/.gltf source files
│   │   │   ├── aircraft/
│   │   │   │   ├── prop.glb
│   │   │   │   ├── airliner.glb
│   │   │   │   ├── fighter.glb
│   │   │   │   └── bomber.glb
│   │   │   ├── environment/
│   │   │   │   ├── runway.glb
│   │   │   │   ├── hangar.glb
│   │   │   │   ├── control_tower.glb
│   │   │   │   └── patrol_boat.glb
│   │   │   └── weapons/
│   │   │       ├── missile.glb
│   │   │       └── bomb.glb
│   │   ├── textures/
│   │   │   ├── terrain/
│   │   │   └── sky/
│   │   ├── audio/
│   │   │   ├── engines/
│   │   │   ├── weapons/
│   │   │   └── ambient/
│   │   └── heightmaps/
│   │       └── default.png
│   ├── missions/                   # mission definitions (shared JSON)
│   │   ├── schema.json             # mission definition schema
│   │   ├── ferry_istanbul_ankara.json
│   │   ├── intercept_aegean.json
│   │   ├── naval_strike_black_sea.json
│   │   └── facility_raid_desert.json
│   └── aircraft/                   # aircraft specs (shared JSON)
│       ├── schema.json
│       ├── prop.json               # max_speed, climb_rate, weapons, etc.
│       ├── airliner.json
│       ├── fighter.json
│       └── bomber.json
│
├── scripts/
│   ├── bump-version.sh             # used by GH Actions to bump versions
│   └── sync-assets.sh              # copies shared/ assets into each package
│
├── .gitignore
├── .editorconfig
├── LICENSE
├── README.md                       # root readme with project overview
└── CONTRIBUTING.md                  # dev workflow, commit conventions
```

---

## Aircraft & Mission Matrix

| Aircraft       | Ferry (A→B) | Intercept Drones | Bomb Boats | Bomb Facilities |
|----------------|:-----------:|:----------------:|:----------:|:---------------:|
| Prop Plane     | ✓           |                  |            |                 |
| Airliner       | ✓           |                  |            |                 |
| F-15 Fighter   | ✓           | ✓                | ✓          |                 |
| B-2 Bomber     | ✓           |                  | ✓          | ✓               |

### Aircraft Specs (baseline values)

| Stat            | Prop       | Airliner   | F-15       | B-2        |
|-----------------|------------|------------|------------|------------|
| Max Speed (kts) | 180        | 490        | 1,450      | 560        |
| Ceiling (ft)    | 15,000     | 41,000     | 65,000     | 50,000     |
| Climb (ft/min)  | 1,000      | 3,000      | 50,000     | 5,000      |
| Weapons         | None       | None       | AIM-9, AGM | GBU, JDAM  |
| Agility         | High       | Low        | Very High  | Low        |

### Mission Types

**1. Ferry (fly A → B)**
- Available to: all aircraft
- Objectives: take off, fly route, land safely
- Scoring: fuel efficiency, time, landing smoothness
- Failure: crash, run out of fuel, deviate too far from route

**2. Intercept Drones**
- Available to: F-15
- Objectives: locate and destroy N drone targets
- Mechanics: radar lock, AIM-9 missiles, gun
- Scoring: time to kill, missiles used, no friendly fire
- Failure: drones escape zone, player destroyed

**3. Naval Strike (bomb patrol boats)**
- Available to: F-15, B-2
- Objectives: destroy patrol boats in target zone
- Mechanics: AGM-65 (F-15) or GBU-31 (B-2), approach altitude matters
- Scoring: targets destroyed, collateral, time
- Failure: targets survive, player destroyed

**4. Facility Raid (bomb enemy facilities)**
- Available to: B-2
- Objectives: destroy hardened targets, minimize collateral
- Mechanics: JDAM precision bombing, bombing altitude/angle matters
- Scoring: precision, collateral damage rating, stealth (undetected)
- Failure: primary target survives, excessive collateral

---

## Flight Model (shared across both implementations)

Both implementations use the same simplified flight model. The physics are tuned for fun over realism, but grounded in real aerodynamic principles.

### Core Forces

```
Lift      = 0.5 × ρ × V² × S × Cl(α)
Drag      = 0.5 × ρ × V² × S × (Cd0 + Cl²/(π × e × AR))
Thrust    = throttle × max_thrust × (ρ / ρ_sea_level)
Weight    = mass × g
```

Where:
- `ρ` = air density (decreases with altitude via ISA model)
- `V` = airspeed
- `S` = wing area
- `Cl(α)` = lift coefficient as function of angle of attack
- `Cd0` = parasitic drag coefficient
- `e` = Oswald efficiency
- `AR` = aspect ratio

### Simplifications
- No fuel burn (weight stays constant) — keeps it arcade-friendly
- No wind/turbulence in v1
- Flat earth (no curvature) — terrain is a finite heightmap
- Simplified stall: below critical α, Cl drops linearly
- No gyroscopic effects or adverse yaw

### Control Surfaces → Rotation
- Pitch: elevator → rotation around lateral axis
- Roll: ailerons → rotation around longitudinal axis
- Yaw: rudder → rotation around vertical axis
- Each has a rate limit per aircraft type (fighter >> airliner)

---

## Versioning System

### Scheme

Both packages follow **Semantic Versioning (semver)**: `MAJOR.MINOR.PATCH`

- `MAJOR` — breaking changes (new save format, incompatible mission schema)
- `MINOR` — new content (new aircraft, missions, features)
- `PATCH` — bug fixes, balance tweaks, polish

### Where Versions Live

| Package | Version Source | Tag Format |
|---------|---------------|------------|
| `@skycommand/wasm` | `packages/wasm/Cargo.toml` | `wasm-v0.1.0` |
| `@skycommand/web` | `packages/web/package.json` | `web-v0.1.0` |

### Version Display in UI

Both implementations display their version in the bottom-right corner of the screen:

```
SkyCommand v0.3.1 (wasm)
```
or
```
SkyCommand v0.2.0 (web)
```

The version is read at build time:
- **Rust/WASM**: `env!("CARGO_PKG_VERSION")` baked in at compile time
- **Next.js/web**: imported from `package.json` version field via `src/lib/version.ts`

---

## GitHub Actions

### 1. `ci.yml` — Continuous Integration

**Triggers:** every push and PR to `main`

```
Steps:
  ┌─ detect changed paths ─┐
  │                         │
  ├─ packages/wasm/**  ─────┤──→ cargo fmt --check
  │                         │    cargo clippy -- -D warnings
  │                         │    cargo test
  │                         │    wasm-pack build --target web
  │                         │
  ├─ packages/web/**   ─────┤──→ pnpm lint
  │                         │    pnpm type-check
  │                         │    pnpm test
  │                         │    pnpm build
  │                         │
  └─ shared/**         ─────┘──→ validate JSON schemas
                                 trigger both pipelines
```

### 2. `release-wasm.yml` — Release Rust/WASM Package

**Triggers:** manual dispatch (`workflow_dispatch`) with version bump type input

```
Inputs:
  bump: patch | minor | major

Steps:
  1. Checkout repo
  2. Read current version from Cargo.toml
  3. Calculate new version based on bump type
  4. Update Cargo.toml version
  5. Update Cargo.lock
  6. Commit: "chore(wasm): release v{NEW_VERSION}"
  7. Tag: "wasm-v{NEW_VERSION}"
  8. Push commit + tag
  9. Build: wasm-pack build --release --target web
  10. Create GitHub Release with tag "wasm-v{NEW_VERSION}"
      - Attach wasm build artifacts (.wasm, .js glue)
  11. Deploy to Cloudflare Pages (wasm project)
```

### 3. `release-web.yml` — Release Three.js/Next.js Package

**Triggers:** manual dispatch (`workflow_dispatch`) with version bump type input

```
Inputs:
  bump: patch | minor | major

Steps:
  1. Checkout repo
  2. Read current version from package.json
  3. Calculate new version based on bump type
  4. Update package.json version
  5. Commit: "chore(web): release v{NEW_VERSION}"
  6. Tag: "web-v{NEW_VERSION}"
  7. Push commit + tag
  8. Build: pnpm build && pnpm export (static output)
  9. Create GitHub Release with tag "web-v{NEW_VERSION}"
  10. Deploy to Cloudflare Pages (web project)
```

### `scripts/bump-version.sh`

```bash
#!/bin/bash
# Usage: ./scripts/bump-version.sh <package> <bump>
# Example: ./scripts/bump-version.sh wasm patch
#          ./scripts/bump-version.sh web minor

PACKAGE=$1  # wasm | web
BUMP=$2     # patch | minor | major

if [ "$PACKAGE" = "wasm" ]; then
  FILE="packages/wasm/Cargo.toml"
  CURRENT=$(grep '^version' "$FILE" | head -1 | sed 's/.*"\(.*\)"/\1/')
elif [ "$PACKAGE" = "web" ]; then
  FILE="packages/web/package.json"
  CURRENT=$(node -p "require('./$FILE').version")
fi

IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"

case $BUMP in
  major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
  minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
  patch) PATCH=$((PATCH + 1)) ;;
esac

NEW="${MAJOR}.${MINOR}.${PATCH}"

if [ "$PACKAGE" = "wasm" ]; then
  sed -i "s/^version = \"$CURRENT\"/version = \"$NEW\"/" "$FILE"
elif [ "$PACKAGE" = "web" ]; then
  node -e "
    const fs = require('fs');
    const pkg = JSON.parse(fs.readFileSync('$FILE'));
    pkg.version = '$NEW';
    fs.writeFileSync('$FILE', JSON.stringify(pkg, null, 2) + '\n');
  "
fi

echo "$NEW"
```

---

## Cloudflare Pages Deployment

Two separate Cloudflare Pages projects:

| Project | Domain | Build Command | Output Dir |
|---------|--------|---------------|------------|
| `skycommand-wasm` | `wasm.skycommand.dev` | `cd packages/wasm && wasm-pack build --release --target web` | `packages/wasm/web/` |
| `skycommand-web` | `skycommand.dev` | `cd packages/web && pnpm build` | `packages/web/out/` |

### Asset Strategy

Large assets (3D models, textures, audio) are stored in Cloudflare R2 and lazy-loaded at runtime, not bundled with the deploy:

```
R2 bucket: skycommand-assets
├── models/v1/
│   ├── prop.glb
│   ├── fighter.glb
│   └── ...
├── textures/v1/
└── audio/v1/
```

Asset URLs follow: `https://assets.skycommand.dev/models/v1/fighter.glb`

The `shared/assets/` directory in the repo contains source files and a build script that optimizes (compresses, converts) and uploads to R2. Local development uses the raw files directly.

---

## Development Guidelines

### Branch Strategy

```
main              ← always deployable, protected
├── feat/xxx      ← new features
├── fix/xxx       ← bug fixes
├── refactor/xxx  ← code cleanup
└── docs/xxx      ← documentation
```

PRs into `main` require:
- CI passing
- At least the relevant package building successfully
- Descriptive PR title following conventional commits

### Commit Convention

Use **Conventional Commits** scoped to the package:

```
feat(wasm): add F-15 aircraft model and flight characteristics
fix(web): correct altimeter reading at high altitude
feat(shared): add naval strike mission definition
chore(wasm): release v0.3.0
docs: update README with deployment instructions
```

Scopes: `wasm`, `web`, `shared`, `ci`, or omit for repo-wide changes.

### Development Workflow Rules

1. **Commit after every meaningful change.** A meaningful change is:
   - A new module/file that compiles/passes lint
   - A feature that works end-to-end (even if minimal)
   - A bug fix with its test
   - A refactor that passes all tests
   - Do NOT accumulate 500 lines of uncommitted work

2. **Test before committing.** Run the relevant test suite:
   - Rust: `cargo test` in `packages/wasm/`
   - Web: `pnpm test` in `packages/web/`

3. **One concern per commit.** Don't mix "add fighter jet" with "fix terrain rendering."

4. **Keep PRs focused.** One feature or fix per PR. A PR adding a new aircraft type is fine. A PR adding a new aircraft + rewriting the physics engine + updating CI is not.

5. **Write the test alongside the feature.** Flight model changes require corresponding test updates. Mission logic requires mission completion/failure tests.

### Local Development Setup

**Rust/WASM:**
```bash
cd packages/wasm

# install dependencies
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# dev build + serve
wasm-pack build --dev --target web
# serve packages/wasm/web/ with any static server
npx serve web/

# run tests
cargo test

# lint
cargo fmt --check
cargo clippy -- -D warnings
```

**Three.js/Next.js:**
```bash
cd packages/web

# install dependencies
pnpm install

# dev server
pnpm dev          # http://localhost:3000

# run tests
pnpm test

# lint + type check
pnpm lint
pnpm type-check

# production build (static export)
pnpm build
```

**Asset pipeline:**
```bash
# sync shared assets into package-specific locations
./scripts/sync-assets.sh

# optimize and upload to R2 (requires wrangler auth)
cd shared/assets && ./build.sh
```

---

## Tech Stack Summary

### Rust / WASM Package

| Concern | Library |
|---------|---------|
| Game engine | Bevy 0.15+ |
| WASM target | wasm-pack, wasm-bindgen |
| GPU | wgpu (via Bevy) → WebGPU / WebGL2 fallback |
| 3D models | bevy_gltf (built-in) |
| Audio | bevy_audio (built-in) |
| UI (menus) | bevy_ui or egui (via bevy_egui) |
| Physics | Custom (see flight model above) |

### Three.js / Next.js Package

| Concern | Library |
|---------|---------|
| Framework | Next.js 15 (static export) |
| 3D engine | Three.js via React Three Fiber |
| 3D helpers | @react-three/drei |
| Physics | Custom (same flight model, ported to TS) |
| State | Zustand |
| Styling | Tailwind CSS |
| 3D models | @react-three/drei useGLTF |
| Audio | Howler.js or Three.js AudioListener |
| Input | @react-three/drei KeyboardControls + Gamepad API |

### Shared / Infra

| Concern | Tool |
|---------|------|
| Monorepo | Simple directory structure (no turborepo needed) |
| Versioning | semver, bump script, GH Actions |
| CI | GitHub Actions |
| Hosting | Cloudflare Pages |
| Asset CDN | Cloudflare R2 |
| 3D models | Blender → glTF/GLB export |

---

## Milestone Plan

### v0.1.0 — Flight Prototype
- [ ] Basic terrain (flat ground + heightmap)
- [ ] Skybox
- [ ] One aircraft (prop plane) with flight model
- [ ] Keyboard input (pitch, roll, yaw, throttle)
- [ ] Chase camera
- [ ] HUD: speed, altitude, heading
- [ ] Version display in UI
- [ ] CI pipeline working
- [ ] Deploys to Cloudflare Pages

### v0.2.0 — All Aircraft
- [ ] Add airliner, F-15, B-2 models and flight characteristics
- [ ] Aircraft selection menu
- [ ] Per-aircraft HUD differences (weapons display for combat aircraft)
- [ ] Cockpit camera option
- [ ] Engine sound per aircraft type

### v0.3.0 — Ferry Mission
- [ ] Airport A and B with runways
- [ ] Takeoff and landing detection
- [ ] Route waypoints
- [ ] Mission briefing screen
- [ ] Mission success/failure conditions
- [ ] Scoring system

### v0.4.0 — Combat Missions
- [ ] Weapon systems (missiles, bombs)
- [ ] Drone AI (simple patrol + evade)
- [ ] Patrol boat targets (moving on water)
- [ ] Facility targets (static, hardened)
- [ ] Explosion effects
- [ ] Damage model (player can be shot down)
- [ ] Radar/targeting system for HUD

### v0.5.0 — Polish
- [ ] Mission select screen with map
- [ ] Sound design pass
- [ ] Particle effects (contrails, smoke, fire)
- [ ] Settings (graphics quality, controls rebind)
- [ ] Gamepad support
- [ ] Loading screen with tips
- [ ] Performance optimization pass

### v1.0.0 — Release
- [ ] All missions playable and balanced
- [ ] Both implementations feature-complete
- [ ] Performance tested on mid-range hardware
- [ ] Landing page with implementation switcher