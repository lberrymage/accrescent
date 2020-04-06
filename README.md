# Accrescent [![Build Status](https://travis-ci.com/lberrymage/accrescent.svg?branch=master)](https://travis-ci.com/lberrymage/accrescent)

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
[this issue](https://github.com/amethyst/amethyst/issues/1846) is completely resolved.


[Amethyst game engine]: https://amethyst.rs
