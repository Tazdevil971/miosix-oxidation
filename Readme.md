# Miosix-oxidation
Welcome to miosix-oxidation! Rust experimental miosix support

## Status
Many feature are currently being worked on, here is a list showing what you can/cannot use:

| feature | status |
| ------- | ------ |
| `core` | working
| `std::time` | working
| `std::alloc` | working
| `std::fs` | working (mostly)
| `std::io` | working
| `panic!` | stops and does nothing
| `main()` | no way
| miosix bindings | not yet
| everything else | doesn't work

The rule of thumb is: if it shouldn't work on miosix, then it doesn't (even if the compiler says otherwise).

Also the currenlty supported targets:
- `thumbv7em-miosix-eabihf`

## Usage
To compile this test you will need to have a special version of rustc installed that supports miosix as a target.

1. Check [rust] for building requirements

2. Clone the source with `git`

    ```sh
    git clone https://github.com/Tazdevil971/rust.git
    cd rust
    ```

3. Copy the build configuration 

    ```sh
    cp config.toml.example config.toml
    ```

4. Build

    ```sh
    ./x.py build library/std
    ```

5. Load the new toolchain

    ```sh
    rustup toolchain link miosix build/<target>/stage1
    ```
    Where `<target>` is you **host** machine target

6. Select the new toolchain as the default one

    ```sh
    rustup default miosix
    ```

7. Go and experiment with the new compiler!

## Structure
Here is a quick overview of the project components:
- [rust]: Contains a patched version of the rust compiler with support for miosix targets
- [libc]: Fork of libc containing definitions for miosix libc types
- [cc-rs]: Fork of cc-rs containing support for miosix targets and miosix toolchain

[rust]: https://github.com/Tazdevil971/rust#building-on-a-unix-like-system
[libc]: https://github.com/Tazdevil971/libc.git
[cc-rs]: https://github.com/Tazdevil971/cc-rs.git