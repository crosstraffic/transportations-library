## About
This is the repository for the knowledge stack of transporation.

## What this covers.

This covers Highway Capacity Manual (HCM) and Green Book.

HCM
- Chapter 15 (Two Lane Highways)

## Test
1. Make sure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

2. Prepare the virtual environment with `uv venv .venv`.

3. To activate the environment, type (Ubuntu) `source .venv/bin/activate` and (Windows) `.venv\Scripts\activate`.

4. Then `maturin develop` to build the Rust library and make it available in Python.

5. Run the tests with `cargo test` for Rust, and `pytest ./tests` for Python wrapper.

Note: If you want to have changes in the Rust code to be reflected in Python, you need to run `cargo clean` and `maturin develop` again after making changes.

## TODO

Add new chapter on HCM.