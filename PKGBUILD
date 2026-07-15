# Maintainer: hieubuiduc1@gmail.com
pkgname=keqingsay
pkgver=0.1.0
pkgrel=1
pkgdesc="A cowsay-like CLI tool featuring Keqing ASCII art animations"
arch=('x86_64')
url="https://github.com/keqing-dots/keqingsay"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
options=('!debug')
source=()
sha256sums=()

build() {
    cargo build --release --manifest-path "$startdir/Cargo.toml" --target-dir "$startdir/target"
}

package() {
    install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    rm -rf "$startdir/target"
    rm -f "$startdir/Cargo.lock"
}
