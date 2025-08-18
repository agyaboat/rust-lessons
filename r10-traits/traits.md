# Traits: Defining Shared Behavior

- A trait is a way to define shared behaviour (like an interface or contract in other programming languages)
- Trait says `"if you implement this trait, you must provide the following functionality"`
- Better still, Trait says
    - Any type that implements these methods can be treated as having  this ability
- Traits can be implemented by any type, including structs, enums, and even other traits
- Traits are abstracts:
    - They don't hold any data
    - They just define behaviour signatures

- Traits simply define `what a type can do`, and not `what it is` or `how it does it`

## Defining a Trait
- To define a trait, use the `trait` keyword followed by the trait name and a block containing the method signatures.
    ```rust
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    ```
- Realise that after the function signature in the trait, there is no body implementation
- Thus, the trait only specifies the method's name and its parameters, but not how the method is implemented.
- The body implementation is left to the types that implement the trait.
- It is assumed that each type will have different implementations of the trait's methods, but:
  - will use the same method signature: parameters and return type

- A trait can have multiple methods.
- The method signatures are listed per line, and each line ends in a semicolon `;`.

## Implementing a Trait
- To implement a trait for a specific type, use the `impl` keyword followed by the trait name and the type name.
    ```rust
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{} by {} ({})", self.title, self.author, self.location)
        }
    }
    ```
- This block provides the body implementation for the `summarize` method for the `NewsArticle` struct.

## Why use Traits?
- The summarize method could be implemented directly on the NewsArticle struct without the need for a trait.
- However, using a trait allows for greater flexibility and code reuse.
- By defining a trait, we can implement the same behavior for multiple types without duplicating code.
- This promotes a more modular and maintainable codebase.

