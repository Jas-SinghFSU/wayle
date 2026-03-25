#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

ICON_DIR="$PROJECT_DIR/resources/icons/hicolor/scalable/actions"
CODE_DIRS=("$PROJECT_DIR/crates" "$PROJECT_DIR/wayle")
ICON_PATTERN='"(ld|tb|tbf|md|cm|si)-[a-zA-Z0-9_-]+-symbolic"'

IGNORE=(
    "cm-wayle-symbolic"
)

available=$(
    find "$ICON_DIR" -name "*.svg" -exec basename {} .svg \; | sort
)

referenced=$(
    grep -rohE --include="*.rs" "$ICON_PATTERN" "${CODE_DIRS[@]}" 2>/dev/null |
        tr -d '"' |
        sort -u ||
        true
)

is_ignored() {
    local name="$1"

    for ignored in "${IGNORE[@]}"; do
        [[ "$name" == "$ignored" ]] && return 0
    done

    return 1
}

code_references() {
    local name="$1"

    grep -rn --include="*.rs" "\"$name\"" "${CODE_DIRS[@]}" 2>/dev/null |
        grep -v '/// ' |
        grep -v '//!' |
        grep -v '#\[doc' ||
        true
}

errors=0

while IFS= read -r icon; do
    [[ -z "$icon" ]] && continue
    is_ignored "$icon" && continue
    echo "$available" | grep -qx "$icon" && continue

    loc=$(code_references "$icon" | head -1)
    [[ -z "$loc" ]] && continue

    file=$(echo "$loc" | cut -d: -f1)
    line=$(echo "$loc" | cut -d: -f2)
    rel="${file#"$PROJECT_DIR/"}"

    echo "::error file=${rel},line=${line}::${icon}.svg missing from resources/icons/"

    errors=$((errors + 1))
done <<<"$referenced"

if ((errors > 0)); then
    echo "${errors} missing icon(s) in resources/icons/"
    exit 1
fi

echo "Icons OK"
