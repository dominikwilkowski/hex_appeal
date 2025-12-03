## Hex Appeal

For Raquel

<3 Dom and Yurm

## Requirements

- Install rust
- Install WASM target `rustup target add wasm32-unknown-unknown`
- Install cargo-generate, trunk and leptosfmt `cargo install cargo-generate trunk leptosfmt --locked`

## Development

Dev locally:

```sh
trunk serve
```

Before committing make sure you run:
```sh
leptosfmt ./**/*.rs && cargo fmt -- -l
```

## Release

Locally you can run:

```sh
trunk build --release
```

For a release to the domain, we use GitHub action which will be triggered by a
GitHub release.
