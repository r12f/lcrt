#!/usr/bin/env just --justfile

COLOR_GREEN := '(ansi green)'
COLOR_BLUE := '(ansi blue)'
COLOR_RED := '(ansi r)'
COLOR_NONE := '(ansi reset)'

set shell := ["nu", "-c"]

build:
    cargo build

test:
    cargo test

#
# Development tasks:
#
commit m:
    git add .
    git commit -m "{{m}}"
    git push

#
# Lint tasks:
#
fix: format lint-fix

format:
    cargo fmt -- --emit files

lint:
    cargo clippy --all-targets --all-features

lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty