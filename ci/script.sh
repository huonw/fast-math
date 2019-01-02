#!/usr/bin/env bash
set -ex

cargo=cargo
target_param=""
features=""
if [ ! -z "$UNSTABLE" ]; then
    features+=" unstable"
fi
if [ ! -z "$TARGET" ]; then
    rustup target add "$TARGET"
    cargo install -v cross --force
    cargo="cross"
    target_param="--target $TARGET"
fi

$cargo build -v $target_param --features "$features"
$cargo test -v $target_param --features "$features"

# for now, `cross bench` is broken https://github.com/rust-embedded/cross/issues/239
if [ "$cargo" != "cross" ]; then
    $cargo bench -v $target_param --features "$features" -- --test # don't actually record numbers
fi

$cargo doc -v $target_param --features "$features"

$cargo test -v --release

if [ ! -z "$COVERAGE" ]; then
    if [ ! -z "$TARGET" ]; then
        echo "cannot record coverage while cross compiling"
        exit 1
    fi

    cargo install -v cargo-travis || echo "cargo-travis already installed"
    cargo coverage -v -m coverage-reports --kcov-build-location "$PWD/target" --features "$features"
    bash <(curl -s https://codecov.io/bash) -c -X gcov -X coveragepy -s coverage-reports
fi
