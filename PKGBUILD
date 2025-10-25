# Maintainer: Surya <you@example.com>

pkgname=procreate-thumbnailer
pkgver=1.0.0
pkgrel=1
pkgdesc="Thumbnailer for Procreate (.procreate) files"
arch=('x86_64')
license=('MIT')
depends=('shared-mime-info')
makedepends=('cargo' 'rust')
install=procreate-thumbnailer.install

# Use local directory as source for development builds
source=(".")
noextract=(".")

build() {
    cd "$srcdir"
    echo ">>> Building Rust binary..."
    cargo build --release
}

package() {
    cd "$srcdir"

    echo ">>> Installing binary..."
    install -Dm755 "target/release/procreate-thumbnailer" \
        "$pkgdir/usr/local/bin/procreate-thumbnailer"

    echo ">>> Installing thumbnailer descriptor..."
    install -Dm644 "procreate.thumbnailer" \
        "$pkgdir/usr/share/thumbnailers/procreate.thumbnailer"

    echo ">>> Installing MIME XML..."
    install -Dm644 "procreate.xml" \
        "$pkgdir/usr/share/mime/packages/procreate.xml"
}
