use_https::use_https! {
  // Will add a submodule for the contents
  import cool_lib_module from "https://raw.githubusercontent.com/Maki325/use-https/master/example/example/cool_lib.rs";
  // Eg.
  // mod cool_lib_module {
  // ...
  // }
}

// This is a file with an imported module
mod test;

fn main() {
  test::say_hello();

  cool_lib_module::say_hello();
}
