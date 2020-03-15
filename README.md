# work_in_progress [![Build Status](https://travis-ci.com/lberrymage/work_in_progress.svg?branch=master)](https://travis-ci.com/lberrymage/work_in_progress)

## What is this?

This repository is for my personal experimentation with the [amethyst game engine](https://amethyst.rs).

Everything here is experimental and has no definite direction, and the
repository name is very subject to change. I have made this public so that

1.) I can use CIs for free

2.) I can accept suggestions from others around whatever
subject I come across, and

3.) Everyone can see what I'm doing in case they have an interest in how I
solved a common but annoying problem.

Thanks for checking everything out, but know that this project isn't going
anywhere at the moment, so hold tight.

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
