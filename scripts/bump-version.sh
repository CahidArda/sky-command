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
  sed -i '' "s/^version = \"$CURRENT\"/version = \"$NEW\"/" "$FILE"
elif [ "$PACKAGE" = "web" ]; then
  node -e "
    const fs = require('fs');
    const pkg = JSON.parse(fs.readFileSync('$FILE'));
    pkg.version = '$NEW';
    fs.writeFileSync('$FILE', JSON.stringify(pkg, null, 2) + '\n');
  "
fi

echo "$NEW"
