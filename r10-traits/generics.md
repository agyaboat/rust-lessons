# What are Generics?
Generics allow us to replace specific types with a placeholder that represent multiple types to 
remove code deduplication.

Duplicating code is tedious and error prone

We use generics to create definitions for functions or structs, which we can use with many different concrete data types

## In function definitions
- Place the generics in the signature of the function, specifying the data types of the parameters
and the return value
- This:
  1. gives flexibility
  2. Removes code duplication
  3. provides more functionality

- To parameterize types in a new function, 
  1. First name the type parameter, using any identifier, but it should be short, often just one letter recommended
  2. Before using the type parameter in the function signature, you have to declare the type parameter name:
     - Place it in a square bracket `<>`
     - between the function name and the parameter list
     - as `fn smallest<T>(list: &[T]) -> &T`
     - This is read as: the function `smallest` is generic over some type `T`. The function has one parameter
     named `list`, which is a slice of values of type `T`. The smallest function will return a 
     reference to a value of the same type `T`.

### Example
```rust
fn smallest<T>(list: &[T]) -> &T {
    let mut smallest = &list[0];
    for item in list {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}
```

## Struct Definition
- Similar way as the function, using square brackets

### Example 1.1
```rust
struct Coordinates<T>{
    x: T, 
    y: T,
}

fn main(){
    let cord1 = Coordinates{x: 6, y: 2};
}
```

To use multiple types or varying types for each parameter, it is appropropriate to declare that aforehand

### Example 1.2
```rust
struct Coordinates<T, U>{
    x: T, 
    y: U
}

fn main(){
    let cord2 = Coordinates{x: 3, y: 6.8};
}
```

Just above, in the example used:
- `x` is an `i32`
- `y` is `f64`
- If this was not anticipated and room provided for it, an error will be thrown at compile time
for using the same type `T` to represent the varying types `int` and `float`.
- Thus, the first definition of the struct in `Example 1.1` above cannot be used for the `cord2` definition
in `Example 1.2` here.

## Enum definitions

### Example 2.1
```rust
enum Option<T> {
    Some(T),
    None,
}
```
- The above is read as `Option` enum is a generic over type `T` and has two variants:
  - `Some(T)` which holds a value of type `T`
  - `None` which represents the absence of a value

Enums can also have multiple type parameters, similar to structs.
An example is the `Result` enum in the standard library, which is defined as:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
} 
```
- Read as: 
  - `Result` enum is generic over two types, `T` and `E`
  - and has two variants:
    - `Ok(T)` which holds a value of type `T` representing a successful operation
    - `Err(E)` which holds a value of type `E` representing an error
  - This makes it possible to use the Result enum anywhere an operation might:
   - Succeed and return a value of type `T`
   - or Fail and return an error of type `E`


## Method Definitions
- When you implement methods for a generic struct, you must also declare the generic(s) after impl.
    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }
    ```

- This tells Rust: "I'm implementing methods for Point that can hold any type T."
- Works for any Point<T> regardless of whether T is i32, f64, char, etc.

- *Convention*:
  - You can choose a different generic name in impl (impl<U> Point<U>), but using the same one (T) is conventional → clearer to read.

- *Flexibility*:
  - With impl<T>, methods apply to all types of Point<T>.
  - With impl Point<f32>, methods apply only to Point<f32>.