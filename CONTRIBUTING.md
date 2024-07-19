# Contributing

## Local environment setup

1. Ensure you have an up-to-date version of Rust (see: [rust-toolchain.toml](./rust-toolchain.toml)).
2. Install [pre-commit] hooks: `pre-commit install -t pre-commit -t commit-msg`.

[pre-commit]: https://pre-commit.com/

## Style guide

> [!NOTE]
> This guide is work in progress and will be updated as we think of things to add. For now, Rust code will use the
> default [`rustfmt` rules][1], but we might introduce a `rustfmt.toml` to override certain rules.

To keep documents and code well formatted and consistent between developers, we make use of pre-commit hooks. Please
install them speed up the development loop.

The hooks will also run in CI for pull requests.

[1]: https://rust-lang.github.io/rustfmt/
