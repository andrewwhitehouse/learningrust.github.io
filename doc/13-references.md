**References**

We have seen how some simpler data types whose size is known at compile time are allocated on the stack.

However other types, like String, are allocated on the heap and so their memory usage needs to be managed. Rust achieves this through the principle of _ownership_ which brings a set of rules that are checked at compile time (meaning there is no extra work to do at runtime, unlike languages that use garbage collection). 

Rust programmes release the heap associated with a variable when the variable's owner goes out of scope.

Losing access to a variable when it is assigned, or passed to a function, is a significant constraint. Fortunately Rust gives us _references_ which allow us to access variables without the overhead of ownership. 

References are not freed when they go out of scope. So every heap-allocated variable in your programme will have one owner (which determines when the memory can be freed) and references which access the variable, but without ownership.




Copyright (c) 2022 Andrew Whitehouse. All rights reserved.
