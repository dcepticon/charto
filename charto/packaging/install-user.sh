#!/bin/sh
# Installs Charto for the current user, following the freedesktop.org specs:
# desktop entry, shared MIME info (so .spf files open with Charto and file
# dialogs can filter by type), icon theme icon, XDG data directory layout.
#
# usage: packaging/install-user.sh [path/to/charto]   (default: target/release/charto)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
bin=${1:-$root/target/release/charto}
[ -x "$bin" ] || { echo "cannot find the charto binary at $bin (run: cargo build -p charto-app --release)" >&2; exit 1; }

data=${XDG_DATA_HOME:-$HOME/.local/share}
mkdir -p "$HOME/.local/bin" "$data/charto"
install -m 755 "$bin" "$HOME/.local/bin/charto"
rm -rf "$data/charto/symbols"
cp -r "$root/assets/symbols" "$data/charto/symbols"

xdg-mime install --mode user "$here/charto-mime.xml"
xdg-icon-resource install --mode user --novendor --size 128 "$here/charto.svg" charto 2>/dev/null \
  || { mkdir -p "$data/icons/hicolor/scalable/apps" && cp "$here/charto.svg" "$data/icons/hicolor/scalable/apps/charto.svg"; }
xdg-desktop-menu install --mode user --novendor "$here/charto.desktop"
xdg-mime default charto.desktop application/x-charto-pattern
command -v update-desktop-database >/dev/null && update-desktop-database "$data/applications" || true

echo "Installed. Make sure \$HOME/.local/bin is in your PATH."
