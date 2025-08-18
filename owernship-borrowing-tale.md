# The Parable of the Monster Rust

In a quiet land lived a **monster named Rust**.

From time to time, maidens were brought to his lair. If one entered **directly**, she was **consumed** and never returned — this was the **law of Ownership** (a move).

But the wise discovered rituals to survive:

- The **Ritual of Reference** (`&`): a maiden may visit and **leave unharmed**.  
- The **Dance of Cloning** (`clone()`): a maiden splits in two; one may be consumed while the other lives on, though it **costs energy**.

Over the years, the village wrote down the strict **Rules of the Rituals**:

1. **Many may watch, none may change**  
   - Multiple maidens may perform the Ritual of Reference **at the same time** (multiple immutable borrows).  
   - While watchers are present, **no one may alter anything** (no mutation).

2. **Only one may change, and only alone**  
   - The **Exclusive Rite** (`&mut`) allows exactly **one** maiden to visit and **make changes** (one mutable borrow).  
   - While this rite is active, **no other visits** are allowed — neither watchers nor changers (no other borrows).

3. **No mixing rites**  
   - If watchers are inside (immutable borrows), **no Exclusive Rite** may begin.  
   - If the Exclusive Rite has begun (mutable borrow), **no watchers** may enter.

4. **No lingering threads**  
   - If a maiden turns to dust (the value is dropped), **all threads tied to her must already be cut** (no dangling references).

5. **Cloning costs**  
   - The Dance of Cloning preserves a copy, but **energy is spent** (allocation/time).

**Moral:** In the monster’s realm, safety is earned by **choosing one path at a time**—many may observe **or** one may change; mixing paths invites chaos.


## Consider writing the rust tale
