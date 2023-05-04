# This is a game for May

Run `cargo run` in the root directory.
It should compile and run the code if you have rust and cargo installed. If not, see https://www.rust-lang.org/learn/get-started.

# WASM

To compile and run a web-based version, use `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner cargo run --target wasm32-unknown-unknown`. This requires that you have previously set up the WASM toolchain with:
- `rustup target add wasm32-unknown-unknown`
- `cargo install wasm-server-runner`
    - This needs to install to somewhere on your `PATH`. Consider adding e.g. `--root=$HOME/.local`.

See [docs](https://bevy-cheatbook.github.io/platforms/wasm.html) for more. Note that the suggested addition of `runner` to `Cargo.toml` doesn't work, so we use the `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER`, as seen above.
