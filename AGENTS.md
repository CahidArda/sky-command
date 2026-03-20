# Agent Instructions

## Before Every Commit

Run these checks before committing. CI will reject the push if they fail.

```bash
# 1. Fix formatting (auto-fixes all .rs files)
cargo fmt

# 2. Verify formatting passes (what CI runs)
cargo fmt --check

# 3. Build for WASM target (what CI verifies)
cargo build --target wasm32-unknown-unknown
```

## Why Not Clippy/Tests in CI?

This is a WASM-only app. The `bevy_winit` and `bevy_audio` crates require
native platform libraries (`libasound2-dev`, `libudev-dev`, X11/Wayland)
that aren't relevant to our WASM target. Running clippy or tests on the
CI's Linux host fails because we don't enable the `x11` Cargo feature.

**Run clippy and tests locally on macOS** where native backends work:

```bash
cargo clippy -- -D warnings
cargo test
```

## Commit Convention

Use [Conventional Commits](https://www.conventionalcommits.org/) scoped to the area:

```
feat: add aircraft selection menu
fix: correct angle of attack at 90-degree bank
refactor: simplify F-15 mesh from 120 to 47 entities
chore: update Cargo.lock
docs: update README with deploy instructions
```

## Building for WASM (local dev)

```bash
wasm-pack build --dev --target web
rm -rf web/pkg web/assets
ln -s ../pkg web/pkg
ln -s ../assets web/assets
npx serve web/
```

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | Native entry point |
| `src/lib.rs` | WASM entry point (must mirror main.rs plugin setup) |
| `src/aircraft/mod.rs` | Aircraft enum, selection, spawn logic |
| `src/physics/flight_model.rs` | All aerodynamic force calculations |
| `src/physics/mod.rs` | Physics systems, control surface animations |
| `src/camera/mod.rs` | Chase + cockpit camera with AoA/slip offset |
| `src/ui/hud.rs` | HUD indicators (speed, alt, AoA, G, stall warning) |
| `src/ui/menu.rs` | Aircraft selection menu |
| `web/index.html` | HTML host for WASM (includes AudioContext resume hack) |
