# android-rs-snake
This is a small reimplementation of my [previous snake project](https://github.com/OptimisticPeach/Android-SFML-Snake) (written in C++/SFML), rewritten in rust with piston. 

Here's a breakdown of what I've done:
- Snake moves in a data moving fashion similar to how a snake actually moves
- Corner pieces to the snake to make it seem nicer when it turns
- The snake's body is on a gradient of width for more visual effects
- Apples are randomly spawned and give the snake three body pieces
- Snake wraps around the map, in torus-like fashion
- Portals appear every 5 apples gathered (Called bridges in the implementation)
  - Snake can go over the portal in both directions, but can't use the portal as a place to actually be
  - Portals can't appear beside eachother
- Snake body length is shown in a binary counter at the top of the screen
    - I couldn't get text working, this is going to be changed soon
- Similar colour scheme to that of my original implementation
- Pauses when the app loses its focus
    - Unpauses when tapped
    - Processing the `LostFocus` and `GainedFocus` events fix [#418 in glutin](https://github.com/tomaka/glutin/issues/418)
    - Again I couldn't get text working, so no real gui for now

To run
------------------
Thankfully the `Cargo.toml` file doesn't require too many locally-compiled crates, but it does require you to do the steps outlined in [this issue](https://github.com/PistonDevelopers/glutin_window/issues/154) on `glutin_window` version `"0.48.0"`  
Now steps to run:
- Have [`rustup`](https://rustup.rs/) installed
- Run the following to install the proper toolchains and targets
    - On WSL or Linux
        - `rustup toolchain nightly-x86_64-unknown-linux-gnu`
        - `rustup target add arm-linux-androideabi`
        - Or to add 32 bit support for android: `rustup target add i686-linux-android`
    - Make sure to have installed the proper libraries and toolchains for the appropriate OS you're on, as outlined in [this readme](https://github.com/tomaka/android-rs-glue/blob/master/README.md)
- Download [this commit](https://github.com/PistonDevelopers/glutin_window.git) of `glutin_window` and edit line 58 in `/src/lib.rs` like in [this issue](https://github.com/PistonDevelopers/glutin_window/issues/154)
- Make sure the `Cargo.toml` points to the appropriate directory for `glutin_window` that you just downloaded
- Install `cargo apk`: `cargo install cargo-apk` if you haven't already
- Connect your device of choice (Emulator or physical)
- Run `cargo apk run` in the root of this project (with the `Cargo.toml`)