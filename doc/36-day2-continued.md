**Day 2 continued**, following the [earlier setup](./doc/35-day2.md).

Now that we have the pieces in place, let's add tests and an implementation for the command parsing.

```
mod tests {
    use super::*;
    use crate::Direction::{Up,Down,Forward};

    #[test]
    fn test_parse_multiple_commands() {
        let expected = vec!(Command(Forward, 3), Command(Up, 5), Command(Down, 10));
        assert_eq!(parse(String::from("forward 3\nup 5\ndown 10")), expected);
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("forward 5"), Command(Forward,5));
    }
}
```

```
fn parse_command(s: &str) -> Command {
    let parts: Vec<&str> = s.split(" ").collect();
    let distance: u32 = parts[1].parse().expect("Not a number!");
    match parts[0] {
        "forward" => Command(Direction::Forward, distance),
        "up" => Command(Direction::Up, distance),
        "down" => Command(Direction::Down, distance),
        _ => panic!("Unrecognised command {}", s)
    }
}

pub fn parse(input: String) -> Vec<Command> {
    input.lines().map(|line| parse_command(line)).collect()
}
```

The purpose of this parsing code is to take the external input, which usually starts off as strings, and convert it into valid data types that are a better match for our domain logic. 

In this problem we have a limited set of commands, each of which has an associated distance. We apply rules to these commands to end up with a result.

Now let's add the navigation domain logic using the `Command` types we've parsed the input into.

```
#[derive(PartialEq, Debug)]
pub struct Location { horizontal_position: u32, depth: u32 }
```

Tests

```
#[test]
fn test_navigate_no_commands() {
    assert_eq!(navigate(Vec::new()), Location { horizontal_position: 0, depth: 0 });
}

#[test]
fn test_navigate() {
    let commands = vec!(Command(Direction::Forward, 5),
        Command(Direction::Down, 5),
        Command(Direction::Forward, 8),
        Command(Direction::Up, 3),
        Command(Direction::Down, 8),
        Command(Direction::Forward, 2));
    assert_eq!(navigate(commands), Location { horizontal_position: 15, depth: 10 });
}
```

Implementation:    

```
fn navigate(commands: Vec<Command>) -> Location {
    let mut horizontal_position: u32 = 0;
    let mut depth: u32 = 0;
    for command in commands {
        if command.0 == Direction::Forward {
            horizontal_position += command.1;
        } else if command.0 == Direction::Up {
            depth -= command.1;
        } else {
            depth += command.1;
        }
    }
    Location { horizontal_position, depth }
}
```

Make some minor changes so that we can run both sets of tests.

`Cargo.toml`

```
[[bin]]
name = "day1"
path = "src/day1.rs"
```

`day1.rs`

Rename tests module:

```
mod day1_tests {
```

`day2.rs`

```
mod day2_tests {
```

Run the tests.

`cargo test`

```
 Running unittests (target/debug/deps/day1-f06424c4e9a7009b)

running 3 tests
test day1_tests::test_count_increases ... ok
test day1_tests::test_parse ... ok
test day1_tests::test_sliding_window_sum ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/day2-55fed2c67c13c633)

running 4 tests
test day2_tests::test_navigate ... ok
test day2_tests::test_navigate_no_commands ... ok
test day2_tests::test_parse_command ... ok
test day2_tests::test_parse_multiple_commands ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Add final implementation:

```
pub fn part1(input: String) -> u32 {
    let location = navigate(parse(input));
    location.horizontal_position * location.depth
}
```

and acceptance test (which takes string input and returns the desired result given in the problem statement).

```
#[test]
fn test_part1() {
    let commands = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
    assert_eq!(part1(commands), 150);
}
```    

Run the test

`cargo test day2_tests::test_part1`

Update the day2_tests ... I haven't figured out yet why I need to do this.

```
use Direction::{Up,Down,Forward};
```

but it's after I added a module statement to the main file.

Now add the main implementation in main.rs:

Add to top of the file:

```
mod day2;
```

Implementation function:

```
fn solve_day2() {
    let input = fs::read_to_string("input/day2.txt")
        .expect("Something went wrong reading the file");
    println!("Day 2 Part 1 {}", day2::part1(input));
}
```

and call it from the main function:

```
pub fn main() {
    solve_day1();
    solve_day2();
}
```

It took me a little while to figure out the binary and library settings so that `cargo test` runs the tests and `cargo run` runs the main in main.rs:

In the [package] section of `Cargo.toml`:

```
default-run  ="main"
```
 
and below:

```
[[bin]]
name = "day2"
path = "src/day2.rs"

[[binc]]
name = "day1"
path = "src/day1.rs"

[[bin]]
name = "main"
path = "src/main.rs"
```

`git add src input Cargo.toml`

`git commit -m "Day 2 Part 1 done"`

```
$ cargo run
. . .
Day 1 Part 1 1342 Part 2 1378
Day 2 Part 1 1636725
$ 
```

## Part 2

This changes the rules.

> In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:
>
- down X increases your aim by X units.
- up X decreases your aim by X units.
- forward X does two things:
    - It increases your horizontal position by X units.
    - It increases your depth by your aim multiplied by X.

The command representation doesn't change but the result does.

I called it Location in the first part (composed of horizontal location and depth). What do we call this with an added attribute now?

What is a word for position combined with orientation?

Googling ...

>Position is where you are; orientation is which way you're pointing (including any roll)

Apparently there's a thing called a [Pose](https://digitalrune.github.io/DigitalRune-Documentation/html/d995ee69-0650-4993-babd-1cdb1fd8fd7a.htm), which is not a word I'm familar with but it does seem to be a formal definition.

In Day 1 I was able to adapt the part 1 solution with the additional of a functional transformation, reusing the existing code. It will be more difficult to do this in part 2  because the algorithm is changing.

So let's write a new navigation function.

But before we do, let's refactor the `Command` struct to use named attributes, which will make the code easier to read.

Here is the updated implementation:

```
fn parse_command(s: &str) -> Command {
    let parts: Vec<&str> = s.split(" ").collect();
    let distance: u32 = parts[1].parse().expect("Not a number!");
    match parts[0] {
        "forward" => Command { direction: Direction::Forward, distance },
        "up" => Command { direction:Direction::Up, distance },
        "down" => Command { direction:Direction::Down, distance },
        _ => panic!("Unrecognised command {}", s)
    }
}
```

```
fn navigate(commands: Vec<Command>) -> Location {
    let mut horizontal_position: u32 = 0;
    let mut depth: u32 = 0;
    for command in commands {
        if command.direction == Direction::Forward {
            horizontal_position += command.distance;
        } else if command.direction == Direction::Up {
            depth -= command.distance;
        } else {
            depth += command.distance;
        }
    }
    Location { horizontal_position, depth }
}
```

and one of the updated tests:

```
#[test]
fn test_parse_multiple_commands() {
    let expected = vec!(Command { direction: Forward, distance: 3 },
                        Command { direction: Up, distance: 5 },
                        Command { direction: Down, distance: 10 });
    assert_eq!(parse(String::from("forward 3\nup 5\ndown 10")), expected);
}
```

Having the tests already in place allows us to refactor the code and re-run the tests until they pass again. 

This gives us the confidence that we haven't broken the code. We're relying on picking good test cases and often in a professional setting with a larger code base we would set some quality requirement for the percentage of our code that needs to be exercised when we run the tests. 

I'm not going to add a code coverage tool. We have unit tests for the individual functions, and acceptance tests to verify that the pieces work together, using the test cases given in the problem description. 

And that seems to be enough. If we spot any unexpected behaviour, we diagnose the problem and then add a test case to catch that bug in the future.

Here's my first implementation:

```
#[derive(PartialEq, Debug)]
pub struct Pose { horizontal_position: u32, depth: u32, aim: u32 }
```

```
fn navigate_with_aim(commands: Vec<Command>) -> Pose {
    let mut pose = Pose { horizontal_position: 0, depth: 0, aim: 0 };
    for command in commands {
        if command.direction == Direction::Down {
            pose.aim += command.distance;
        } else if command.direction == Direction::Up {
            pose.aim -= command.distance;
        } else { // forward
            pose.horizontal_position += command.distance;
            pose.depth += pose.aim * command.distance;
        }
    }
    pose
}
```

```
#[test]
fn test_navigate_with_aim() {
    let commands = vec!(Command { direction: Forward, distance: 5},
                        Command { direction: Down, distance: 5 },
                        Command { direction: Forward, distance: 8 },
                        Command { direction: Up, distance: 3 },
                        Command { direction: Down, distance: 8},
                        Command { direction: Forward, distance: 2 });
    assert_eq!(navigate_with_aim(commands),
               Pose { horizontal_position: 15, depth: 60, aim: 10 });
}
```

This test passes.    

So far I've picked unsigned values (which can't be negative); I'm looking at the potential for aim to go negative if we encounter an "up" command. And if the aim becomes negative it's possible that the depth could decrease, but it clearly shouldn't go negative. (There's nothing in the description about out submarine being able to fly!) Let's assume that for our limited scenario it's not possible for the depth to be negative.

So although our simple case passes, it's possible that the more complex test case could make the aim negative, at least temporarily.

We don't know whether this is a valid scenario or not because the problem description (our requirements) are silent.

I'm going to err on the side of caution and change the type of the Post aim.

```
#[derive(PartialEq, Debug)]
pub struct Pose { horizontal_position: u32, depth: u32, aim: i32 }
```

When I do this, the compiler starts to complain about me mixing types:

```
error[E0308]: mismatched types
  --> src/day2.rs:56:25
   |
56 |             pose.aim += command.distance;
   |                         ^^^^^^^^^^^^^^^^ expected `i32`, found `u32`

error[E0277]: cannot add-assign `u32` to `i32`
  --> src/day2.rs:56:22
   |
56 |             pose.aim += command.distance;
   |                      ^^ no implementation for `i32 += u32`
   |
   = help: the trait `AddAssign<u32>` is not implemented for `i32`
```

OK. What to do here? Do I change the type of the depth parameter to make it signed. It logically doesn't make sense to have a negative depth. 

Let's make the simpler changes and see how it looks ...

```
fn navigate_with_aim(commands: Vec<Command>) -> Pose {
    let mut pose = Pose { horizontal_position: 0, depth: 0, aim: 0 };
    for command in commands {
        if command.direction == Direction::Down {
            pose.aim += command.distance as i32;
        } else if command.direction == Direction::Up {
            pose.aim -= command.distance as i32;
        } else { // forward
            pose.horizontal_position += command.distance;
            let depth_change: i32 = pose.aim * (command.distance as i32);
            pose.depth += depth_change as u32;
        }
    }
    pose
}
```

That's not too bad. 

The main potential code smell is applying the depth change (which can be negative) to the unsigned depth. When we convert a signed value to an unsigned value that could lead to a bug if the signed value ever becomes negative (which depends on our input data). 

What I would really like is for our product owner to tell us that the input won't ever make our depth go negative. 

For now, I'm going to assume that it won't. And the result will either be correct, or it won't.

In a long-lived software project I would clarify this with the product owner and write tests to codify that decision so that future maintainers understand what the expected behaviour is.

Here's the full implementation for part 2

`day2.rs`

```
pub fn part2(input: String) -> u32 {
    let pose = navigate_with_aim(parse(input));
    pose.horizontal_position * pose.depth
}
```

```
#[test]
fn test_part2() {
    let commands = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
    assert_eq!(part2(commands), 900);
}
```    

```
running 10 tests
test day2::day2_tests::test_navigate_no_commands ... ok
test day2::day2_tests::test_navigate ... ok
test day1::tests::test_count_increases ... ok
test day1::tests::test_sliding_window_sum ... ok
test day2::day2_tests::test_navigate_with_aim ... ok
test day2::day2_tests::test_parse_command ... ok
test day1::tests::test_parse ... ok
test day2::day2_tests::test_parse_multiple_commands ... ok
test day2::day2_tests::test_part1 ... ok
test day2::day2_tests::test_part2 ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

You can see that we are accumulating a suite of tests. As we develop the code, and add more tests, it gives us the confidence that any changes haven't broken our existing code and allows us to refactor when we see a better design.

Let's update our main to run the updated solution:

```
fn solve_day2() {
    let input = fs::read_to_string("input/day2.txt")
        .expect("Something went wrong reading the file");
    println!("Day 2 Part 1 {} Part 2 {}", day2::part1(input.clone()), day2::part2(input.clone()));
}
```

Rather than dealing with ownership issues on the shared input (by passing the same data structure to two different functions) I did the simplest thing, which is to clone the data.

```
$ cargo run
. . .
Day 1 Part 1 1342 Part 2 1378
Day 2 Part 1 1636725 Part 2 1872757425
$ 
```

(That's the right answer for my input data. If you try this you may have different input data, with a different result.)

```
$ git add src
$ git commit -m "Day 2 part 2 solution"
[master cae6825] Day 2 part 2 solution
 2 files changed, 12 insertions(+), 1 deletion(-)
$ 
```

I noticed that we're still using `master` as the branch name. Let's change that to `main`, which is the [preferred name](https://github.com/github/renaming/) for the trunk branch nowadays.

```
$ git branch -M main
$ git branch
* main
$ 
```

