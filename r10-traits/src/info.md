# Modules and Crates usage here

## The `aggregator` crate
- There is a library crate called `aggregator` which contains structs and traits.
  - It is defined in the `aggregator` directory.
  - which was created by running `cargo new --lib aggregator` in this same project
- The main crate can use this library by adding it as a dependency in `Cargo.toml`.
```toml
[dependencies]
aggregator = { path = "../aggregator" }
```

- After it is added as a dependency, the main crate can use the structs and traits defined in the library crate by importing them.
    ```rust
    use aggregator::{NewsArticle, Summary};
    ```
   - Realise that before the `summary` method could be called on the `NewsArticle` instance,
   the `Summary` trait has to also be brought into scope.
   - This is because the `summary` method is defined in the `Summary` trait, and the `NewsArticle` struct implements this trait.
   - Failure to bring the `Summary` trait into scope will result in a compilation error when trying to call the `summary` method.

## The `utils` module
- There is a module called `utils` which contains utility functions.
- It is defined in the `utils.rs` file.
- This module was used in the `generics.rs` file to demonstrate the use of generics.
- It shipped reusable utility functions for working with generics.
- It was imported in the file as follows:
    ```rust
        use crate::utils;
    ```
- But before you can use this module, it needs to be resolved as a module in the main file.
- Thus, the above code will throw an error as unfound module if this is not done.

- To resolve this, you need to declare the `utils` module in the main file.
- This can be done by adding the following line to the main file:
    ```rust
    mod utils;
    ```

## Use of `aggregator` in `traits.rs`
- In order to make the code modular and organised, 
a module called `traits.rs` is created to work on the `Traits` examples.
- In order to make it part of the project and be included when running the entry file (`main.rs`),
the `traits` module needs to be declared in the `main` file.
    ```rust
    mod traits;
    ```

- also, the traits module makes use of the `aggregator` crate which has been added as a dependency.
- Thus, it was imported in the `traits.rs` as:
    ```rust
    use crate::aggregator;
    ```
- This allows the `traits.rs` file to use the types and traits defined in the `aggregator` crate.