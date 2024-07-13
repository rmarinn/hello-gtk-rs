# Hello gtk-rs

*Instructions to build a simple native app written in rust using [gtk-rs](https://gtk-rs.org/).*

This project was completed using the [introductory book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)

Note that this project was built on a **Windows development environment**.

## Prerequisites

1. install [rustup](https://rustup.rs/)
2. tell rust to use MSVC using `rustup default stable-msvc`
3. install [gtk4](https://github.com/wingtk/gvsbuild#development-environment)
4. update environment variables
    * Ensure there is a user variable named `PKG_CONFIG_PATH` with that points to `..\gtk\lib\pkgconfig` (where you placed the installation from step 3.)
    * Edit the variable named `Path` and add `..\gtk\bin` to it
    * Ensure there is a user variable named `Lib` with value `gtk\lib` (or add new value to existing variable if already exists)

## Adding gtk to the project

first find out what the gtk4 version on your machine is by running
```
pkg-config --modversion gtk4
```
Use this information to add the gtk4 crate to your dependencies in Cargo.toml. For example, if the version returned by the command above is `4.14.4`:
```
cargo add gtk4 --rename gtk --features v4_14
```

## The project

* all the code can be found in `src/main.rs`
* run the app using `cargo run`
