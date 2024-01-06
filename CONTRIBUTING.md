# Contributing

All pull requests are welcome.

## TODO

1. TCP

## Run CI locally

### Requirements

1. [cargo-make](https://github.com/sagiegurari/cargo-make#installation)

### Run

Go to the project root and run:
```sh
cargo make all
```

### pre-commit hook

Install [pre-commit](https://pre-commit.com/#install)
```sh
pre-commit install
```

Add changes
```sh
git add .
```

The hook will be triggered automatically at each commit.
