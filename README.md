# Procreate Thumbnailer for Arch linux
 
- This small Rust app can generate thumbnails for `.procreate` documents. 
- `Procreate` is a painting aplication for iPad. 
- `.procreate` documents are zip files which have a thumbnail generated at export inside a `QuickLook` directory.
- This app just exposes that thumbnail for linux systems.

## Architecture
- Main thumbnailer code is made using rust. 
- Additionally `procreate.xml` and `procreate.thumbnailer` files are installed.


## Installation
### For Arch Linux:
- Now available as an **AUR** package.

`$ yay -S procreate-thumbnailer-git`

- I encourage you to go through the `PKGBUILD` first.

### Alternate:
- Download the `PKGBUILD` from **AUR** 

`yay -G procreate-thumbnailer-git	`

and run:

`$ makepkg -si`

 in the folder containing `PKGBUILD`. 
 
- Installs binary file in `/usr/share/bin`.
- Alternatively you can generate a tarball to distribute it, and install it using 

`$ pacman -U`.

### Build from source code/For other linux distributions:
- Main rust app works for any distro.
- Build the rust app with 

`$ cargo build`

- Install in your path with cargo: 

`$ cargo install --path .`

- `procreate.xml` and `procreate.thumbnailer` files need to be installed manually.

## Uninstall (Arch)
- `$ pacman -R procreate-thumbnailer`

## Quirks
- Currently, there is no support for thumbnail size.

# IMPORTANT  
- **YOU MUST ENABLE THE FILE MANAGER TO SHOW PREVIEWS FOR "PROCREATE ARTWORK" FILES IN ITS SETTINGS.**
- For example, in `dolphin (KDE)` file manager:

```Settings (Ctrl+Shift+,) > Interface > Previews > [✔️] Procreate Artwork```

---

# AUR
- [AUR Link for the package](https://aur.archlinux.org/packages/procreate-thumbnailer-git)
- Please find the `PKGBUILD` file in the AUR.
- The `PKGBUILD` file previously in this repo is no longer supported (To avoid ambiguity).

---

# Disclaimer & LISENCE

- I do not own `PROCREATE` or its propietary file format.
- Everything in this project is disributed under MIT lisence.
- Suggestions are welcome.
