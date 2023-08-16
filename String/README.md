# Rust_programming
This directory is intented to have examples on string slice and String.

General description of string slice
*)str is an immutable sequence of UTF-8 bytes of dynamic lenght somewhere in memory
  since size is unknown, one can only handle it by a pointer.
*)It is commandly refered by &str in rust.
*)Its size is known at compile time.


General description of String.
*)String is dynamic and mutable.
*)It is commanly referenced by String in rust.
*)Its size is unknown at compile time.
*)The type itself is a struct of the form:
  pub struct String {
    vec: Vec<u8>,
  }
*)Since it contains a Vec, we know that it has a pointer to a chunk of memory, a size,    and a capacity.
*)The size gives us the length of the string and the capacity tells us how much long it can get before we need to reallocate.
*)The pointer points to a contiguous char array on the heap of capacity length and size entries in it.
