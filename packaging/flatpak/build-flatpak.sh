#!/bin/sh
# Builds Charto as a Flatpak and installs it for the current user.
#
# usage: packaging/flatpak/build-flatpak.sh [--bundle]
#   --bundle   also write dist/charto.flatpak, a single file you can give to others
#
# One-time setup (Ubuntu):
#   sudo apt install flatpak flatpak-builder
#   flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
#   flatpak install --user flathub org.kde.Platform//6.10 org.kde.Sdk//6.10 \
#       org.freedesktop.Sdk.Extension.rust-stable//25.08
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$(dirname "$here")")
app=org.charto.Charto

command -v flatpak-builder >/dev/null || { echo "flatpak-builder is not installed (sudo apt install flatpak-builder)" >&2; exit 1; }

# regenerate the crate list so it always matches Cargo.lock
python3 "$here/generate-cargo-sources.py"

mkdir -p "$root/target" "$root/dist"
flatpak-builder --user --install --force-clean \
  --state-dir="$root/target/flatpak-state" \
  --install-deps-from=flathub \
  "$root/target/flatpak-build" "$here/$app.yaml"

if [ "$1" = "--bundle" ]; then
  flatpak-builder --user --force-clean --disable-rofiles-fuse \
    --state-dir="$root/target/flatpak-state" \
    --repo="$root/target/flatpak-repo" \
    "$root/target/flatpak-build" "$here/$app.yaml"
  flatpak build-bundle "$root/target/flatpak-repo" "$root/dist/charto.flatpak" "$app"
  echo "Wrote $root/dist/charto.flatpak"
fi

echo "Installed. Run it with: flatpak run $app"
