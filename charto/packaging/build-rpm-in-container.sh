#!/bin/sh
# Builds the RPM inside a Fedora container, so it works from Ubuntu or any other system.
#
# usage: packaging/build-rpm-in-container.sh
# Needs podman or docker (Ubuntu: sudo apt install podman). The first run downloads Fedora and the
# build tools, which takes a while. The result is dist/charto-*.rpm.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
engine=$(command -v podman || command -v docker || true)
[ -n "$engine" ] || { echo "install podman or docker first (sudo apt install podman)" >&2; exit 1; }

mkdir -p "$root/dist"
"$engine" run --rm -v "$root":/src:Z docker.io/library/fedora:latest sh -ec '
  dnf install -y rpm-build cargo rust gcc-c++ make tar gzip desktop-file-utils \
      qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtsvg-devel
  rm -rf /build && mkdir /build
  tar -C /src --exclude=./target --exclude=./dist --exclude=./.git -cf - . | tar -C /build -xf -
  cd /build
  packaging/build-rpm.sh
  cp dist/*.rpm /src/dist/
'
echo "Done. The package is in $root/dist/"
