# Hello gtk-rs Template Setup

*Instructions to build a simple native app written in rust using [gtk-rs](https://gtk-rs.org/).*


This project was completed using the [introductory book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)

This branch specifically, is a demo on how to use [composite templates](https://gtk-rs.org/gtk4-rs/stable/latest/book/composite_templates.html)

For a basic setup without using templates, see the [main branch](https://github.com/rmarinn/hello-gtk-rs)

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

## Templating Prerequisites

1. add `glib-build-tools` as a dependency
    ```cargo add glib-build-tools --build```
2. create build script at the root of the package (see `build.rs`)

## Templating Basics

when using templates, you must first specify a gresource file
```xml
<!-- gtk-templates/resources/resources.gresource.xml -->
<?xml version="1.0" encoding="UTF-8"?>
<gresources>
  <gresource prefix="/org/gtk_rs/hello_gtk_rs/">
    <file compressed="true" preprocess="xml-stripblanks">window.ui</file>
  </gresource>
</gresources>
```

then tell `build.rs` where it's located and the target is
```rust
// build.rs
fn main() {
    glib_build_tools::compile_resources(
        &["gtk-templates/resources"], // source directories 
        "gtk-templates/resources/resources.gresource.xml", // gresource
        "hello_gtk_rs_templates.gresource", // target
    );
}
```

register and include the resources by calling the macro `gio::resources_register_include!`, pointing to the target specified at `build.rs`
```rust
// src/main.rs
const APP_ID: &str = "org.gtk_rs.CompositeTemplates1";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("hello_gtk_rs_templates.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // --- snip ---
}
```

when using new templates, the path the the resource must point to the prefix defined in `gtk-templates/resources/resources.gresource.xml`
```rust
// src/window/imp.rs
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/hello_gtk_rs/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<Button>,
}
```
