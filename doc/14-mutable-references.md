**Mutable References**

The previous example dealt with immutable references.

We would also like to be able to change the data for which we have "borrowed" a reference.

Here is a maze:

```
*-*-*-*-*-*
|o        |
*-* *-*-*-*
| |       |
* * *-*-* *
| | |   | |
* * * *-*-*
|     |   |
* *-* * *-*
| |       |
*-*-*-*-*-*
```

We can traverse this maze using a [wall follower](https://en.wikipedia.org/wiki/Maze-solving_algorithm#Wall_follower) alorithm.

If our current direction of travel is south, we try to go west first; if that direction is blocked then we try south, then east, then north. 

If we're travelling east, we try south, then east, then north, then west. 

Following this approach we end up with this path through the maze:

*east-south-south-south-west-...*

which we can abbreviate with the initial letter:

*essswnnsssneesweee*

There are a few instances of back-tracking in this route:

For example, after:

*essswnn* 

we reach a dead and and then have to backtrack

*sss*

and then continue:

*ne*

Let's simplify this route to give the most direct path through the maze.

We can do this by modifying the original string.

```
fn simplify(s: &mut String) {
    let mut changed;
    loop {
       changed = false;
       let backtrack = ["ns", "sn", "we"];
       for removal in backtrack {
           if s.contains(removal) {
               changed = true;
               *s = s.replace(removal, "");
           }
       }
       if !changed {
           break;
       }
    }
}

fn main() {
    let mut route = String::from("essswnnsssneesweee");
    simplify(&mut route);
    println!("{}", route);
}
```

The algorithm continually removes directions that cancel each other out, until there are no more remaining.

## Discussion

Mutable code modifies the data in-place. We could also implement this algorithm immutably, passing an immutable reference to `simplify()` and then returning the simplified value.

What is better? A lawyer or software engineer might say: it depends.

When writing your code with mutable data it may lead to reducing the memory footprint of your code. When you are manipulating large amounts of input this can be beneficial.

However code that mutates values in place can be harder to follow so it's a good idea to localise the changes. 

```
    let route = String::from("essswnnsssneesweee");
    let simplified = simplify(&route);
```

Is this easier to read? Maybe. When I'm in doubt, I optimise for readability first.

The other observation I'd like to make is that this code works. But there may be a more elegant way to do it.

This is in part because I'm limiting the number of new concepts I introduce in each chapter (while wanting to keep the exapmles interesting). 

But it's working code. Start with code that solves the problem you have, and meets the requirements you've been given. And then make it better.

**Exercise**:

What is the simplified route through the maze? 

[details="Answer"]

esssesee

[/details]




