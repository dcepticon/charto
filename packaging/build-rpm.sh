#!/bin/sh
# Builds an RPM: dist/charto-<version>-1.<dist>.<arch>.rpm
#
# Run this on Fedora (or use packaging/build-rpm-in-container.sh from any system with podman or
# docker). Needs: cargo, rpm-build, and the Qt 6 development packages listed in charto.spec.
#
# The Rust crates are downloaded once with `cargo vendor` and packed into a second tarball, so the
# build itself needs no network access.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
version=$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)
[ -n "$version" ] || { echo "cannot read the version from Cargo.toml" >&2; exit 1; }
command -v rpmbuild >/dev/null || { echo "rpmbuild is missing (dnf install rpm-build)" >&2; exit 1; }

top=$root/target/rpmbuild
rm -rf "$top"
mkdir -p "$top/SOURCES" "$top/SPECS" "$top/BUILD" "$top/RPMS" "$top/SRPMS" "$top/BUILDROOT"

# source tarball (the folder name inside must be charto-<version>)
tar -C "$root" \
  --exclude=./target --exclude=./dist --exclude=./.git \
  --transform "s,^\./,charto-$version/," \
  -czf "$top/SOURCES/charto-$version.tar.gz" .

# vendored crates
(cd "$root" && cargo vendor --locked "$top/vendor" >/dev/null)
tar -C "$top" -czf "$top/SOURCES/charto-$version-vendor.tar.gz" vendor
rm -rf "$top/vendor"

sed "s/^Version:.*/Version:        $version/" "$here/charto.spec" > "$top/SPECS/charto.spec"
rpmbuild -ba --define "_topdir $top" "$top/SPECS/charto.spec"

mkdir -p "$root/dist"
cp "$top"/RPMS/*/*.rpm "$root/dist/"
echo "Built:"
ls "$root"/dist/*.rpm
