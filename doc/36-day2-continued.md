**Day 2 Part 1**

Day 2 continued, following the [earlier setup](./doc/35-day2.md).

Update the parsing for the commands

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





