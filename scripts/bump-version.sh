#!/bin/bash
# Usage: ./scripts/bump-version.sh <bump>
# Example: ./scripts/bump-version.sh patch

BUMP=$1  # patch | minor | major
FILE="Cargo.toml"

CURRENT=$(grep '^version' "$FILE" | head -1 | sed 's/.*"\(.*\)"/\1/')
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"

case $BUMP in
  major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
  minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
  patch) PATCH=$((PATCH + 1)) ;;
esac

NEW="${MAJOR}.${MINOR}.${PATCH}"
sed -i '' "s/^version = \"$CURRENT\"/version = \"$NEW\"/" "$FILE"
echo "$NEW"
