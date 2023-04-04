#!/usr/bin/env bash
cargo build --release
cargo install sqlx-cli
sqlx database setup
