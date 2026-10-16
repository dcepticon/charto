#!/bin/sh
# Builds a Debian/Ubuntu package: dist/charto.deb
#
# usage: packaging/build-deb.sh [path/to/charto]
#   without an argument the release binary is built first (cargo build -p charto-app --release)
#
# Needs: cargo (unless you pass a binary), dpkg-dev (dpkg-deb, dpkg-shlibdeps).
# Optional: CHARTO_DEB_SUFFIX is appended to the version (build-deb-in-container.sh uses it to
# tell the packages for different Ubuntu releases apart), e.g. CHARTO_DEB_SUFFIX="~ubuntu26.04".
# DEBFULLNAME and DEBEMAIL set the maintainer line, e.g.
#   DEBFULLNAME="Your Name" DEBEMAIL=you@example.org packaging/build-deb.sh
set -e
umask 022
here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")

version=$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)
[ -n "$version" ] || { echo "cannot read the version from Cargo.toml" >&2; exit 1; }
arch=$(dpkg --print-architecture)
revision=2
full="$version-$revision${CHARTO_DEB_SUFFIX:-}"

bin=$1
if [ -z "$bin" ]; then
  (cd "$root" && cargo build -p charto-app --release)
  bin=$root/target/release/charto
fi
[ -x "$bin" ] || { echo "cannot find the charto binary at $bin" >&2; exit 1; }

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
pkg=$work/pkg

# ---- the files -------------------------------------------------------------
install -D -m 755 "$bin" "$pkg/usr/bin/charto"
strip --strip-unneeded "$pkg/usr/bin/charto" 2>/dev/null || true

mkdir -p "$pkg/usr/share/charto"
cp -r "$root/assets/symbols" "$pkg/usr/share/charto/symbols"
find "$pkg/usr/share/charto" -type d -exec chmod 755 {} +
find "$pkg/usr/share/charto" -type f -exec chmod 644 {} +

install -D -m 644 "$here/charto.desktop"  "$pkg/usr/share/applications/charto.desktop"
install -D -m 644 "$here/charto.svg"      "$pkg/usr/share/icons/hicolor/scalable/apps/charto.svg"
install -D -m 644 "$here/charto-mime.xml" "$pkg/usr/share/mime/packages/charto.xml"
# AppStream metadata: this is what software centres (Discover, GNOME Software) show as the
# author, licence, home page and name. It is the Flatpak file with the native desktop-file name.
sed 's,<launchable type="desktop-id">org.charto.Charto.desktop</launchable>,<launchable type="desktop-id">charto.desktop</launchable>,' \
  "$here/flatpak/org.charto.Charto.metainfo.xml" > "$work/org.charto.Charto.metainfo.xml"
install -D -m 644 "$work/org.charto.Charto.metainfo.xml" "$pkg/usr/share/metainfo/org.charto.Charto.metainfo.xml"

doc=$pkg/usr/share/doc/charto
mkdir -p "$doc"
cat > "$doc/copyright" <<COPYRIGHT
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: Charto
Upstream-Contact: Claude and Fibrely Designs
Comment: Charto is a Rust + Qt 6 port of sconcho, the knitting chart designer
 written by Markus Dittrich (https://github.com/craftoid/sconcho).

Files: *
Copyright: 2026 Claude and Fibrely Designs
License: GPL-3+

License: GPL-3+
 This program is free software: you can redistribute it and/or modify it
 under the terms of the GNU General Public License as published by the
 Free Software Foundation, either version 3 of the License, or (at your
 option) any later version.
 .
 On Debian systems, the full text of the GNU General Public License
 version 3 can be found in /usr/share/common-licenses/GPL-3.
COPYRIGHT
chmod 644 "$doc/copyright"
printf 'charto (%s) unstable; urgency=low\n\n  * Packaged with packaging/build-deb.sh.\n\n -- %s <%s>  %s\n' \
  "$full" "${DEBFULLNAME:-Claude and Fibrely Designs}" "${DEBEMAIL:-info@fibrely.com}" "$(date -R)" \
  | gzip -9n > "$doc/changelog.Debian.gz"
chmod 644 "$doc/changelog.Debian.gz"

# ---- run-time dependencies --------------------------------------------------
# shared libraries: found by dpkg-shlibdeps (needs a debian/control stub)
mkdir -p "$work/debian"
printf 'Source: charto\n\nPackage: charto\nArchitecture: any\n' > "$work/debian/control"
libdeps=$(cd "$work" && dpkg-shlibdeps -O -e"$pkg/usr/bin/charto" 2>/dev/null | sed -n 's/^shlibs:Depends=//p')
[ -n "$libdeps" ] || echo "warning: could not work out the library dependencies" >&2

# QML modules are loaded at run time, so they are not linked and must be listed
qml="qml6-module-qtquick, qml6-module-qtquick-controls, qml6-module-qtquick-dialogs, qml6-module-qtquick-layouts, qml6-module-qtquick-templates, qml6-module-qtquick-window, qml6-module-qtqml, libqt6svg6"
depends=$qml
[ -n "$libdeps" ] && depends="$libdeps, $qml"

# ---- control file -------------------------------------------------------------
mkdir -p "$pkg/DEBIAN"
size=$(du -sk "$pkg" | cut -f1)
cat > "$pkg/DEBIAN/control" <<CONTROL
Package: charto
Version: $full
Architecture: $arch
Maintainer: ${DEBFULLNAME:-Claude and Fibrely Designs} <${DEBEMAIL:-info@fibrely.com}>
Installed-Size: $size
Depends: $depends
Recommends: xdg-desktop-portal, xdg-desktop-portal-kde | xdg-desktop-portal-gtk | xdg-desktop-portal-gnome
Section: graphics
Homepage: https://dcepticon.github.io/charto/
Priority: optional
Description: knitting chart designer
 Charto designs knitting charts and patterns: place stitch symbols on a
 grid, colour cells, repeat rows, number rows and stitches, and export or
 print the chart. It opens and saves sconcho (.spf) pattern files.
 .
 Open, save and print use the desktop's own dialogs through the freedesktop
 portals. Charto is a Rust and Qt 6 port of sconcho.
CONTROL

# a plain file name: software centres show the name up to the first dot ("charto_3" for charto_3.1.0-...)
out=${CHARTO_DEB_OUT:-$root/dist/charto.deb}
mkdir -p "$(dirname "$out")"
dpkg-deb --root-owner-group --build "$pkg" "$out" >/dev/null
echo "Built $out"
echo "Install with: sudo apt install ./${out#$PWD/}"
