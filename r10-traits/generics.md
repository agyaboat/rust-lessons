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
     - as `fn smallest<T>(list: &[T]) -> T`


