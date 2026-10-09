#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

PY=""
for c in python3 python; do
  if command -v "$c" >/dev/null 2>&1; then
    PY="$c"
    break
  fi
done
if [ -z "$PY" ]; then
  echo "check-layering: python3 or python is required" >&2
  exit 1
fi

cargo metadata --format-version 1 --no-deps | "$PY" -c '
import json
import sys

allowed = {
    "pattern_core": set(),
    "pattern_templates": {"pattern_core"},
    "pattern_document": {"pattern_core", "pattern_templates"},
    "pattern_export": {"pattern_core"},
    "pattern_material": {"pattern_core"},
    "pattern_render": {"pattern_core"},
    "pattern_ui": {
        "pattern_core",
        "pattern_templates",
        "pattern_document",
        "pattern_export",
        "pattern_material",
        "pattern_render",
    },
    "pattern_web": {
        "pattern_core",
        "pattern_templates",
        "pattern_document",
        "pattern_export",
        "pattern_material",
        "pattern_render",
        "pattern_ui",
    },
    "pattern-cli": {
        "pattern_core",
        "pattern_templates",
        "pattern_document",
        "pattern_export",
    },
}

meta = json.load(sys.stdin)
violations = []
for pkg in meta["packages"]:
    name = pkg["name"]
    if name not in allowed:
        violations.append("%s: crate is not in the layering matrix (docs/architecture/01-modules-and-boundaries.md)" % name)
        continue
    deps = {d["name"] for d in pkg["dependencies"] if d["name"] in allowed}
    forbidden = deps - allowed[name]
    for dep in sorted(forbidden):
        violations.append("%s may not depend on %s" % (name, dep))

if violations:
    for v in violations:
        print("check-layering: FAIL: %s" % v, file=sys.stderr)
    sys.exit(1)
print("check-layering: OK (%d crates)" % len(meta["packages"]))
'
