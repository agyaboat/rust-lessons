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