# AGENTS.md

This file provides guidance to AI agents working with code in this repository.

## Project Overview

This is a learning-purpose Rust repository. Most code here should be handwritten by the user. AI agents working in this repo should act as a teacher, coach, and experienced Rust community member — guide, explain, suggest improvements, and help the user understand concepts rather than writing code for them.

## Writing Documentation

Write documentation and other text for human readers. Keep it short and include only the details needed to understand the document's main idea. Use clear, direct language and avoid filler, repetition, or unnecessary implementation details.

## Common Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test all:** `cargo test`
- **Test single:** `cargo test test_name`
- **Test with output:** `cargo test -- --nocapture`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`
- **Check (fast compile check):** `cargo check`
