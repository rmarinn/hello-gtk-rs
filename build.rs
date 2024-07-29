fn main() {
    glib_build_tools::compile_resources(
        &["gtk-templates/resources"],                      // source directories
        "gtk-templates/resources/resources.gresource.xml", // gresource
        "hello_gtk_rs_templates.gresource",                // target
    );
}
