# android-rs-snake
This is a reimplementation of my [previous snake project](https://github.com/OptimisticPeach/Android-SFML-Snake) (written in C++/SFML), rewritten in rust with piston. 

Here's a breakdown of what I've done:
- Snake moves in a data moving fashion similar to how a snake actually moves.
- Corner pieces to the snake to make it seem nicer when it turns.
- The snake's body is on a gradient of width for more visual effects.
- Apples are randomly spawned and give the snake three body pieces.
- Snake wraps around the map, in a fashion topologically equal to the inside of a torus.
- Portals appear every 5 apples gathered. (Named bridges in the implementation)
  - Snake can go over the portal in both directions, but can't use the portal as a place to actually be.
  - Portals can't appear beside eachother.
- A number counter for the score at the top of the screen.
- Similar colour scheme to that of my original implementation.
- Pauses when the app loses its focus.
    - Unpauses when tapped.
    - Processing the `LostFocus` and `GainedFocus` events fix [#418 in glutin](https://github.com/tomaka/glutin/issues/418)
    - No real gui for now.
- When your body is on an edge, a green 'ghost' appears on the other side to warn you to not go there.
- 2 player mode
    - Rotate to landscape to activate
    - There is a green and cyan player, and a red and purple player.
    - Rules here:
        - Each player can collect an apple to grow
        - If either player collides with the opposing player, or if they collide with themself, then the opposing player wins
        - Actions started on the right apply to the green and cyan player
        - Actions started on the left apply to the red and purple player

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
    - Note, that this version is necessary because of how this is pieced together, even though this was changed already in the newest version of `glutin_window` 
- Make sure the `Cargo.toml` points to the appropriate directory for `glutin_window` that you just downloaded
- Install  an appropriate font into `/fonts/`, and change the font referred to in `/src/app/mod.rs` in `App::new()`
- Install `cargo apk`: `cargo install cargo-apk` if you haven't already
- Connect your device of choice (Emulator or physical)
- Run `cargo apk run` in the root of this project (with the `Cargo.toml`)
  
If you want to remove the navigation and decoration bars at the top (and potentially bottom) of the screen: 
- Clone `android-rs-glue`
- Navigate to `cargo-apk/src/ops/build.rs`
- At around line 440, there will be a string containing java code, change the java code to the one in `nativeactivity.java` in the root of this project
- Compile, and replace the `cargo-apk[.exe]` in `~/.cargo/bin/` with the one in `./target/debug/cargo-apk[.exe]`
