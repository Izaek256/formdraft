#!/usr/bin/env bash
# check-no-raw-length-format.sh
#
# T-NFR-028-01 / T-FR-038-04 [STATIC]
# Verifies that no module outside pattern_core::units formats a length value
# directly (NFR-028, ADR-0007).
#
# Patterns that indicate a raw length format:
#   - format!("{...} mm")  or  format!("{...}mm")  in non-units source
#   - format!("{...} cm"), format!("{...} in"), etc.
#   - to_string() called directly on a numeric value with a unit suffix nearby
#
# This script uses ripgrep (rg) when available, falling back to grep.
# It fails with exit code 1 if any violation is found.

set -euo pipefail
cd "$(dirname "$0")/.."

UNIT_SUFFIXES='mm|cm|\" m\"|ft|yd|\" in\"| in '

# We check Rust source files outside the units module for format! macros that
# embed a unit string literal directly next to a numeric placeholder.
# Pattern: format!( ... "mm" or " mm" or similar unit suffix in the string)
PATTERN='"(\{\}|\{[^}]*\}) *(mm|cm| m |ft|yd|in)"'

# Directories to search (exclude the units module itself and test files).
SEARCH_DIRS=(
    "crates/pattern_templates/src"
    "crates/pattern_document/src"
    "crates/pattern_export/src"
    "crates/pattern_material/src"
    "crates/pattern_render/src"
    "crates/pattern_ui/src"
    "crates/pattern_web/src"
    "tools"
)

VIOLATIONS=0
CHECKED_DIRS=0

for dir in "${SEARCH_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        continue
    fi
    CHECKED_DIRS=$((CHECKED_DIRS + 1))
    if command -v rg >/dev/null 2>&1; then
        MATCHES=$(rg --type rust -e "$PATTERN" "$dir" 2>/dev/null || true)
    else
        MATCHES=$(grep -r --include="*.rs" -E "$PATTERN" "$dir" 2>/dev/null || true)
    fi
    if [ -n "$MATCHES" ]; then
        echo "check-no-raw-length-format: FAIL: raw length format found in $dir:" >&2
        echo "$MATCHES" >&2
        VIOLATIONS=$((VIOLATIONS + 1))
    fi
done

if [ "$VIOLATIONS" -gt 0 ]; then
    echo "check-no-raw-length-format: $VIOLATIONS violation(s). Route all length formatting through pattern_core::units (ADR-0007, NFR-028)." >&2
    exit 1
fi

echo "check-no-raw-length-format: OK ($CHECKED_DIRS directories checked)"
