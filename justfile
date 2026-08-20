# Compile every Rust package without producing release binaries.
default: check

check:
    #!/usr/bin/env bash
    set -euo pipefail
    shopt -s globstar nullglob

    for manifest in programming-rust/**/Cargo.toml practice/**/Cargo.toml; do
        echo "Checking ${manifest%/Cargo.toml}"
        cargo check --locked --manifest-path "$manifest"
    done
