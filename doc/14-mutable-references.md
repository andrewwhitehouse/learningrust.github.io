**Mutable References**

The previous example dealt with immutable references.

We would also like to be able to change the data for which we have "borrowed" a reference.

Here is a maze:

```
*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
|o|         |                     |
* *-*-* * * *-*-* *-*-*-*-*-*-* *-*
|       | |       |             | |
*-*-*-* *-*-*-*-*-*-*-*-*-*-*-* * *
|       |     |   | |   |       | |
*-*-*-*-* *-* *-* * *-* *-*-*-* * *
|   |   | |   | |             |   |
*-* *-* *-*-* * * *-*-* *-*-* * * *
|     |   |       |     |       | |
*-*-* *-* *-*-*-* *-* * *-* *-*-* *
| | |     |       |   | |   |   | |
* * *-*-* *-*-* *-* * *-* *-*-* * *
| |             |   | |   |       |
* * * *-* *-*-* * *-* *-*-*-*-* * *
|   | |   | | | | |   |         | |
* * * * *-* * * *-* *-*-* * * * * *
| | | | |       |   |     | | | | |
* * * *-*-*-*-*-*-*-*-* *-* *-* *-*
| | |   |                 |   |   |
*-*-* * *-*-*-* * *-* * *-* *-* * *
|     | |       | |   | |     | | |
*-* * * * * * *-* * *-*-* * * * * *
|   | | | | |   | |   |   | | | | |
* *-* * *-*-*-* *-* *-* * * * * * *
|   | | |         | |   | | | | | |
* * * * * * *-* * *-*-*-*-* * * * *
| | | | | | |   | | |       | | | |
* * *-*-*-* * *-*-* *-* * *-* * *-*
| |   |     | |         | |   |   |
* * *-*-*-* *-*-*-*-*-* *-* * *-* *
| | | | | | |           |   |   | |
* * * * * *-*-* *-* *-* * *-* *-* *
| | |           |   |   |   |   | |
*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
```

We can traverse this maze using a [wall follower](https://en.wikipedia.org/wiki/Maze-solving_algorithm#Wall_follower) alorithm.

If our current direction of travel is south, we try to go west first; if that direction is blocked then we try south, then east, then north. 

If we're travelling east, we try south, then east, then north, then west. 

Following this approach we end up with this path through the maze:

*seeeswwweeennesneseeeneeeeeeeswwwwwweeeeeeswwweeesswnwwwnweswwnswnwesswnswnnwwsneeswesweeeswwweeswwwnnwnweseswwnwnwesweeseeswwwnsswnnssssnnessnnnesssswweswsssssnnnnessssnnewnnwnenesssnnnnessssnnnnwnnneeswsneneeesswnswnsweeennnenneeeswweswswsnenesswswenennnesnneeeswweswsweneneenessswnweswweeswwwweswweswwwwwwweeeswwwsnesneseswwwsnesswweesnnneeswsnenesnwnwnenessnneeswssnewnenesnewnenessswswswenesnnessswwweswwnswweeeeswwwwweswwwnswnswnseeeeeneesweneeswennnesnennnesssswswsewnenessewnewnnnnnwnewnnesnesssssssess*

Not only is this rather hard to read, but because it is trying all possible routes it involves a fair amount of back-tracking.

For example, after this route:

*seeeswww* 

we reach a dead and and then have to backtrack

*eeenn*

and so on.

Let's simplify this route to give the most direct path through the maze.

We can do this by modifying the original string.



