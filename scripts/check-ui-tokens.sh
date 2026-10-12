#!/usr/bin/env bash
#
# check-ui-tokens: fail when a colour literal or a hard-coded size appears in a
# view outside the theme module (ADR-0014).
#
# Colours and sizes must come from crates/pattern_ui/src/theme. Every other
# view reads them from the theme tokens. This keeps the three palettes honest
# and makes theming testable.
#
# Usage: scripts/check-ui-tokens.sh [directory]
#   The optional directory defaults to crates/pattern_ui/src and exists so the
#   planted-violation self-test can point the check at a temporary directory.
set -euo pipefail
cd "$(dirname "$0")/.."

SCAN_DIR="${1:-crates/pattern_ui/src}"

if [ ! -d "$SCAN_DIR" ]; then
  echo "check-ui-tokens: OK (no source directory at $SCAN_DIR)"
  exit 0
fi

# Colour constructors and named constants. A view must not build a colour.
COLOR='Color32::(from_rgb|from_rgba|from_rgba_premultiplied|from_rgba_unmultiplied|from_rgba_unmultiplied_const|from_gray|from_black|from_white|BLACK|WHITE|RED|GREEN|BLUE|YELLOW|TRANSPARENT|LIGHT_GRAY|GRAY|DARK_GRAY|BROWN|GOLD)'
# Sizes passed straight to a constructor, e.g. Stroke::new(1.0, ..) or vec2(4.0, 4.0).
SIZE='(Stroke::new|Vec2::new|Vec2::splat|vec2|Rangef::new)[[:space:]]*\([[:space:]]*[0-9]'
PATTERN="(${COLOR}|${SIZE})"

# All Rust files under the scan directory, minus the theme module.
FILES=()
while IFS= read -r file; do
  case "$file" in
    */theme/*|*\\theme\\*) continue ;;
  esac
  FILES+=("$file")
done < <(find "$SCAN_DIR" -type f -name '*.rs' 2>/dev/null || true)

if [ "${#FILES[@]}" -eq 0 ]; then
  echo "check-ui-tokens: OK (0 files checked)"
  exit 0
fi

VIOLATIONS=0
for file in "${FILES[@]}"; do
  if matches=$(grep -nE "$PATTERN" "$file"); then
    echo "check-ui-tokens: FAIL: $file" >&2
    echo "$matches" >&2
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

if [ "$VIOLATIONS" -gt 0 ]; then
  echo "check-ui-tokens: FAIL ($VIOLATIONS file(s)) - colours and sizes must come from the theme module (ADR-0014)." >&2
  exit 1
fi

echo "check-ui-tokens: OK (${#FILES[@]} files checked)"
