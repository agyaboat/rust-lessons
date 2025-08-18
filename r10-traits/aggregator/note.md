# Generation

This is a library crate, which was created by running `cargo new --lib aggregator`.

Structs and traits are defined here to be used in other crates.


# Usage in the Main Crate
- The main crate can use this library by adding it as a dependency in `Cargo.toml`.

```toml
[dependencies]
aggregator = { path = "../aggregator" }
```
- The main crate can use the structs and traits defined in the library crate by importing them.
```rust
use aggregator::{NewsArticle, Summary};
```
   - Realise that before the `summary` method could be called on the `NewsArticle` instance,
   the `Summary` trait has to also be brought into scope.
   - This is because the `summary` method is defined in the `Summary` trait, and the `NewsArticle` struct implements this trait.
   - Failure to bring the `Summary` trait into scope will result in a compilation error when trying to call the `summary` method.