# Accrescent [![Build Status](https://github.com/lberrymage/accrescent/workflows/CI/badge.svg)](https://github.com/lberrymage/accrescent/actions)

Accrescent is an open-world, modular sandbox game built upon the [Amethyst game
engine]. It aims to be mod-driven through its pubilc API, allowing anyone to
easily create and distribute personal modifications in source or binary
form.

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

### Issues

If you are running under Wayland and are getting a panic with the message "Image
count not supported", you may need to prefix `WINIT_UNIX_BACKEND=x11` until
[this issue](https://github.com/amethyst/amethyst/issues/1846) is completely
resolved.

## Dependencies

If you are compiling on Linux, make sure to install the dependencies below.

### Arch Linux

```
$ pacman -Syu grep gcc pkgconf openssl alsa-lib cmake make python3 freetype2 awk libxcb
```

### Debian/Ubuntu

```
# apt install gcc pkg-config openssl libasound2-dev cmake build-essential python3 libfreetype6-dev libexpat1-dev libxcb-composite0-dev libssl-dev libx11-dev
```

### Fedora

```
# dnf install pkgconfig gcc openssl-devel alsa-lib-devel cmake make gcc-c++ freetype-devel expat-devel libxcb-devel libX11-devel
```

### openSUSE

```
# zypper install gcc pkg-config libopenssl-devel alsa-devel cmake gcc-c++ python3 freetype2-devel libexpat-devel libxcb-devel
```

### Other

See your distribution-specific installation process for the equivalent
dependencies.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
https://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

If you want to contribute, please check out
[CONTRIBUTING.md](CONTRIBUTING.md) for everything you need to know.


[Amethyst game engine]: https://amethyst.rs
