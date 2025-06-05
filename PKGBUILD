# Maintainer: L4z3x <moussaousselmal1970@gmail.com>
pkgname=mal-cli
pkgver=1.0.0
pkgrel=1
pkgdesc="A powerful CLI tool for MyAnimeList"
arch=('x86_64')
url="https://github.com/L4z3x/mal-tui"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')  

build() {
  cd "$pkgname-$pkgver"
  make build-release 
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  
}