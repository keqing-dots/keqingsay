# Maintainer: hieubuiduc1@gmail.com
pkgname=keqingsay
pkgver=0.1.0
pkgrel=1
pkgdesc="A cowsay-like CLI tool featuring Keqing ASCII art animations"
arch=('x86_64')
url="https://github.com/keqing-dots/keqingsay.git"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
