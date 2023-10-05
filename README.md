# USE HTTPS

This is a toy library with which you can import `Rust` files over `HTTPS`.

Why? Because why **NOT**!

---

### Inspiration

The library is inpired by [@TsodingDaily](https://www.youtube.com/@TsodingDaily)'s [YouTube video](https://youtu.be/4vSyqK3SK-0?si=QgkUc9YR3lFR8qy0)

It's a great watch.

And also much more complicated than what I did for this implementation.

---

### How to use

Add the library to your `Cargo.toml`.

```toml
use-https = { version = "0.1.0", git = "https://github.com/Maki325/use-https.git", tag = "0.1.0" }
```

There are two ways to use the library, and you can of course mix and match:
 1. The code is wrapped into a module (and optionaly `use`)
 2. The code is not wrapped into a module (mostly meant for importing the code into a file, and then using the file as the module)

#### **1. Way**

```rust
use_https::use_https! {
  import cool_lib_module from "https://raw.githubusercontent.com/Maki325/use-https/master/example/example/cool_lib.rs";
}

fn main() {
  cool_lib_module::say_hello();
}
```

The code from the link is wrapped into the `cool_lib_module` module.

We can `use` exports from it with:

```rust
use_https::use_https! {
  import cool_lib_module::{say_hello} from "https://raw.githubusercontent.com/Maki325/use-https/master/example/example/cool_lib.rs";
}
```

So it would end up like this:

```rust
use_https::use_https! {
  import cool_lib_module::{say_hello} from "https://raw.githubusercontent.com/Maki325/use-https/master/example/example/cool_lib.rs";
}

fn main() {
  say_hello();
}
```

Of course, you can still use all the other exports from the `cool_lib_module` by yourself.

#### **2. Way**

In a file named `test.rs`
```rust
use_https::use_https! {
  import "https://raw.githubusercontent.com/Maki325/use-https/master/example/example/cool_lib.rs";
}
```

And then in the main file, we can use it like a normal module:

```rust
mod test;

fn main() {
  test::say_hello();
}
```
