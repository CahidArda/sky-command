#!/bin/bash
# Syncs shared assets into each package's local asset directory

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SHARED_DIR="$ROOT_DIR/shared/assets"

# Sync to web package public directory
WEB_PUBLIC="$ROOT_DIR/packages/web/public/assets"
mkdir -p "$WEB_PUBLIC"
rsync -av --delete "$SHARED_DIR/" "$WEB_PUBLIC/"

echo "Assets synced to packages/web/public/assets/"

# Sync to wasm package assets directory
WASM_ASSETS="$ROOT_DIR/packages/wasm/assets"
mkdir -p "$WASM_ASSETS"
rsync -av --delete "$SHARED_DIR/" "$WASM_ASSETS/"

echo "Assets synced to packages/wasm/assets/"
