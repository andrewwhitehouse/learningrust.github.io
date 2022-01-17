**Control Flow - Part 1**

We've seen if - else in previous examples. 

Here is a slightly more involved example based on the game rock-paper-scissors.

```
use crate::Winner::{PLAYER1, PLAYER2};

#[derive(PartialEq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS
}

#[derive(PartialEq)]
enum Winner {
    PLAYER1,
    PLAYER2
}

fn rock_paper_scissors(player1_choice: Choice, player2_choice: Choice) -> Option<Winner> {
    if player1_choice == Choice::ROCK {
        if player2_choice == Choice::ROCK {
            None
        } else if player2_choice == Choice::SCISSORS { // Rock blunts scissors
            Some(PLAYER1)
        } else { // Paper wraps rock
            Some(PLAYER2)
        }
    } else if player1_choice == Choice::PAPER {
        if player2_choice == Choice::ROCK { // Paper wraps rock
            Some(PLAYER1)
        } else if player2_choice == Choice::SCISSORS { // Scissors cut paper
            Some(PLAYER2)
        } else { // Paper wraps rock
            None
        }
    } else {
        // TODO
    }
}

fn main() {
    let result = rock_paper_scissors(Choice::ROCK,
                                     Choice::SCISSORS);
    if result.is_some() {
        let winner = if result.unwrap() == PLAYER1 { "player1" } else { "player2" };
        println!("rock/scissors {}", winner );
    } else {
        println!("It's a draw!");
    }
}
```

Using `Enum` for the function parameters and result constrains the set of possible values we have to deal with and so reduces the amount of validation we need to do. If we had used string parameters, for example, we would need to verify that the values were recognised before attempting to apply any logic to them.

You can see that condition that follows `if` doesn't need to be enclosed in brackets. [_TODO: check the rules values._]

Using `match` we can simplify the result printing logic a little:

```
fn main() {
    let result = rock_paper_scissors(Choice::ROCK,
                                     Choice::SCISSORS);
    match result {
      Some(PLAYER1) => println!("Player 1 wins"),
      Some(PLAYER2) => println!("Player 2 wins"),
      _ => println!("It's a draw")
    }
}
```

Exercise:

1. Implement the section marked TODO, and modify the main to call your code, when player 1 chooses "scissors". What are the game results when player 2 chooses "rock", "paper" and "scissors" respectively.

[details="Answers"]
player 2 wins (rock blunts scissors); player 1 wins; draw 
[/details]


