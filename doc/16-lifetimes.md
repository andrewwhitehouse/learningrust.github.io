**Lifetimes**

Consider this code (with added numbers):

```
  1 fn main() {
  2     let r;           
  3     {                
  4         let i = 1;
  5         r = &i;
  6     }
  7     println!("{}", r);
  8 }
```

The compiler raises an error:

```
error[E0597]: `i` does not live long enough
 --> main.rs:5:13
  |
5 |         r = &i;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `i` dropped here while still borrowed
7 |     println!("{}", r);
  |                    - borrow later used here
```

What's going on?

The brace on line 3 introduces a nested scope, in which the variable i is declared. The previously declared reference (defined in the outer scope) then borrows that value.

The closing brace ends the nested scope, and so `i` is no longer defined outside of the nested scope.

In this short example, the error can be avoided by moving the reference access inside the nested scope, where `i` is still defined.

```
fn main() {
    let r;
    {
        let i = 1;
        r = &i;
        println!("{}", r);
    }
}
```

I am sure there is more to this. I'm reading the Rust docs and they aren't really making sense to me at the moment. So I have more work to do.

 
  