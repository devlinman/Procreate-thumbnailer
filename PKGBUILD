pkgname=procreate-thumbnailer
pkgver=1.0.0
pkgrel=1
pkgdesc="Thumbnailer for Procreate (.procreate) files"
arch=('x86_64')
license=('MIT')
depends=('shared-mime-info')
makedepends=('cargo' 'rust')
install=procreate-thumbnailer.install


build() {
    cd "$srcdir"
    echo ">>> Building Rust binary..."
    cargo build --release
}

package() {
    cd "$srcdir/../"
    # Ensure directories exist
    install -d "$pkgdir/usr/bin"

    install -Dm755 "target/release/procreate-thumbnailer" "$pkgdir/usr/bin/procreate-thumbnailer"

    install -Dm644 "procreate.thumbnailer" "$pkgdir/usr/share/thumbnailers/procreate.thumbnailer"

    install -Dm644 "procreate.xml" "$pkgdir/usr/share/mime/packages/procreate.xml"
}
