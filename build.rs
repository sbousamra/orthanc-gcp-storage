extern crate cc;

fn main() {
  cc::Build::new()
    .cpp(false) 
    .file("external/wrapper.c")
    .compile("orthancplugin")
}