#!/usr/bin/env bash

set -x
set -e

# Rust CLI tool
cargo build --bin go-outside

# Lambda build:
npm install
npm test
zip -r go-outside-lambda.zip ./src ./package.json ./package-lock.json ./node_modules
