use_https::use_https! {
  // Will add a submodule for the contents
  import test_module from "http://localhost:8000/test.rs";
  // Eg.
  // mod test_module {
  // ...
  // }
}

// This is a file with an imported module
mod test;

fn main() {
  test::example();

  test_module::example();
}
