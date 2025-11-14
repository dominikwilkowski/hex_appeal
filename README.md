## Hex Appeal

For Raquel

<3 Dom and Yurm

## Requirements

- Install rust nightly `rustup toolchain install nightly --allow-downgrade`
- Install WASM target `rustup target add wasm32-unknown-unknown`
- Install cargo-generate, trunk and leptosfmt `cargo install cargo-generate trunk leptosfmt --locked`

## Development

Dev locally:

```sh
trunk serve
```

Before committing make sure you run:
```sh
leptosfmt ./**/*.rs
```

## Release

Locally you can run:

```sh
trunk build --release
```

For a real release we GitHub action which will be triggered by a release.
