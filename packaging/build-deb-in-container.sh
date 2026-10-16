#!/bin/sh
# Builds one .deb per Ubuntu release, each inside a clean container of that release, so every
# package is linked against - and depends on - that release's own Qt.
#
# usage: packaging/build-deb-in-container.sh [release ...]
#   default releases: 24.04 25.10 26.04      (e.g. a newer one: packaging/build-deb-in-container.sh 26.10)
#
# Needs podman or docker (Ubuntu: sudo apt install podman) and an internet connection. It takes a
# while: each release downloads Ubuntu, Qt, Rust and the crates. A release that fails to build does
# not stop the others. The results are dist/ubuntu-<release>/charto.deb; install the one matching
# your release, e.g.: sudo apt install ./dist/ubuntu-26.04/charto.deb
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
engine=$(command -v podman || command -v docker || true)
[ -n "$engine" ] || { echo "install podman or docker first (sudo apt install podman)" >&2; exit 1; }
[ $# -gt 0 ] || set -- 24.04 25.10 26.04

mkdir -p "$root/dist"
built=""
failed=""
for rel in "$@"; do
  echo
  echo "=== Ubuntu $rel ==="
  if "$engine" run --rm \
      -e DEBIAN_FRONTEND=noninteractive -e REL="$rel" \
      -e DEBFULLNAME="${DEBFULLNAME:-}" -e DEBEMAIL="${DEBEMAIL:-}" \
      -v "$root":/src:Z "docker.io/library/ubuntu:$rel" sh -ec '
    apt-get update -qq
    apt-get install -y -qq build-essential pkg-config curl ca-certificates dpkg-dev lld \
        qt6-base-dev qt6-declarative-dev qt6-declarative-dev-tools libgl1-mesa-dev
    curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal >/dev/null
    . "$HOME/.cargo/env"
    rm -rf /build && mkdir /build
    tar -C /src --exclude=./target --exclude=./dist --exclude=./.git -cf - . | tar -C /build -xf -
    cd /build
    CHARTO_DEB_SUFFIX="~ubuntu$REL" packaging/build-deb.sh
    mkdir -p "/src/dist/ubuntu-$REL"
    cp dist/charto.deb "/src/dist/ubuntu-$REL/charto.deb"
  '; then
    built="$built $rel"
  else
    failed="$failed $rel"
  fi
done

echo
[ -z "$built" ] || echo "Built for Ubuntu:$built  (see $root/dist/)"
[ -z "$failed" ] || echo "FAILED for Ubuntu:$failed  (the error is above)"
[ -z "$failed" ]
