<div align="center">
  <img width="75%" src="https://raw.githubusercontent.com/CompeyDev/bf-rs/main/assets/brainfuck_rs-logo-light.png" />
  <h3>Yet another brainfuck interpreter written in Rust.</h2>
</div>
<br />

# Installation
## Prebuilt binaries
Prebuilt binaries can be found at [GitHub Releases](https://github.com/CompeyDev/bf-rs/releases), simply download the one matching your architecture & platform.

## Building from source 
The source code gets published to [crates.io](https://crates.io/crates/bfrs) on every major release. In order to compile from the published source, first make sure you have the required dependencies installed:
- windres (provided by [mingw-w64](https://www.mingw-w64.org/) `binutils`)
- gcc (provided by [mingw-w64](https://www.mingw-w64.org/))

Next, make use of `cargo install` to compile and install the binary.

```
cargo install bfrs
```

If all succeeded, the compiled binary should be present at `$HOME/.cargo/bin` and be called "bfrs".
