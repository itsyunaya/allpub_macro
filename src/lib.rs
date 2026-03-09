//! A procedural macro to make all items in a module public by default.
//!
//! Rust's visibility is private by default, which is what you want for the vast majority of cases.
//! However, in some specific cases - like a large constants file - it may be desirable to have
//! everything be public without having to annotate everything with the `pub` keyword manually.
//! This crate provides the `[all_pub]` attribute macro for that purpose.
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! allpub_macro = "0.1.0"
//! ```
//!
//! Then apply the attribute to any inline module:
//!
//! ```rust
//! use allpub_macro::all_pub;
//!
//! #[all_pub]
//! mod utils {
//!     fn helper() {}
//!     struct Config { port: u16 }
//! }
//! ```
//!
//! # Limitations
//!
//! This macro cannot affect file modules (`mod foo;`), since their contents
//! are not available to the macro at compile time.

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Item, ItemMod, Visibility, token::Pub, ImplItem};

/// Makes all items in a module public by default
///
/// Applying `#[all_pub]` to a module is equivalent to writing `pub` on every item inside it.
/// Visibility is applied recursively for nested modules, struct fields, and `impl` blocks.
///
/// # Examples
///
/// Basic usage
///
/// ```rust
/// #[all_pub]
/// mod myModule {
///     fn foo() {}         // becomes pub fn foo()
///     struct Bar;         // becomes pub struct Bar
///     const VAL: i32 = 5; // becomes pub const VAL: i32 = 5;
/// }
/// ```
///
/// Nested modules
///
/// ```rust
/// #[all_pub]
/// mod myModA {
///     mod myModB {
///         fn nested() {} // becomes pub fn nested()
///     }
/// }
/// ```
///
/// # Caveats
///
/// - Overusing this macro can lead to a lot of namespace pollution, utilize it sparingly
/// - File modules (`mod foo;` with no braces) are not supported since their contents are located
///   in a separate file outside the macro's reach.
///
/// # Panics
/// This macro will emit a compile error if applied to anything other than a module with a body.
#[proc_macro_attribute]
pub fn all_pub(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);

    if let Some((_, items)) = &mut input.content {
        for item in items.iter_mut() {
            set_pub(item);
        }
    }

    quote!(#input).into()
}

/// Recursively sets the visibility of an item and any items nested within it
/// to `pub`.
///
/// Only item types that carry a visibility modifier are affected. Items that
/// don't support visibility (e.g. `extern crate`) are silently
/// skipped via the catch-all `_ => {}` arm.
fn set_pub(item: &mut Item) {
    let vis = Visibility::Public(Pub::default());
    match item {
        Item::Fn(i)     => i.vis = vis,
        Item::Enum(i)   => i.vis = vis,
        Item::Const(i)  => i.vis = vis,
        Item::Static(i) => i.vis = vis,
        Item::Type(i)   => i.vis = vis,
        Item::Trait(i)  => i.vis = vis,
        Item::Use(i)    => i.vis = vis,
        Item::Struct(i) => {
            i.vis = vis.clone();
            for field in i.fields.iter_mut() {
                field.vis = vis.clone();
            }
        },
        Item::Mod(i)    => {
            i.vis = vis.clone();
            if let Some((_, items)) = &mut i.content {
                for item in items.iter_mut() {
                    set_pub(item);
                }
            }
        },
        Item::Impl(i) => {
            for impl_item in i.items.iter_mut() {
                if let ImplItem::Fn(m) = impl_item {
                    m.vis = vis.clone();
                }
            }
        }
        _ => {}
    }
}
