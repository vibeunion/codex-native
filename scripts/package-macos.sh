#!/bin/bash
set -euo pipefail

binary_dir="${1:?usage: package-macos.sh BINARY_DIR OUTPUT_DIR VERSION}"
binary_dir="$(cd "$binary_dir" && pwd)"
output_dir="${2:?output directory required}"
version="${3:?version required}"
script_dir="$(cd "$(dirname "$0")" && pwd)"
app_name="${APP_BRAND_NAME:-Codex Native}"
app_dir_name="${app_name//\//-}.app"
app="$output_dir/$app_dir_name"
signing_identity="${MACOS_SIGNING_IDENTITY:-}"
require_signing="${MACOS_REQUIRE_SIGNING:-0}"
require_notarization="${MACOS_REQUIRE_NOTARIZATION:-0}"
notarization_requested=0

if [[ "$require_signing" == "1" && -z "$signing_identity" ]]; then
    printf '%s\n' "MACOS_SIGNING_IDENTITY is required for a release package" >&2
    exit 1
fi
if [[ "$require_notarization" == "1" || (
    -n "${APPLE_ID:-}" &&
    -n "${APPLE_TEAM_ID:-}" &&
    -n "${APPLE_APP_PASSWORD:-}"
) ]]; then
    notarization_requested=1
    [[ -n "$signing_identity" ]] || {
        printf '%s\n' "A signed app is required before notarization" >&2
        exit 1
    }
    [[ -n "${APPLE_ID:-}" && -n "${APPLE_TEAM_ID:-}" && -n "${APPLE_APP_PASSWORD:-}" ]] || {
        printf '%s\n' "APPLE_ID, APPLE_TEAM_ID and APPLE_APP_PASSWORD are required for notarization" >&2
        exit 1
    }
fi

[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]] || exit 1
[[ ! -e "$app" ]] || { printf '%s\n' "Output app already exists: $app" >&2; exit 1; }
for binary in volt office-cli calc-cli; do
    [[ -x "$binary_dir/$binary" ]] || { printf '%s\n' "Missing executable: $binary" >&2; exit 1; }
done

if [[ -n "$signing_identity" ]]; then
    for binary in volt office-cli calc-cli; do
        codesign --force --options runtime --timestamp \
            --sign "$signing_identity" "$binary_dir/$binary"
    done
fi

mkdir -p "$app/Contents/MacOS"
cp "$binary_dir/volt" "$binary_dir/office-cli" "$binary_dir/calc-cli" "$app/Contents/MacOS/"
cp "$script_dir/macos/Info.plist" "$app/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $version" "$app/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $version" "$app/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleName $app_name" "$app/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleDisplayName $app_name" "$app/Contents/Info.plist"
plutil -lint "$app/Contents/Info.plist"

if [[ -n "$signing_identity" ]]; then
    codesign --force --options runtime --timestamp \
        --sign "$signing_identity" "$app"
fi

if [[ "$notarization_requested" == "1" ]]; then
    notarization_dir="$(mktemp -d "${TMPDIR:-/tmp}/volt-notarization.XXXXXX")"
    notarization_zip="$notarization_dir/${app_dir_name%.app}.zip"
    trap 'rm -rf "$notarization_dir"' EXIT
    ditto -c -k --keepParent "$app" "$notarization_zip"
    xcrun notarytool submit "$notarization_zip" \
        --apple-id "$APPLE_ID" \
        --team-id "$APPLE_TEAM_ID" \
        --password "$APPLE_APP_PASSWORD" \
        --wait
    xcrun stapler staple "$app"
    codesign --verify --deep --strict --verbose=2 "$app"
fi

tar -czf "$output_dir/volt-macos-arm64.tar.gz" \
    -C "$output_dir" "$app_dir_name" \
    -C "$binary_dir" volt office-cli calc-cli
