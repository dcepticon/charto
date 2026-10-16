# RPM spec for Charto (Fedora). Build with packaging/build-rpm.sh, which prepares the two
# source tarballs (the source tree and the vendored Rust crates) and runs rpmbuild.
%global debug_package %{nil}

Name:           charto
Version:        3.1.0
Release:        1%{?dist}
Summary:        Knitting chart designer
License:        GPL-3.0-or-later
URL:            https://dcepticon.github.io/charto/
Packager:       Claude and Fibrely Designs <info@fibrely.com>
Source0:        charto-%{version}.tar.gz
Source1:        charto-%{version}-vendor.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  gcc-c++
BuildRequires:  qt6-qtbase-devel
BuildRequires:  qt6-qtdeclarative-devel
BuildRequires:  qt6-qtsvg-devel
BuildRequires:  desktop-file-utils

# the QML modules (Controls, Dialogs, Layouts, ...) are in qt6-qtdeclarative; the symbols are SVG
Requires:       qt6-qtdeclarative
Requires:       qt6-qtsvg
Recommends:     xdg-desktop-portal

%description
Charto designs knitting charts and patterns: place stitch symbols on a grid,
colour cells, repeat rows, number rows and stitches, and export or print the
chart. It opens and saves sconcho (.spf) pattern files.

Open, save and print use the desktop's own dialogs through the freedesktop
portals. Charto is a Rust and Qt 6 port of sconcho.

%prep
%autosetup -n charto-%{version} -a 1
mkdir -p .cargo
cat > .cargo/config.toml <<CARGOCONFIG
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
CARGOCONFIG

%build
cargo build --release --frozen -p charto-app

%install
install -Dm755 target/release/charto %{buildroot}%{_bindir}/charto
mkdir -p %{buildroot}%{_datadir}/charto
cp -a assets/symbols %{buildroot}%{_datadir}/charto/symbols
install -Dm644 packaging/charto.desktop %{buildroot}%{_datadir}/applications/charto.desktop
install -Dm644 packaging/charto.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/charto.svg
install -Dm644 packaging/charto-mime.xml %{buildroot}%{_datadir}/mime/packages/charto.xml
sed 's,<launchable type="desktop-id">org.charto.Charto.desktop</launchable>,<launchable type="desktop-id">charto.desktop</launchable>,' \
  packaging/flatpak/org.charto.Charto.metainfo.xml > org.charto.Charto.metainfo.xml
install -Dm644 org.charto.Charto.metainfo.xml %{buildroot}%{_datadir}/metainfo/org.charto.Charto.metainfo.xml

%check
desktop-file-validate %{buildroot}%{_datadir}/applications/charto.desktop

%files
%license COPYING
%doc README.md
%{_bindir}/charto
%{_datadir}/charto/
%{_datadir}/applications/charto.desktop
%{_datadir}/icons/hicolor/scalable/apps/charto.svg
%{_datadir}/mime/packages/charto.xml
%{_datadir}/metainfo/org.charto.Charto.metainfo.xml

%changelog
* Tue Oct 13 2026 Claude and Fibrely Designs <info@fibrely.com> - 3.1.0-1
- Packaged with packaging/build-rpm.sh
