# Procreate Thumbnailer for Arch linux
 
- This small Rust app can generate thumbnails for `.procreate` documents. 
- `Procreate` is a painting aplication for iPad. 
- `.procreate` documents are zip files which have a thumbnail generated at export inside a `QuickLook` directory.
- This app just exposes that thumbnail for linux systems.

## Architecture
- Main thumbnailer code is made using rust. 
- Additionally `procreate.xml` and `procreate.thumbnailer` files are installed.


## Installation
### For Arch
- Download the package and run `$ makepkg -si` in the folder containing `PKGBUILD`.
- Installs binary file in `/usr/share/bin`.
- Alternatively you can generate a tarball to distribute it, and install it using `$ pacman -U`.

### For other linux distributions
- Main rust app works for any distro.
- `procreate.xml` and `procreate.thumbnailer` files need to be installed manually.

## Uninstall
- `$ pacman -R procreate-thumbnailer`

## Quirks
- Currently, there is no support for thumbnail size.

---

# Disclaimer & LISENCE

- I do not own `PROCREATE` or its propietary file format.
- Everything in this project is disributed under MIT lisence.
- Suggestions are welcome.