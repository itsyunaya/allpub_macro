# allpub_macro

A procedural macro to make all items in a module public by default.

Rust's visibility is private by default, which is what you want for the vast majority of cases.
However, in some specific cases - like a large constants file - it may be desirable to have
everything be public without having to annotate everything with the `pub` keyword manually.
This crate provides the `[all_pub]` attribute macro for that purpose.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
allpub_macro = "0.1.0"
```

Then apply the attribute to any inline module:

```rust
use allpub_macro::all_pub;

#[all_pub]
mod utils {
    fn helper() {}
    struct Config { port: u16 }
}
```

## Limitations

This macro cannot affect file modules (`mod foo;`), since their contents
are not available to the macro at compile time.
